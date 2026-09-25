// src/main.rs
// 第4章：非同期プログラミングとFuture - メインプログラム

use std::time::Duration;
use tokio::time::sleep;
use futures::future;

#[tokio::main]
async fn main() {
    println!("=== 第4章：非同期プログラミングとFuture ===\n");
    
    basic_async_await().await;
    println!();
    concurrent_execution().await;
    println!();
    sequential_vs_concurrent().await;
}

// 基本的なasync/await
async fn basic_async_await() {
    println!("--- 基本的なasync/await ---");
    
    // 非同期関数の定義
    async fn fetch_data(id: u32) -> String {
        println!("  データ{}の取得を開始...", id);
        sleep(Duration::from_millis(100)).await;
        format!("データ{}", id)
    }
    
    // 非同期関数の呼び出し
    let data1 = fetch_data(1).await;
    println!("  取得完了: {}", data1);
    
    let data2 = fetch_data(2).await;
    println!("  取得完了: {}", data2);
    
    // 複数の非同期操作
    async fn process_data(data: String) -> String {
        println!("  {}を処理中...", data);
        sleep(Duration::from_millis(50)).await;
        format!("処理済み{}", data)
    }
    
    let result = process_data(data1).await;
    println!("  結果: {}", result);
}

// 並行実行
async fn concurrent_execution() {
    println!("--- 並行実行 ---");
    
    // 複数のタスクを同時に実行
    async fn task(name: &str, duration: u64) -> String {
        println!("  タスク'{}' 開始", name);
        sleep(Duration::from_millis(duration)).await;
        println!("  タスク'{}' 完了", name);
        format!("タスク'{}'の結果", name)
    }
    
    // tokio::join!を使った並行実行
    let (result1, result2, result3) = tokio::join!(
        task("A", 100),
        task("B", 200),
        task("C", 150)
    );
    
    println!("  結果: {}, {}, {}", result1, result2, result3);
    
    // selectを使った最初に完了したタスクの選択
    use tokio::select;
    
    println!("\n  最初に完了したタスクを待機:");
    
    select! {
        _ = sleep(Duration::from_millis(100)) => {
            println!("  タイムアウト1が最初に完了");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("  タイムアウト2が最初に完了");
        }
    }
}

// 順次実行と並行実行の比較
async fn sequential_vs_concurrent() {
    println!("--- 順次実行 vs 並行実行 ---");
    
    async fn fetch_user(id: u32) -> String {
        sleep(Duration::from_millis(100)).await;
        format!("ユーザー{}", id)
    }
    
    async fn fetch_posts(user_id: u32) -> Vec<String> {
        sleep(Duration::from_millis(150)).await;
        vec![
            format!("ユーザー{}の投稿1", user_id),
            format!("ユーザー{}の投稿2", user_id),
        ]
    }
    
    async fn fetch_comments(post_id: &str) -> Vec<String> {
        sleep(Duration::from_millis(50)).await;
        vec![
            format!("{}へのコメント1", post_id),
            format!("{}へのコメント2", post_id),
        ]
    }
    
    // 順次実行
    println!("  順次実行:");
    let start = std::time::Instant::now();
    
    let user = fetch_user(1).await;
    let posts = fetch_posts(1).await;
    let mut all_comments = Vec::new();
    
    for post in &posts {
        let comments = fetch_comments(post).await;
        all_comments.extend(comments);
    }
    
    println!("    ユーザー: {}", user);
    println!("    投稿数: {}", posts.len());
    println!("    コメント数: {}", all_comments.len());
    println!("    実行時間: {:?}", start.elapsed());
    
    // 並行実行
    println!("\n  並行実行:");
    let start = std::time::Instant::now();
    
    let user_future = fetch_user(2);
    let posts_future = fetch_posts(2);
    
    let (user, posts) = tokio::join!(user_future, posts_future);
    
    let comment_futures: Vec<_> = posts
        .iter()
        .map(|post| fetch_comments(post))
        .collect();
    
    let all_comments: Vec<Vec<String>> = future::join_all(comment_futures).await;
    let all_comments: Vec<String> = all_comments.into_iter().flatten().collect();
    
    println!("    ユーザー: {}", user);
    println!("    投稿数: {}", posts.len());
    println!("    コメント数: {}", all_comments.len());
    println!("    実行時間: {:?}", start.elapsed());
    
    println!("\n  並行実行は順次実行よりも高速です！");
}
