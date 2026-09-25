//! 第4章 問題5: 非同期ウェブスクレイパー
//! 
//! この問題では、非同期プログラミングの実践的な応用として、
//! ウェブスクレイピングツールを実装します。並行処理、エラーハンドリング、
//! レート制限、リトライ機能など、実際のアプリケーションで必要な機能を学習します。

use futures::{stream, StreamExt};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{Semaphore, RwLock};
use tokio::time::{sleep, timeout};
use url::Url;

/// スクレイピング結果を表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingResult {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_code: u16,
    pub response_time: Duration,
    pub word_count: usize,
    pub links: Vec<String>,
    pub timestamp: String,
}

/// スクレイピング設定
#[derive(Debug, Clone)]
pub struct ScrapingConfig {
    pub max_concurrent_requests: usize,
    pub request_timeout: Duration,
    pub delay_between_requests: Duration,
    pub max_retries: usize,
    pub retry_delay: Duration,
    pub user_agent: String,
}

impl Default for ScrapingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 10,
            request_timeout: Duration::from_secs(30),
            delay_between_requests: Duration::from_millis(1000),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            user_agent: "Rust-WebScraper/1.0".to_string(),
        }
    }
}

/// レート制限機能付きHTTPクライアント
pub struct RateLimitedClient {
    client: Client,
    semaphore: Arc<Semaphore>,
    config: ScrapingConfig,
    request_history: Arc<RwLock<Vec<Instant>>>,
}

