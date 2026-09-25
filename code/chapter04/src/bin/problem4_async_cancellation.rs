//! 第4章 問題4: 非同期キャンセレーション
//! 
//! この問題では、非同期処理のキャンセレーション機能を学習します。
//! タイムアウト、ユーザーによる中断、リソース管理などの実装方法を学びます。

use futures::{select, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

/// キャンセレーション可能なタスクの基本実装
pub struct CancellableTask {
    token: CancellationToken,
}

impl CancellableTask {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    pub fn cancel_token(&self) -> CancellationToken {
        self.token.clone()
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }

    /// 長時間実行されるタスクの例（計算集約的処理）
    pub async fn compute_intensive_task(&self, iterations: u64) -> Result<u64, &'static str> {
        let mut result = 0u64;
        
        for i in 0..iterations {
            // 定期的にキャンセレーションをチェック
            if i % 1000 == 0 {
                if self.token.is_cancelled() {
                    return Err("タスクがキャンセルされました");
                }
            }
            
            // 重い計算の模擬
            result = result.wrapping_add(i * i);
            
            // 少し待機（非同期ポイント）
            if i % 10000 == 0 {
                tokio::task::yield_now().await;
            }
        }
        
        Ok(result)
    }

    /// I/O集約的なタスクの例（ファイルダウンロード模擬）
    pub async fn download_task(&self, file_size: usize, chunk_size: usize) -> Result<Vec<u8>, &'static str> {
        let mut downloaded_data = Vec::new();
        let total_chunks = (file_size + chunk_size - 1) / chunk_size;
        
        for chunk_idx in 0..total_chunks {
            // キャンセレーションチェック
            tokio::select! {
                _ = self.token.cancelled() => {
                    return Err("ダウンロードがキャンセルされました");
                }
                _ = sleep(Duration::from_millis(100)) => {
                    // チャンクのダウンロード模擬
                    let remaining = file_size - downloaded_data.len();
                    let current_chunk_size = std::cmp::min(chunk_size, remaining);
                    
                    let chunk: Vec<u8> = (0..current_chunk_size)
                        .map(|i| ((chunk_idx * chunk_size + i) % 256) as u8)
                        .collect();
                    
                    downloaded_data.extend(chunk);
                    
                    println!("  ダウンロード進捗: {:.1}% ({}/{})",
                             (downloaded_data.len() as f64 / file_size as f64) * 100.0,
                             downloaded_data.len(),
                             file_size);
                }
            }
        }
        
        Ok(downloaded_data)
    }
}

/// タイムアウト付きの非同期操作
pub struct TimeoutHandler;

impl TimeoutHandler {
    /// タイムアウト付きで関数を実行
    pub async fn with_timeout<F, T>(
        future: F,
        duration: Duration,
    ) -> Result<T, &'static str>
    where
        F: Future<Output = T>,
    {
        match timeout(duration, future).await {
            Ok(result) => Ok(result),
            Err(_) => Err("タイムアウトが発生しました"),
        }
    }

    /// 複数の選択肢から最初に完了したものを選択
    pub async fn select_first<F1, F2, T1, T2>(
        future1: F1,
        future2: F2,
    ) -> Result<T1, T2>
    where
        F1: Future<Output = T1>,
        F2: Future<Output = T2>,
    {
        select! {
            result1 = future1.fuse() => Ok(result1),
            result2 = future2.fuse() => Err(result2),
        }
    }
}

/// グレースフルシャットダウン機能を持つワーカー
pub struct Worker {
    shutdown_tx: Option<oneshot::Sender<()>>,
    task_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            shutdown_tx: None,
            task_handle: None,
        }
    }

    /// ワーカーを開始
    pub fn start(&mut self) {
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        
        let handle = tokio::spawn(async move {
            let mut counter = 0u64;
            
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        println!("  ワーカー: グレースフルシャットダウンを受信");
                        break;
                    }
                    _ = sleep(Duration::from_millis(200)) => {
                        counter += 1;
                        println!("  ワーカー: 作業中... (#{:3})", counter);
                        
                        // 長時間実行のシミュレーション
                        if counter >= 20 {
                            println!("  ワーカー: 自然終了");
                            break;
                        }
                    }
                }
            }
            
            // クリーンアップ処理
            println!("  ワーカー: クリーンアップ完了");
        });

        self.shutdown_tx = Some(shutdown_tx);
        self.task_handle = Some(handle);
    }

    /// グレースフルシャットダウン
    pub async fn shutdown(mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        if let Some(handle) = self.task_handle.take() {
            let _ = handle.await;
        }
    }

    /// 強制終了
    pub fn force_shutdown(mut self) {
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }
    }
}

