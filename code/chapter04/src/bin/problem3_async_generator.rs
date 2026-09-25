//! 第4章 問題3: 非同期ジェネレーター
//! 
//! この問題では、非同期ジェネレーターパターンを学習します。
//! 非同期でデータを段階的に生成し、メモリ効率的にストリーミング処理を行う方法を実装します。

use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};

/// 非同期ジェネレーターの基本実装
/// 
/// 指定された範囲の数値を一定間隔で生成します
pub struct AsyncNumberGenerator {
    current: usize,
    max: usize,
    interval: Duration,
    last_yield: Option<Instant>,
}

impl AsyncNumberGenerator {
    pub fn new(max: usize, interval: Duration) -> Self {
        Self {
            current: 0,
            max,
            interval,
            last_yield: None,
        }
    }
}

impl Stream for AsyncNumberGenerator {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current >= self.max {
            return Poll::Ready(None);
        }

        let now = Instant::now();
        
        // 初回またはインターバル経過をチェック
        if let Some(last) = self.last_yield {
            if now.duration_since(last) < self.interval {
                // まだ時間が経っていない場合、後でポーリングし直す
                let sleep_until = last + self.interval;
                let waker = cx.waker().clone();
                let duration = sleep_until.saturating_duration_since(now);
                
                tokio::spawn(async move {
                    sleep(duration).await;
                    waker.wake();
                });
                
                return Poll::Pending;
            }
        }

        self.last_yield = Some(now);
        let value = self.current;
        self.current += 1;
        
        Poll::Ready(Some(value))
    }
}

/// より高度な非同期ジェネレーター: データベースレコードのストリーミング
pub struct AsyncDatabaseStream {
    records: Vec<String>,
    position: usize,
    batch_size: usize,
    processing_delay: Duration,
}

impl AsyncDatabaseStream {
    pub fn new(total_records: usize, batch_size: usize, processing_delay: Duration) -> Self {
        // 模擬的なデータベースレコードを生成
        let records: Vec<String> = (0..total_records)
            .map(|i| format!("record_{:06}", i))
            .collect();

        Self {
            records,
            position: 0,
            batch_size,
            processing_delay,
        }
    }
}

impl Stream for AsyncDatabaseStream {
    type Item = Vec<String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.position >= self.records.len() {
            return Poll::Ready(None);
        }

        let end = std::cmp::min(self.position + self.batch_size, self.records.len());
        let batch: Vec<String> = self.records[self.position..end].to_vec();
        self.position = end;

        // 非同期処理の模擬（データベースI/O）
        let delay = self.processing_delay;
        let waker = cx.waker().clone();
        
        tokio::spawn(async move {
            sleep(delay).await;
            waker.wake();
        });

        Poll::Ready(Some(batch))
    }
}

/// 無限ストリーム: システムメトリクスの定期監視
pub struct SystemMetricsStream {
    interval: Duration,
    last_poll: Option<Instant>,
    cpu_base: f64,
    memory_base: f64,
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub timestamp: Instant,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_io: u64,
}

impl SystemMetricsStream {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_poll: None,
            cpu_base: 10.0,
            memory_base: 50.0,
        }
    }

    fn generate_metrics(&mut self) -> SystemMetrics {
        use std::f64::consts::PI;
        
        let now = Instant::now();
        let elapsed = self.last_poll
            .map(|last| now.duration_since(last).as_secs_f64())
            .unwrap_or(0.0);

        // 模擬的なメトリクス生成（sin波を使って変動を作る）
        let time_factor = elapsed * 0.1;
        let cpu_variation = (time_factor * PI).sin() * 20.0;
        let memory_variation = (time_factor * PI * 0.5).sin() * 15.0;

        SystemMetrics {
            timestamp: now,
            cpu_usage: (self.cpu_base + cpu_variation).max(0.0).min(100.0),
            memory_usage: (self.memory_base + memory_variation).max(0.0).min(100.0),
            disk_io: ((elapsed * 1000.0) as u64) % 10000,
        }
    }
}

impl Stream for SystemMetricsStream {
    type Item = SystemMetrics;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let now = Instant::now();
        
        if let Some(last) = self.last_poll {
            if now.duration_since(last) < self.interval {
                // インターバルが経過していない場合
                let sleep_until = last + self.interval;
                let waker = cx.waker().clone();
                let duration = sleep_until.saturating_duration_since(now);
                
                tokio::spawn(async move {
                    sleep(duration).await;
                    waker.wake();
                });
                
                return Poll::Pending;
            }
        }

        self.last_poll = Some(now);
        let metrics = self.generate_metrics();
        
        Poll::Ready(Some(metrics))
    }
}

/// バックプレッシャー制御を持つ非同期ジェネレーター
pub struct BackpressureStream {
    buffer: Arc<Mutex<Vec<String>>>,
    max_buffer_size: usize,
    production_rate: Duration,
    last_produce: Option<Instant>,
    finished: bool,
    total_items: usize,
    produced_count: usize,
}

impl BackpressureStream {
    pub fn new(total_items: usize, max_buffer_size: usize, production_rate: Duration) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            max_buffer_size,
            production_rate,
            last_produce: None,
            finished: false,
            total_items,
            produced_count: 0,
        }
    }

    fn try_produce_item(&mut self) -> bool {
        if self.produced_count >= self.total_items {
            self.finished = true;
            return false;
        }

        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() >= self.max_buffer_size {
            // バッファが満杯の場合、新しいアイテムを生成しない
            return false;
        }

        let item = format!("item_{:06}", self.produced_count);
        buffer.push(item);
        self.produced_count += 1;
        true
    }
}

