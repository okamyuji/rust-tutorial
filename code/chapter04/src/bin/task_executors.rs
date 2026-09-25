// src/bin/task_executors.rs

use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== タスクスポーンとエグゼキュータの選択 ===\n");
    
    executor_comparison().await;
    tokio_task_spawning().await;
    task_local_storage().await;
    runtime_configuration().await;
}

// エグゼキュータの比較
async fn executor_comparison() {
    println!("--- 主要な非同期ランタイム ---");
    
    println!("1. Tokio:");
    println!("   - 最も広く使われるランタイム");
    println!("   - マルチスレッド対応");
    println!("   - 豊富な機能（タイマー、I/O、同期プリミティブ）");
    println!("   - 用途: Webサーバー、ネットワークアプリケーション");
    
    println!("\n2. async-std:");
    println!("   - 標準ライブラリに似たAPI");
    println!("   - タスクベースの並行性");
    println!("   - 学習しやすい");
    println!("   - 用途: 一般的な非同期アプリケーション");
    
    println!("\n3. smol:");
    println!("   - 軽量で高速");
    println!("   - 最小限の依存関係");
    println!("   - シンプルなAPI");
    println!("   - 用途: 組み込みシステム、軽量アプリケーション");
    
    println!("\n4. actix:");
    println!("   - アクターモデルベース");
    println!("   - メッセージパッシング");
    println!("   - 高度な並行性制御");
    println!("   - 用途: 複雑な状態管理が必要なシステム");
}

// Tokioでのタスクスポーン
async fn tokio_task_spawning() {
    println!("\n--- Tokioでのタスクスポーン ---");
    
    // 基本的なタスクスポーン
    println!("  基本的なタスクスポーン:");
    
    let handle = task::spawn(async {
        println!("    スポーンされたタスク開始");
        sleep(Duration::from_millis(100)).await;
        println!("    スポーンされたタスク完了");
        42
    });
    
    let result = handle.await.unwrap();
    println!("    タスクの結果: {}", result);
    
    // 複数タスクの並行実行
    println!("\n  複数タスクの並行実行:");
    
    let mut handles = vec![];
    let start = Instant::now();
    
    for i in 0..5 {
        let handle = task::spawn(async move {
            println!("    タスク{} 開始", i);
            sleep(Duration::from_millis(100 * (i as u64 + 1))).await;
            println!("    タスク{} 完了", i);
            i * 2
        });
        handles.push(handle);
    }
    
    // すべてのタスクの完了を待つ
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    println!("    結果: {:?}", results);
    println!("    総実行時間: {:?}", start.elapsed());
    
    // spawn_blocking: ブロッキング操作用
    println!("\n  ブロッキング操作の実行:");
    
    let blocking_result = task::spawn_blocking(|| {
        println!("    CPUバウンドなタスクを実行中...");
        std::thread::sleep(Duration::from_millis(100));
        
        // 重い計算のシミュレーション
        let mut sum = 0u64;
        for i in 0..1_000_000 {
            sum = sum.wrapping_add(i);
        }
        sum
    }).await.unwrap();
    
    println!("    ブロッキングタスクの結果: {}", blocking_result);
    
    // タスクのキャンセル
    println!("\n  タスクのキャンセル:");
    
    let long_task = task::spawn(async {
        for i in 0..10 {
            println!("    長いタスク: ステップ {}", i);
            sleep(Duration::from_millis(100)).await;
        }
        "完了"
    });
    
    // 200ms後にキャンセル
    sleep(Duration::from_millis(250)).await;
    long_task.abort();
    
    match long_task.await {
        Ok(result) => println!("    タスク完了: {}", result),
        Err(e) if e.is_cancelled() => println!("    タスクはキャンセルされました"),
        Err(e) => println!("    タスクエラー: {}", e),
    }
}

// タスクローカルストレージ
async fn task_local_storage() {
    println!("\n--- タスクローカルストレージ ---");
    
    // タスクローカル変数の定義
    tokio::task_local! {
        static REQUEST_ID: String;
    }
    
    async fn process_with_context() {
        // タスクローカル変数の読み取り
        REQUEST_ID.with(|id| {
            println!("    処理中 - リクエストID: {}", id);
        });
        
        sleep(Duration::from_millis(50)).await;
        
        REQUEST_ID.with(|id| {
            println!("    完了 - リクエストID: {}", id);
        });
    }
    
    // 異なるコンテキストでタスクを実行
    let task1 = REQUEST_ID.scope("req-001".to_string(), async {
        println!("  タスク1のコンテキスト:");
        process_with_context().await;
    });
    
    let task2 = REQUEST_ID.scope("req-002".to_string(), async {
        println!("  タスク2のコンテキスト:");
        process_with_context().await;
    });
    
    tokio::join!(task1, task2);
    
    // スコープ外ではアクセスできない
    // REQUEST_ID.with(|_| {}); // パニック！
}

// ランタイムの設定
async fn runtime_configuration() {
    println!("\n--- ランタイムの設定 ---");
    
    // カスタムランタイムの作成（メイン関数外で）
    println!("  カスタムランタイムの例:");
    
    // #[tokio::main] の中でランタイムを作って block_on・drop すると panic するため、ランタイム外のスレッドで試す
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("my-worker")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_all()
            .build()
            .unwrap();
    
        // ランタイム上でタスクを実行
        runtime.spawn(async {
            println!("    カスタムランタイムでタスク実行");
        });
    
        // ブロッキング
        runtime.block_on(async {
            println!("    ブロッキング実行");
            sleep(Duration::from_millis(100)).await;
        });
    })
    .join()
    .unwrap();
    
    println!("\n  現在のランタイム情報:");
    
    // 現在のランタイムハンドル
    let handle = tokio::runtime::Handle::current();
    
    // メトリクスの取得（unstable feature）
    println!("    アクティブなタスク数: （メトリクスAPIは不安定）");
    
    // 異なるランタイム構成
    println!("\n  ランタイム構成オプション:");
    println!("    1. multi_thread: 複数のワーカースレッド");
    println!("    2. current_thread: シングルスレッド");
    println!("    3. enable_io: I/Oドライバを有効化");
    println!("    4. enable_time: タイマーを有効化");
    
    // シングルスレッドランタイムの例（同じ理由で別スレッドから使う）
    std::thread::spawn(|| {
        let single_thread_rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
    
        single_thread_rt.block_on(async {
            println!("\n    シングルスレッドランタイムで実行");
        
            // このランタイムではspawnも同じスレッドで実行
            let handle = task::spawn(async {
                println!("    同じスレッドでスポーン");
            });
        
            handle.await.unwrap();
        });
    })
    .join()
    .unwrap();
    
    // グリーンスレッドとOSスレッド
    println!("\n  グリーンスレッド vs OSスレッド:");
    println!("    - Tokioタスク = グリーンスレッド（軽量）");
    println!("    - spawn_blocking = OSスレッド（重い操作用）");
    
    // パフォーマンスの考慮事項
    println!("\n  パフォーマンスの考慮事項:");
    println!("    1. タスクは軽量（数KB）");
    println!("    2. コンテキストスイッチが高速");
    println!("    3. work-stealingスケジューラ");
    println!("    4. 適切なワーカー数の設定が重要");
}

#[cfg(test)]
mod tests {
    // tokio ランタイムの中から呼んでも panic しないこと
    #[tokio::test]
    async fn runtime_configuration_runs_inside_tokio_runtime() {
        super::runtime_configuration().await;
    }
}
