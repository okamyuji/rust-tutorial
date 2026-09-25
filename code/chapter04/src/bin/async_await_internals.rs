// src/bin/async_await_internals.rs

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== async/awaitの内部動作 ===\n");
    
    state_machine_demo();
    manual_future_impl().await;
    async_block_demo().await;
    generator_transformation().await;
}

// ステートマシンの説明
fn state_machine_demo() {
    println!("--- ステートマシンへの変換 ---");
    
    // async関数は以下のようなステートマシンに変換される
    enum AsyncFunctionState {
        Start,
        AwaitingFirstOperation { start_time: Instant },
        AwaitingSecondOperation { intermediate_result: String },
        Complete,
    }
    
    println!("async関数のステート:");
    println!("  1. Start - 開始状態");
    println!("  2. AwaitingFirstOperation - 最初の.awaitで待機");
    println!("  3. AwaitingSecondOperation - 次の.awaitで待機");
    println!("  4. Complete - 完了状態");
    
    // 実際のasync関数
    async fn example_async_function() -> String {
        println!("\n  [Start] 関数開始");
        
        // 最初の非同期操作
        sleep(Duration::from_millis(100)).await;
        println!("  [AwaitingFirstOperation] 最初の操作完了");
        
        let intermediate = "中間結果".to_string();
        
        // 次の非同期操作
        sleep(Duration::from_millis(100)).await;
        println!("  [AwaitingSecondOperation] 次の操作完了");
        
        format!("{} -> 最終結果", intermediate)
    }
    
    // 実行
    // #[tokio::main] の中で別のランタイムを block_on すると panic するため、ランタイム外のスレッドで実行する
    let result = std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(example_async_function())
    })
    .join()
    .unwrap();
    println!("  [Complete] 結果: {}", result);
}

// 手動でのFuture実装
async fn manual_future_impl() {
    println!("\n--- 手動でのFuture実装 ---");
    
    // カスタムFuture
    struct DelayedValue {
        value: String,
        delay: Duration,
        start: Option<Instant>,
    }
    
    impl DelayedValue {
        fn new(value: String, delay: Duration) -> Self {
            DelayedValue {
                value,
                delay,
                start: None,
            }
        }
    }
    
    impl Future for DelayedValue {
        type Output = String;
        
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.start {
                None => {
                    // 初回のpoll
                    println!("  DelayedValue: 初回poll、タイマー開始");
                    self.start = Some(Instant::now());
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                Some(start) => {
                    // 経過時間をチェック
                    if start.elapsed() >= self.delay {
                        println!("  DelayedValue: 完了!");
                        Poll::Ready(self.value.clone())
                    } else {
                        // まだ時間が経っていない
                        println!("  DelayedValue: まだ待機中... ({}ms経過)", 
                                start.elapsed().as_millis());
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
        }
    }
    
    let future = DelayedValue::new("カスタムFutureの結果".to_string(), 
                                    Duration::from_millis(200));
    let result = future.await;
    println!("  結果: {}", result);
}

// asyncブロックのデモ
async fn async_block_demo() {
    println!("\n--- asyncブロック ---");
    
    // asyncブロックは即座にFutureを返す
    let future1 = async {
        println!("  asyncブロック1: 開始");
        sleep(Duration::from_millis(100)).await;
        println!("  asyncブロック1: 完了");
        42
    };
    
    let future2 = async {
        println!("  asyncブロック2: 開始");
        sleep(Duration::from_millis(150)).await;
        println!("  asyncブロック2: 完了");
        "結果"
    };
    
    // Futureは.awaitするまで実行されない
    println!("  Futureを作成しました（まだ実行されていません）");
    
    // 実行
    let (num, text) = tokio::join!(future1, future2);
    println!("  結果: {} と {}", num, text);
    
    // moveキーワードの使用
    let data = vec![1, 2, 3];
    let moved_future = async move {
        println!("  移動されたデータ: {:?}", data);
        data.iter().sum::<i32>()
    };
    
    let sum = moved_future.await;
    println!("  合計: {}", sum);
}

// ジェネレータへの変換の説明
async fn generator_transformation() {
    println!("\n--- ジェネレータへの変換 ---");
    
    // この関数は...
    async fn multi_step_process() -> Result<String, &'static str> {
        println!("  ステップ1: 初期化");
        let step1_result = async_operation("初期化").await?;
        
        println!("  ステップ2: 処理");
        let step2_result = async_operation(&step1_result).await?;
        
        println!("  ステップ3: 完了");
        let final_result = async_operation(&step2_result).await?;
        
        Ok(final_result)
    }
    
    // 補助関数
    async fn async_operation(input: &str) -> Result<String, &'static str> {
        sleep(Duration::from_millis(50)).await;
        Ok(format!("{} -> 完了", input))
    }
    
    // 実行と状態遷移の観察
    match multi_step_process().await {
        Ok(result) => println!("  最終結果: {}", result),
        Err(e) => println!("  エラー: {}", e),
    }
    
    // async関数の特性
    println!("\n--- async関数の特性 ---");
    
    // 1. 遅延実行
    async fn lazy_execution() {
        println!("  この関数は.awaitされるまで実行されない");
    }
    
    let _future = lazy_execution(); // まだ実行されない
    println!("  Futureを作成（まだ実行されていない）");
    
    // 実行
    _future.await;
    
    // 2. キャンセル可能
    use tokio::select;
    
    let long_operation = async {
        println!("  長い操作開始");
        sleep(Duration::from_secs(10)).await;
        println!("  長い操作完了（実際には到達しない）");
    };
    
    let timeout = sleep(Duration::from_millis(100));
    
    select! {
        _ = long_operation => {
            println!("  操作完了");
        }
        _ = timeout => {
            println!("  タイムアウト！操作はキャンセルされました");
        }
    }
    
    // 3. ゼロコスト抽象化
    println!("\n  async/awaitはゼロコスト抽象化:");
    println!("  - 実行時のオーバーヘッドなし");
    println!("  - 最適化されたステートマシン");
    println!("  - 必要なメモリのみ使用");
}

#[cfg(test)]
mod tests {
    // tokio ランタイムの中から呼んでも panic しないこと
    #[tokio::test]
    async fn state_machine_demo_runs_inside_tokio_runtime() {
        super::state_machine_demo();
    }
}