impl Stream for BackpressureStream {
    type Item = String;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // バッファからアイテムを取得
        {
            let mut buffer = self.buffer.lock().unwrap();
            if let Some(item) = buffer.pop() {
                return Poll::Ready(Some(item));
            }
        }

        // ストリーム終了チェック
        if self.finished {
            return Poll::Ready(None);
        }

        // 新しいアイテムの生成タイミングをチェック
        let now = Instant::now();
        let should_produce = self.last_produce
            .map(|last| now.duration_since(last) >= self.production_rate)
            .unwrap_or(true);

        if should_produce {
            self.last_produce = Some(now);
            if self.try_produce_item() {
                // アイテムが生成されたので、すぐに取得
                let mut buffer = self.buffer.lock().unwrap();
                if let Some(item) = buffer.pop() {
                    return Poll::Ready(Some(item));
                }
            }
        }

        // 直前の try_produce_item で生成し終えた場合、wake を予約せず Pending を返すと永久に止まる
        if self.finished {
            return Poll::Ready(None);
        }

        // まだ生成するアイテムがある場合、後でポーリング
        let waker = cx.waker().clone();
        let delay = self.production_rate;

        tokio::spawn(async move {
            sleep(delay).await;
            waker.wake();
        });

        Poll::Pending
    }
}

/// 実用的な使用例とデモンストレーション
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 非同期ジェネレーターのデモンストレーション ===\n");

    // 1. 基本的な数値ジェネレーター
    println!("1. 基本的な数値ジェネレーター（1秒間隔で5個の数値を生成）");
    let mut number_gen = AsyncNumberGenerator::new(5, Duration::from_millis(500));
    
    let start = Instant::now();
    while let Some(num) = number_gen.next().await {
        let elapsed = start.elapsed();
        println!("  生成: {} (経過時間: {:.2}秒)", num, elapsed.as_secs_f64());
    }
    println!("  完了: 数値ジェネレーター\n");

    // 2. データベースストリーミング
    println!("2. データベースレコードのバッチストリーミング");
    let mut db_stream = AsyncDatabaseStream::new(10, 3, Duration::from_millis(200));
    
    let mut batch_count = 0;
    while let Some(batch) = db_stream.next().await {
        batch_count += 1;
        println!("  バッチ {}: {:?}", batch_count, batch);
    }
    println!("  完了: データベースストリーミング\n");

    // 3. システムメトリクス監視（5回のみ）
    println!("3. システムメトリクス監視（300ms間隔で5回）");
    let mut metrics_stream = SystemMetricsStream::new(Duration::from_millis(300));
    
    for i in 0..5 {
        if let Some(metrics) = metrics_stream.next().await {
            println!("  #{}: CPU: {:.1}%, Memory: {:.1}%, Disk I/O: {} KB", 
                     i + 1, metrics.cpu_usage, metrics.memory_usage, metrics.disk_io);
        }
    }
    println!("  完了: システムメトリクス監視\n");

    // 4. バックプレッシャー制御
    println!("4. バックプレッシャー制御ストリーム（バッファサイズ: 3, 生成間隔: 100ms）");
    let mut backpressure_stream = BackpressureStream::new(8, 3, Duration::from_millis(100));
    
    let mut item_count = 0;
    while let Some(item) = backpressure_stream.next().await {
        item_count += 1;
        println!("  受信: {} (#{}/8)", item, item_count);
        
        // 意図的に処理を遅延させてバックプレッシャーをテスト
        if item_count % 2 == 0 {
            println!("    処理遅延中...");
            sleep(Duration::from_millis(250)).await;
        }
    }
    println!("  完了: バックプレッシャー制御\n");

    // 5. ストリーム変換とフィルタリング
    println!("5. ストリーム変換とフィルタリングの例");
    let number_stream = AsyncNumberGenerator::new(10, Duration::from_millis(100));
    
    let transformed: Vec<String> = number_stream
        .filter(|n| futures::future::ready(*n % 2 == 0)) // 偶数のみ
        .map(|n| format!("偶数: {}", n))
        .take(3) // 最初の3個のみ
        .collect()
        .await;
    
    println!("  フィルタリング結果: {:?}", transformed);
    println!("  完了: ストリーム変換\n");

    println!("=== すべてのデモンストレーション完了 ===");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn test_async_number_generator() {
        let mut gen = AsyncNumberGenerator::new(3, Duration::from_millis(10));
        
        let numbers: Vec<usize> = gen.collect().await;
        assert_eq!(numbers, vec![0, 1, 2]);
    }

    #[tokio::test]
    async fn test_database_stream() {
        let mut stream = AsyncDatabaseStream::new(5, 2, Duration::from_millis(1));
        
        let mut batches = Vec::new();
        while let Some(batch) = stream.next().await {
            batches.push(batch);
        }
        
        assert_eq!(batches.len(), 3); // 5個のレコードを2個ずつ: [2, 2, 1]
        assert_eq!(batches[0].len(), 2);
        assert_eq!(batches[1].len(), 2);
        assert_eq!(batches[2].len(), 1);
    }

    #[tokio::test]
    async fn test_backpressure_stream() {
        let mut stream = BackpressureStream::new(5, 10, Duration::from_millis(1));
        
        let items: Vec<String> = stream.collect().await;
        assert_eq!(items.len(), 5);
        
        for (i, item) in items.iter().enumerate() {
            assert_eq!(item, &format!("item_{:06}", i));
        }
    }

    #[tokio::test]
    async fn test_stream_transformation() {
        let stream = AsyncNumberGenerator::new(6, Duration::from_millis(1));
        
        let even_numbers: Vec<usize> = stream
            .filter(|n| futures::future::ready(*n % 2 == 0))
            .collect()
            .await;
        
        assert_eq!(even_numbers, vec![0, 2, 4]);
    }
}