impl RateLimitedClient {
    pub fn new(config: ScrapingConfig) -> Self {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(config.request_timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            semaphore: Arc::new(Semaphore::new(config.max_concurrent_requests)),
            config,
            request_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// レート制限を考慮したHTTPリクエスト
    pub async fn get_with_rate_limit(&self, url: &str) -> Result<Response, reqwest::Error> {
        // セマフォで同時リクエスト数を制限
        let _permit = self.semaphore.acquire().await.unwrap();

        // レート制限のための遅延
        self.enforce_rate_limit().await;

        // リクエスト履歴を記録
        {
            let mut history = self.request_history.write().await;
            history.push(Instant::now());
            
            // 古い履歴をクリーンアップ（過去1分間のみ保持）
            let cutoff = Instant::now() - Duration::from_secs(60);
            history.retain(|&timestamp| timestamp > cutoff);
        }

        self.client.get(url).send().await
    }

    async fn enforce_rate_limit(&self) {
        let history = self.request_history.read().await;
        
        if let Some(&last_request) = history.last() {
            let elapsed = last_request.elapsed();
            if elapsed < self.config.delay_between_requests {
                let sleep_duration = self.config.delay_between_requests - elapsed;
                drop(history); // ロックを早めに解放
                sleep(sleep_duration).await;
            }
        }
    }

    /// リクエスト統計を取得
    pub async fn get_request_stats(&self) -> RequestStats {
        let history = self.request_history.read().await;
        let now = Instant::now();
        
        let last_minute = history.iter()
            .filter(|&&timestamp| now.duration_since(timestamp) < Duration::from_secs(60))
            .count();
        
        let last_hour = history.iter()
            .filter(|&&timestamp| now.duration_since(timestamp) < Duration::from_secs(3600))
            .count();

        RequestStats {
            requests_last_minute: last_minute,
            requests_last_hour: last_hour,
            total_requests: history.len(),
            active_connections: self.config.max_concurrent_requests - self.semaphore.available_permits(),
        }
    }
}

#[derive(Debug)]
pub struct RequestStats {
    pub requests_last_minute: usize,
    pub requests_last_hour: usize,
    pub total_requests: usize,
    pub active_connections: usize,
}

/// リトライ機能付きスクレイパー
pub struct WebScraper {
    client: RateLimitedClient,
    config: ScrapingConfig,
    results: Arc<Mutex<Vec<ScrapingResult>>>,
}

impl WebScraper {
    pub fn new(config: ScrapingConfig) -> Self {
        let client = RateLimitedClient::new(config.clone());
        
        Self {
            client,
            config,
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 単一URLのスクレイピング（リトライ機能付き）
    pub async fn scrape_url(&self, url: &str) -> Result<ScrapingResult, String> {
        let mut last_error = String::new();
        
        for attempt in 1..=self.config.max_retries {
            let start_time = Instant::now();
            
            match timeout(
                self.config.request_timeout,
                self.client.get_with_rate_limit(url)
            ).await {
                Ok(Ok(response)) => {
                    let response_time = start_time.elapsed();
                    
                    match self.parse_response(url.to_string(), response, response_time).await {
                        Ok(result) => {
                            // 成功した結果を保存
                            self.results.lock().unwrap().push(result.clone());
                            return Ok(result);
                        }
                        Err(e) => {
                            last_error = format!("Parse error: {}", e);
                        }
                    }
                }
                Ok(Err(e)) => {
                    last_error = format!("Request error: {}", e);
                }
                Err(_) => {
                    last_error = "Request timeout".to_string();
                }
            }
            
            if attempt < self.config.max_retries {
                println!("  リトライ {}/{} for {}: {}", 
                         attempt, self.config.max_retries, url, last_error);
                
                let delay = self.config.retry_delay * attempt as u32;
                sleep(delay).await;
            }
        }
        
        Err(format!("最大リトライ回数に達しました: {}", last_error))
    }

    async fn parse_response(
        &self,
        url: String,
        response: Response,
        response_time: Duration,
    ) -> Result<ScrapingResult, String> {
        let status_code = response.status().as_u16();
        let text = response.text().await.map_err(|e| e.to_string())?;
        
        // HTMLの基本的な解析（実際のプロジェクトではscraper crateなどを使用）
        let title = extract_title(&text);
        let description = extract_description(&text);
        let word_count = text.split_whitespace().count();
        let links = extract_links(&text, &url);
        
        Ok(ScrapingResult {
            url,
            title,
            description,
            status_code,
            response_time,
            word_count,
            links,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// 複数URLの並行スクレイピング
    pub async fn scrape_urls(&self, urls: Vec<String>) -> Vec<Result<ScrapingResult, String>> {
        let tasks = stream::iter(urls)
            .map(|url| async move {
                println!("  スクレイピング開始: {}", url);
                let result = self.scrape_url(&url).await;
                
                match &result {
                    Ok(data) => {
                        println!("  ✓ 完了: {} ({}ms, {} words)", 
                                 url, data.response_time.as_millis(), data.word_count);
                    }
                    Err(e) => {
                        println!("  ✗ 失敗: {} - {}", url, e);
                    }
                }
                
                result
            })
            .buffer_unordered(self.config.max_concurrent_requests);

        tasks.collect().await
    }

    /// 結果を取得
    pub fn get_results(&self) -> Vec<ScrapingResult> {
        self.results.lock().unwrap().clone()
    }

    /// 統計情報を取得
    pub async fn get_stats(&self) -> ScrapingStats {
        let results = self.get_results();
        let request_stats = self.client.get_request_stats().await;
        
        let total_scraped = results.len();
        let successful = results.iter().filter(|r| r.status_code == 200).count();
        let failed = total_scraped - successful;
        
        let avg_response_time = if !results.is_empty() {
            results.iter()
                .map(|r| r.response_time.as_millis() as f64)
                .sum::<f64>() / results.len() as f64
        } else {
            0.0
        };
        
        let total_words = results.iter().map(|r| r.word_count).sum();
        
        ScrapingStats {
            total_urls_scraped: total_scraped,
            successful_requests: successful,
            failed_requests: failed,
            average_response_time_ms: avg_response_time,
            total_words_extracted: total_words,
            request_stats,
        }
    }
}

#[derive(Debug)]
pub struct ScrapingStats {
    pub total_urls_scraped: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_response_time_ms: f64,
    pub total_words_extracted: usize,
    pub request_stats: RequestStats,
}

/// 簡単なHTML解析関数群
fn extract_title(html: &str) -> Option<String> {
    // 単純な正規表現ベースのタイトル抽出
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html[start..].find("</title>") {
            let title = &html[start + 7..start + end];
            return Some(html_escape::decode_html_entities(title).to_string());
        }
    }
    None
}

fn extract_description(html: &str) -> Option<String> {
    // meta descriptionの抽出
    for line in html.lines() {
        if line.contains("name=\"description\"") || line.contains("name='description'") {
            if let Some(content_start) = line.find("content=\"") {
                let content_start = content_start + 9;
                if let Some(content_end) = line[content_start..].find('"') {
                    let description = &line[content_start..content_start + content_end];
                    return Some(html_escape::decode_html_entities(description).to_string());
                }
            }
        }
    }
    None
}

fn extract_links(html: &str, base_url: &str) -> Vec<String> {
    let mut links = Vec::new();
    let base = match Url::parse(base_url) {
        Ok(url) => url,
        Err(_) => return links,
    };
    
    // href属性を含む行を検索
    for line in html.lines() {
        if line.contains("href=") {
            // 単純なリンク抽出
            let mut start = 0;
            while let Some(href_pos) = line[start..].find("href=\"") {
                let abs_start = start + href_pos + 6;
                if let Some(href_end) = line[abs_start..].find('"') {
                    let link = &line[abs_start..abs_start + href_end];
                    
                    // 相対URLを絶対URLに変換
                    if let Ok(absolute_url) = base.join(link) {
                        let url_str = absolute_url.to_string();
                        if url_str.starts_with("http") && !links.contains(&url_str) {
                            links.push(url_str);
                        }
                    }
                    
                    start = abs_start + href_end;
                } else {
                    break;
                }
            }
        }
    }
    
    links
}

/// バッチスクレイピング処理
pub struct BatchScraper {
    scraper: WebScraper,
}

impl BatchScraper {
    pub fn new(config: ScrapingConfig) -> Self {
        Self {
            scraper: WebScraper::new(config),
        }
    }

    /// サイトマップからURLを生成してスクレイピング
    pub async fn scrape_sitemap(&self, base_urls: Vec<String>) -> Vec<ScrapingResult> {
        println!("バッチスクレイピングを開始...");
        
        let mut all_urls = base_urls.clone();
        let mut discovered_urls = Vec::new();
        
        // 最初のページセットをスクレイピング
        let initial_results = self.scraper.scrape_urls(base_urls).await;
        
        // 成功した結果からリンクを収集
        for result in initial_results {
            if let Ok(page_result) = result {
                discovered_urls.extend(page_result.links.clone());
            }
        }
        
        // 重複を除去し、新しいURLのみを抽出
        discovered_urls.sort();
        discovered_urls.dedup();
        discovered_urls.retain(|url| !all_urls.contains(url));
        
        // 発見されたURLの一部をスクレイピング（最大20個）
        if !discovered_urls.is_empty() {
            let additional_urls: Vec<String> = discovered_urls
                .into_iter()
                .take(20)
                .collect();
            
            println!("発見されたリンクをスクレイピング中... ({} URLs)", additional_urls.len());
            all_urls.extend(additional_urls.clone());
            
            let _ = self.scraper.scrape_urls(additional_urls).await;
        }
        
        self.scraper.get_results()
    }

    /// 結果をJSON形式で保存
    pub async fn save_results_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let results = self.scraper.get_results();
        let json = serde_json::to_string_pretty(&results)?;
        
        tokio::fs::write(filename, json).await?;
        println!("結果を {} に保存しました", filename);
        
        Ok(())
    }

    /// CSVレポートの生成
    pub async fn generate_csv_report(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let results = self.scraper.get_results();
        
        let mut csv_content = String::from("URL,Title,Status,Response Time (ms),Word Count,Links Count\n");
        
        for result in results {
            let title = result.title.as_deref().unwrap_or("N/A").replace(',', ";");
            csv_content.push_str(&format!(
                "{},{},{},{},{},{}\n",
                result.url,
                title,
                result.status_code,
                result.response_time.as_millis(),
                result.word_count,
                result.links.len()
            ));
        }
        
        tokio::fs::write(filename, csv_content).await?;
        println!("CSVレポートを {} に保存しました", filename);
        
        Ok(())
    }
}

/// 実用的な使用例とデモンストレーション
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 非同期ウェブスクレイパーのデモンストレーション ===\n");

    // 設定
    let config = ScrapingConfig {
        max_concurrent_requests: 5,
        request_timeout: Duration::from_secs(10),
        delay_between_requests: Duration::from_millis(500),
        max_retries: 2,
        retry_delay: Duration::from_millis(500),
        user_agent: "Rust-Tutorial-Scraper/1.0".to_string(),
    };

    // テスト用のURL（実際には有効なサイトを使用してください）
    let test_urls = vec![
        "https://httpbin.org/html".to_string(),
        "https://httpbin.org/json".to_string(),
        "https://httpbin.org/xml".to_string(),
        "https://httpbin.org/status/404".to_string(), // エラーテスト用
        "https://httpbin.org/delay/3".to_string(),    // タイムアウトテスト用
    ];

    // 1. 基本的なスクレイピング
    println!("1. 基本的なスクレイピング");
    demo_basic_scraping(&config, test_urls.clone()).await?;
    println!();

    // 2. バッチスクレイピング
    println!("2. バッチスクレイピング");
    demo_batch_scraping(&config, test_urls.clone()).await?;
    println!();

    // 3. レート制限とリトライのテスト
    println!("3. レート制限とリトライのテスト");
    demo_rate_limiting(&config).await?;
    println!();

    println!("=== すべてのデモンストレーション完了 ===");
    Ok(())
}

async fn demo_basic_scraping(
    config: &ScrapingConfig,
    urls: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scraper = WebScraper::new(config.clone());
    
    println!("  {} URLのスクレイピングを開始...", urls.len());
    let start_time = Instant::now();
    
    let results = scraper.scrape_urls(urls).await;
    let elapsed = start_time.elapsed();
    
    println!("  スクレイピング完了 (経過時間: {:.2}秒)", elapsed.as_secs_f64());
    
    // 結果の統計
    let stats = scraper.get_stats().await;
    println!("  統計:");
    println!("    成功: {}", stats.successful_requests);
    println!("    失敗: {}", stats.failed_requests);
    println!("    平均応答時間: {:.0}ms", stats.average_response_time_ms);
    println!("    抽出単語数: {}", stats.total_words_extracted);
    
    Ok(())
}

async fn demo_batch_scraping(
    config: &ScrapingConfig,
    urls: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let batch_scraper = BatchScraper::new(config.clone());
    
    println!("  バッチスクレイピングを開始...");
    let results = batch_scraper.scrape_sitemap(urls).await;
    
    println!("  バッチスクレイピング完了: {} ページ処理", results.len());
    
    // 結果をファイルに保存
    if !results.is_empty() {
        batch_scraper.save_results_json("scraping_results.json").await?;
        batch_scraper.generate_csv_report("scraping_report.csv").await?;
    }
    
    Ok(())
}

async fn demo_rate_limiting(config: &ScrapingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = RateLimitedClient::new(config.clone());
    
    println!("  レート制限テスト（5回連続リクエスト）");
    
    for i in 1..=5 {
        let start = Instant::now();
        let result = client.get_with_rate_limit("https://httpbin.org/get").await;
        let elapsed = start.elapsed();
        
        match result {
            Ok(response) => {
                println!("    リクエスト {}: {} ({}ms)", 
                         i, response.status(), elapsed.as_millis());
            }
            Err(e) => {
                println!("    リクエスト {}: エラー - {} ({}ms)", 
                         i, e, elapsed.as_millis());
            }
        }
        
        // 統計情報を表示
        let stats = client.get_request_stats().await;
        println!("      統計: アクティブ接続={}, 分間リクエスト={}",
                 stats.active_connections, stats.requests_last_minute);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_html_parsing() {
        let html = r#"
            <html>
                <head>
                    <title>Test Page</title>
                    <meta name="description" content="This is a test page">
                </head>
                <body>
                    <a href="/page1">Link 1</a>
                    <a href="https://example.com/page2">Link 2</a>
                </body>
            </html>
        "#;
        
        let title = extract_title(html);
        assert_eq!(title, Some("Test Page".to_string()));
        
        let description = extract_description(html);
        assert_eq!(description, Some("This is a test page".to_string()));
        
        let links = extract_links(html, "https://example.com/");
        assert!(links.contains(&"https://example.com/page1".to_string()));
        assert!(links.contains(&"https://example.com/page2".to_string()));
    }

    #[tokio::test]
    async fn test_rate_limited_client() {
        let config = ScrapingConfig {
            max_concurrent_requests: 2,
            delay_between_requests: Duration::from_millis(100),
            ..Default::default()
        };
        
        let client = RateLimitedClient::new(config);
        
        // 統計情報の初期状態をテスト
        let stats = client.get_request_stats().await;
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.active_connections, 0);
    }

    #[test]
    fn test_scraping_config_default() {
        let config = ScrapingConfig::default();
        assert_eq!(config.max_concurrent_requests, 10);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.user_agent, "Rust-WebScraper/1.0");
    }
}