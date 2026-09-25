// src/bin/concurrency_patterns.rs

use std::time::{Duration, Instant};
use tokio::{join, select, task, time::sleep};
use futures::future::{join_all, try_join_all, select_all};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Semaphore, Barrier};

#[tokio::main]
async fn main() {
    println!("=== 並行性と並列性 ===\n");
    
    concurrent_vs_parallel().await;
    join_patterns().await;
    select_patterns().await;
    synchronization_primitives().await;
    advanced_patterns().await;
}

// 並行性と並列性の違い
async fn concurrent_vs_parallel() {
    println!("--- 並行性 vs 並列性 ---");
    
    println!("  並行性（Concurrency）:");
    println!("  - 複数のタスクが論理的に同時に進行");
    println!("  - 実際には交互に実行される可能性");
    println!("  - I/Oバウンドなタスクに適している");
    
    println!("\n  並列性（Parallelism）:");
    println!("  - 複数のタスクが物理的に同時に実行");
    println!("  - 複数のCPUコアを使用");
    println!("  - CPUバウンドなタスクに適している");
    
    // 並行実行の例
    println!("\n  並行実行の例:");
    let start = Instant::now();
    
    let task1 = async {
        println!("    タスク1: 開始");
        sleep(Duration::from_millis(200)).await;
        println!("    タスク1: 完了");
        "結果1"
    };
    
    let task2 = async {
        println!("    タスク2: 開始");
        sleep(Duration::from_millis(100)).await;
        println!("    タスク2: 完了");
        "結果2"
    };
    
    let (result1, result2) = join!(task1, task2);
    println!("    結果: {} と {} (時間: {:?})", result1, result2, start.elapsed());
    
    // 並列実行の例
    println!("\n  並列実行の例:");
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4)
        .map(|i| {
            task::spawn_blocking(move || {
                println!("    CPUタスク{}: 開始", i);
                // CPU集約的な作業のシミュレーション
                let mut sum = 0u64;
                for j in 0..10_000_000 {
                    sum = sum.wrapping_add(j);
                }
                println!("    CPUタスク{}: 完了", i);
                sum
            })
        })
        .collect();
    
    let results: Vec<u64> = join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();
    
    println!("    結果: {:?} (時間: {:?})", results, start.elapsed());
}

// joinパターン
async fn join_patterns() {
    println!("\n--- Joinパターン ---");
    
    // 基本的なjoin
    println!("  基本的なjoin:");
    
    async fn fetch_user(id: u32) -> String {
        sleep(Duration::from_millis(100)).await;
        format!("ユーザー{}", id)
    }
    
    async fn fetch_posts(user_id: u32) -> Vec<String> {
        sleep(Duration::from_millis(150)).await;
        vec![format!("投稿1-{}", user_id), format!("投稿2-{}", user_id)]
    }
    
    async fn fetch_comments(post_id: &str) -> Vec<String> {
        sleep(Duration::from_millis(50)).await;
        vec![format!("コメント1-{}", post_id), format!("コメント2-{}", post_id)]
    }
    
    let (user, posts) = join!(
        fetch_user(1),
        fetch_posts(1)
    );
    
    println!("    ユーザー: {}", user);
    println!("    投稿: {:?}", posts);
    
    // join_all for 動的な数のFuture
    println!("\n  join_all:");
    
    let comment_futures: Vec<_> = posts
        .iter()
        .map(|post| fetch_comments(post))
        .collect();
    
    let all_comments = join_all(comment_futures).await;
    println!("    すべてのコメント: {:?}", all_comments);
    
    // try_join_all でエラーハンドリング
    println!("\n  try_join_all:");
    
    async fn fallible_task(id: u32) -> Result<String, String> {
        sleep(Duration::from_millis(50)).await;
        if id == 2 {
            Err(format!("タスク{}でエラー", id))
        } else {
            Ok(format!("タスク{}成功", id))
        }
    }
    
    let tasks: Vec<_> = (0..4).map(|i| fallible_task(i)).collect();
    
    match try_join_all(tasks).await {
        Ok(results) => println!("    すべて成功: {:?}", results),
        Err(e) => println!("    エラー発生: {}", e),
    }
    
    // マクロによる複数のjoin
    println!("\n  複数のjoin:");
    
    let (a, b, c, d) = join!(
        async { sleep(Duration::from_millis(10)).await; "A" },
        async { sleep(Duration::from_millis(20)).await; "B" },
        async { sleep(Duration::from_millis(30)).await; "C" },
        async { sleep(Duration::from_millis(40)).await; "D" },
    );
    
    println!("    結果: {}, {}, {}, {}", a, b, c, d);
}