/// 複数タスクの協調的キャンセレーション
pub struct TaskGroup {
    pub token: CancellationToken,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl TaskGroup {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            tasks: Vec::new(),
        }
    }

    /// タスクを追加
    pub fn spawn_task<F>(&mut self, task_name: String, task: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let token = self.token.clone();
        
        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = token.cancelled() => {
                    println!("  タスク '{}' がキャンセルされました", task_name);
                }
                _ = task => {
                    println!("  タスク '{}' が正常に完了しました", task_name);
                }
            }
        });

        self.tasks.push(handle);
    }

    /// すべてのタスクをキャンセル
    pub fn cancel_all(&self) {
        self.token.cancel();
    }

    /// すべてのタスクの完了を待機
    pub async fn wait_all(self) {
        for handle in self.tasks {
            let _ = handle.await;
        }
    }
}

/// リソース管理とクリーンアップ
pub struct ResourceManager {
    resources: Vec<String>,
    cleanup_performed: bool,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            cleanup_performed: false,
        }
    }

    pub async fn acquire_resource(&mut self, name: &str) -> Result<(), &'static str> {
        // リソース取得の模擬
        sleep(Duration::from_millis(100)).await;
        self.resources.push(name.to_string());
        println!("  リソース '{}' を取得しました", name);
        Ok(())
    }

    pub async fn perform_work_with_resources(&self) -> Result<(), &'static str> {
        for (i, resource) in self.resources.iter().enumerate() {
            println!("  リソース '{}' を使用して作業中... ({}/{})", 
                     resource, i + 1, self.resources.len());
            sleep(Duration::from_millis(200)).await;
        }
        Ok(())
    }

    pub async fn cleanup(&mut self) {
        if self.cleanup_performed {
            return;
        }

        println!("  リソースクリーンアップを開始...");
        for resource in &self.resources {
            println!("    リソース '{}' を解放中...", resource);
            sleep(Duration::from_millis(50)).await;
        }
        
        self.resources.clear();
        self.cleanup_performed = true;
        println!("  リソースクリーンアップ完了");
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        if !self.cleanup_performed && !self.resources.is_empty() {
            println!("  警告: ResourceManagerがdropされましたが、クリーンアップが実行されていません");
            println!("  未解放のリソース: {:?}", self.resources);
        }
    }
}

/// 実用的な使用例とデモンストレーション
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 非同期キャンセレーションのデモンストレーション ===\n");

    // 1. 基本的なキャンセレーション
    println!("1. 基本的なキャンセレーション");
    demo_basic_cancellation().await?;
    println!();

    // 2. タイムアウト処理
    println!("2. タイムアウト処理");
    demo_timeout_handling().await?;
    println!();

    // 3. グレースフルシャットダウン
    println!("3. グレースフルシャットダウン");
    demo_graceful_shutdown().await?;
    println!();

    // 4. 複数タスクの協調的キャンセレーション
    println!("4. 複数タスクの協調的キャンセレーション");
    demo_task_group_cancellation().await?;
    println!();

    // 5. リソース管理とクリーンアップ
    println!("5. リソース管理とクリーンアップ");
    demo_resource_management().await?;
    println!();

    println!("=== すべてのデモンストレーション完了 ===");
    Ok(())
}

async fn demo_basic_cancellation() -> Result<(), Box<dyn std::error::Error>> {
    let task = CancellableTask::new();
    let cancel_token = task.cancel_token();

    // 2秒後にキャンセル
    let cancel_handle = tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        cancel_token.cancel();
        println!("  キャンセル信号を送信しました");
    });

    // 計算集約的なタスク（通常5秒かかる）
    println!("  計算集約的タスクを開始...");
    let start = Instant::now();
    
    match task.compute_intensive_task(100_000_000).await {
        Ok(result) => {
            println!("  タスク完了: 結果 = {}, 経過時間: {:.2}秒", 
                     result, start.elapsed().as_secs_f64());
        }
        Err(msg) => {
            println!("  {}, 経過時間: {:.2}秒", 
                     msg, start.elapsed().as_secs_f64());
        }
    }

    cancel_handle.await?;
    Ok(())
}