// selectパターン
async fn select_patterns() {
    println!("\n--- Selectパターン ---");
    
    // 基本的なselect
    println!("  基本的なselect:");
    
    let task1 = sleep(Duration::from_millis(100));
    let task2 = sleep(Duration::from_millis(200));
    
    select! {
        _ = task1 => println!("    タスク1が先に完了"),
        _ = task2 => println!("    タスク2が先に完了"),
    }
    
    // バイアスありselect
    println!("\n  バイアスありselect:");
    
    let mut count1 = 0;
    let mut count2 = 0;
    
    for _ in 0..5 {
        select! {
            biased;
            
            _ = async { sleep(Duration::from_millis(10)).await } => {
                count1 += 1;
            }
            _ = async { sleep(Duration::from_millis(10)).await } => {
                count2 += 1;
            }
        }
    }
    
    println!("    カウント1: {}, カウント2: {} (バイアスあり)", count1, count2);
    
    // select_all
    println!("\n  select_all:");
    
    // async ブロックは1つずつ別の型になるため、トレイトオブジェクトに揃える
    let futures: Vec<Pin<Box<dyn Future<Output = &str> + Send>>> = vec![
        Box::pin(async { sleep(Duration::from_millis(300)).await; "遅い" }),
        Box::pin(async { sleep(Duration::from_millis(100)).await; "速い" }),
        Box::pin(async { sleep(Duration::from_millis(200)).await; "中間" }),
    ];
    
    let (result, index, remaining) = select_all(futures).await;
    println!("    最初に完了: {} (インデックス: {})", result, index);
    println!("    残りのタスク数: {}", remaining.len());
    
    // タイムアウト付きselect
    println!("\n  タイムアウト付きselect:");
    
    let long_task = async {
        sleep(Duration::from_secs(10)).await;
        "完了"
    };
    
    select! {
        result = long_task => {
            println!("    タスク完了: {}", result);
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("    タイムアウト！");
        }
    }
    
    // キャンセレーション
    println!("\n  キャンセレーション:");
    
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
    
    let cancellable_task = async {
        select! {
            _ = async {
                for i in 0..10 {
                    println!("    作業中... {}", i);
                    sleep(Duration::from_millis(100)).await;
                }
            } => {
                println!("    タスク完了");
            }
            _ = rx => {
                println!("    タスクがキャンセルされました");
            }
        }
    };
    
    tokio::spawn(async move {
        sleep(Duration::from_millis(250)).await;
        tx.send(()).unwrap();
    });
    
    cancellable_task.await;
}

// 同期プリミティブ
async fn synchronization_primitives() {
    println!("\n--- 同期プリミティブ ---");
    
    // Mutex
    println!("  Mutex:");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = task::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
            println!("    タスク{}: カウンタ = {}", i, *num);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    println!("    最終カウント: {}", *counter.lock().await);
    
    // RwLock
    println!("\n  RwLock:");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // 複数の読み取り
    let read_handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            task::spawn(async move {
                let values = data.read().await;
                println!("    リーダー{}: {:?}", i, *values);
                sleep(Duration::from_millis(100)).await;
            })
        })
        .collect();
    
    join_all(read_handles).await;
    
    // 書き込み
    {
        let mut values = data.write().await;
        values.push(4);
        println!("    ライター: 4を追加");
    }
    
    println!("    最終データ: {:?}", *data.read().await);
    
    // Semaphore
    println!("\n  Semaphore:");
    
    let semaphore = Arc::new(Semaphore::new(2)); // 同時実行数を2に制限
    let mut handles = vec![];
    
    for i in 0..5 {
        let semaphore = Arc::clone(&semaphore);
        let handle = task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("    タスク{}: 実行開始", i);
            sleep(Duration::from_millis(200)).await;
            println!("    タスク{}: 実行完了", i);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    // Barrier
    println!("\n  Barrier:");
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = task::spawn(async move {
            println!("    ワーカー{}: 第1フェーズ", i);
            sleep(Duration::from_millis(100 * (i as u64 + 1))).await;
            
            barrier.wait().await;
            
            println!("    ワーカー{}: 第2フェーズ", i);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
}

// 高度なパターン
async fn advanced_patterns() {
    println!("\n--- 高度な並行性パターン ---");
    
    // ファンアウト・ファンイン
    println!("  ファンアウト・ファンイン:");
    
    use tokio::sync::mpsc;
    
    let (tx, mut rx) = mpsc::channel(100);
    
    // ファンアウト: 複数のワーカー
    for i in 0..3 {
        let tx = tx.clone();
        task::spawn(async move {
            for j in 0..3 {
                let value = i * 10 + j;
                tx.send(value).await.unwrap();
                sleep(Duration::from_millis(50)).await;
            }
        });
    }
    
    drop(tx); // 元のsenderをドロップ
    
    // ファンイン: 単一のコレクター
    let collector = task::spawn(async move {
        let mut results = vec![];
        while let Some(value) = rx.recv().await {
            results.push(value);
        }
        results.sort();
        results
    });
    
    let results = collector.await.unwrap();
    println!("    収集結果: {:?}", results);
    
    // パイプライン処理
    println!("\n  パイプライン処理:");
    
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);
    
    // ステージ1: 生成
    task::spawn(async move {
        for i in 0..5 {
            tx1.send(i).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
    });
    
    // ステージ2: 変換
    task::spawn(async move {
        while let Some(value) = rx1.recv().await {
            let transformed = value * 2;
            tx2.send(transformed).await.unwrap();
        }
    });
    
    // ステージ3: 消費
    task::spawn(async move {
        while let Some(value) = rx2.recv().await {
            println!("    パイプライン出力: {}", value);
        }
    });
    
    sleep(Duration::from_millis(500)).await;
    
    // レート制限
    println!("\n  レート制限:");
    
    use tokio::time::interval;
    
    let mut interval = interval(Duration::from_millis(100));
    let tasks: Vec<_> = (0..10).map(|i| async move {
        println!("    タスク{} 待機中", i);
        i
    }).collect();
    
    for task in tasks {
        interval.tick().await;
        let result = task.await;
        println!("    タスク{} 実行", result);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    // 各デモが完走すること（Pending のまま wake されないなどのハングも検出する）
    #[tokio::test]
    async fn select_patterns_completes() {
        tokio::time::timeout(Duration::from_secs(30), super::select_patterns())
            .await
            .expect("select_patterns がハングした");
    }
}