async fn demo_timeout_handling() -> Result<(), Box<dyn std::error::Error>> {
    let task = CancellableTask::new();

    // タイムアウト付きダウンロード
    println!("  ダウンロードタスクを開始... (タイムアウト: 3秒)");
    let start = Instant::now();
    
    match TimeoutHandler::with_timeout(
        task.download_task(1000, 50),
        Duration::from_secs(3),
    ).await {
        Ok(Ok(data)) => {
            println!("  ダウンロード完了: {} バイト, 経過時間: {:.2}秒", 
                     data.len(), start.elapsed().as_secs_f64());
        }
        Ok(Err(msg)) => {
            println!("  ダウンロードエラー: {}, 経過時間: {:.2}秒", 
                     msg, start.elapsed().as_secs_f64());
        }
        Err(msg) => {
            println!("  {}, 経過時間: {:.2}秒", 
                     msg, start.elapsed().as_secs_f64());
        }
    }

    Ok(())
}

async fn demo_graceful_shutdown() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = Worker::new();
    
    println!("  ワーカーを開始...");
    worker.start();
    
    // 3秒後にシャットダウンシグナル
    sleep(Duration::from_secs(3)).await;
    println!("  グレースフルシャットダウンを開始...");
    
    worker.shutdown().await;
    println!("  ワーカーのシャットダウン完了");
    
    Ok(())
}

async fn demo_task_group_cancellation() -> Result<(), Box<dyn std::error::Error>> {
    let mut task_group = TaskGroup::new();
    
    // 複数のタスクを追加
    task_group.spawn_task("データ処理".to_string(), async {
        for i in 1..=10 {
            sleep(Duration::from_millis(300)).await;
            println!("    データ処理: ステップ {}/10", i);
        }
    });
    
    task_group.spawn_task("ログ出力".to_string(), async {
        for i in 1..=15 {
            sleep(Duration::from_millis(200)).await;
            println!("    ログ出力: メッセージ {}/15", i);
        }
    });
    
    task_group.spawn_task("監視タスク".to_string(), async {
        for i in 1..=8 {
            sleep(Duration::from_millis(400)).await;
            println!("    監視タスク: チェック {}/8", i);
        }
    });
    
    // 2秒後にすべてのタスクをキャンセル
    let cancel_token = task_group.token.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        println!("  すべてのタスクをキャンセル中...");
        cancel_token.cancel();
    });
    
    task_group.wait_all().await;
    println!("  すべてのタスクが完了しました");
    
    Ok(())
}

async fn demo_resource_management() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ResourceManager::new();
    
    // リソース取得
    println!("  リソースを取得中...");
    manager.acquire_resource("データベース接続").await?;
    manager.acquire_resource("ファイルハンドル").await?;
    manager.acquire_resource("ネットワークソケット").await?;
    
    // キャンセレーション可能な作業
    let token = CancellationToken::new();
    let cancel_token = token.clone();
    
    // 3秒後にキャンセル
    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        cancel_token.cancel();
        println!("  作業のキャンセル信号を送信");
    });
    
    // 作業実行またはキャンセレーション
    tokio::select! {
        result = manager.perform_work_with_resources() => {
            match result {
                Ok(_) => println!("  作業が正常に完了しました"),
                Err(msg) => println!("  作業エラー: {}", msg),
            }
        }
        _ = token.cancelled() => {
            println!("  作業がキャンセルされました");
        }
    }
    
    // 必ずクリーンアップを実行
    manager.cleanup().await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cancellable_task() {
        let task = CancellableTask::new();
        
        // キャンセルトークンを即座にキャンセル
        task.cancel();
        
        let result = task.compute_intensive_task(1000).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "タスクがキャンセルされました");
    }

    #[tokio::test]
    async fn test_timeout_handling() {
        let task = CancellableTask::new();
        
        // 非常に短いタイムアウト
        let result = TimeoutHandler::with_timeout(
            task.download_task(1000, 100),
            Duration::from_millis(1),
        ).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "タイムアウトが発生しました");
    }

    #[tokio::test]
    async fn test_resource_manager_cleanup() {
        let mut manager = ResourceManager::new();
        
        manager.acquire_resource("test_resource").await.unwrap();
        assert_eq!(manager.resources.len(), 1);
        
        manager.cleanup().await;
        assert_eq!(manager.resources.len(), 0);
        assert!(manager.cleanup_performed);
    }

    #[tokio::test]
    async fn test_task_group_cancellation() {
        let mut task_group = TaskGroup::new();
        
        task_group.spawn_task("test_task".to_string(), async {
            sleep(Duration::from_secs(10)).await; // 長時間のタスク
        });
        
        // 即座にキャンセル
        task_group.cancel_all();
        
        // タスクグループが速やかに完了することを確認
        let start = Instant::now();
        task_group.wait_all().await;
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_secs(1), "キャンセレーションが効果的でない");
    }
}