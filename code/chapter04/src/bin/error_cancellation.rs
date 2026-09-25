// src/bin/error_cancellation.rs

use std::time::Duration;
use tokio::{select, time::{sleep, timeout, interval}};
use tokio::sync::oneshot;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
async fn main() {
    println!("=== エラーハンドリングとキャンセレーション ===\n");
    
    error_handling_patterns().await;
    timeout_patterns().await;
    cancellation_patterns().await;
    graceful_shutdown().await;
    error_propagation().await;
}

// エラーハンドリングパターン
async fn error_handling_patterns() {
    println!("--- エラーハンドリングパターン ---");
    
    // 基本的なResult型の使用
    println!("  基本的なエラーハンドリング:");
    
    #[derive(Debug)]
    enum AppError {
        Network(String),
        Timeout,
        InvalidData(String),
        Internal(String),
    }
    
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AppError::Network(msg) => write!(f, "ネットワークエラー: {}", msg),
                AppError::Timeout => write!(f, "タイムアウト"),
                AppError::InvalidData(msg) => write!(f, "無効なデータ: {}", msg),
                AppError::Internal(msg) => write!(f, "内部エラー: {}", msg),
            }
        }
    }
    
    impl std::error::Error for AppError {}
    
    async fn fetch_data(url: &str) -> Result<String, AppError> {
        if url.is_empty() {
            return Err(AppError::InvalidData("URLが空です".to_string()));
        }
        
        // ネットワーク操作のシミュレーション
        sleep(Duration::from_millis(100)).await;
        
        if url.contains("error") {
            Err(AppError::Network("接続失敗".to_string()))
        } else {
            Ok(format!("データ from {}", url))
        }
    }
    
    // エラーハンドリング
    match fetch_data("https://example.com").await {
        Ok(data) => println!("    成功: {}", data),
        Err(e) => println!("    エラー: {}", e),
    }
    
    match fetch_data("https://error.com").await {
        Ok(data) => println!("    成功: {}", data),
        Err(e) => println!("    エラー: {}", e),
    }
    
    // ?演算子の使用
    println!("\n  ?演算子の使用:");
    
    async fn process_multiple_urls() -> Result<Vec<String>, AppError> {
        let urls = vec!["https://site1.com", "https://site2.com", "https://site3.com"];
        let mut results = Vec::new();
        
        for url in urls {
            let data = fetch_data(url).await?;
            results.push(data);
        }
        
        Ok(results)
    }
    
    match process_multiple_urls().await {
        Ok(results) => println!("    すべて成功: {:?}", results),
        Err(e) => println!("    処理中にエラー: {}", e),
    }
    
    // リトライロジック
    println!("\n  リトライロジック:");
    
    async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, AppError> {
        let mut retries = 0;
        
        loop {
            match fetch_data(url).await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    retries += 1;
                    if retries > max_retries {
                        println!("    リトライ回数超過");
                        return Err(e);
                    }
                    println!("    リトライ {} 回目...", retries);
                    sleep(Duration::from_millis(100 * retries as u64)).await;
                }
            }
        }
    }
    
    match fetch_with_retry("https://error.com", 3).await {
        Ok(data) => println!("    最終的に成功: {}", data),
        Err(e) => println!("    最終的に失敗: {}", e),
    }
}

// タイムアウトパターン
async fn timeout_patterns() {
    println!("\n--- タイムアウトパターン ---");
    
    // 基本的なタイムアウト
    println!("  基本的なタイムアウト:");
    
    async fn slow_operation() -> String {
        sleep(Duration::from_secs(2)).await;
        "完了".to_string()
    }
    
    match timeout(Duration::from_millis(100), slow_operation()).await {
        Ok(result) => println!("    成功: {}", result),
        Err(_) => println!("    タイムアウト！"),
    }
    
    // カスタムタイムアウトエラー
    println!("\n  カスタムタイムアウト処理:");
    
    #[derive(Debug)]
    enum OperationError {
        Timeout,
        Failed(String),
    }
    
    async fn operation_with_timeout() -> Result<String, OperationError> {
        match timeout(Duration::from_millis(200), slow_operation()).await {
            Ok(result) => Ok(result),
            Err(_) => Err(OperationError::Timeout),
        }
    }
    
    match operation_with_timeout().await {
        Ok(result) => println!("    成功: {}", result),
        Err(OperationError::Timeout) => println!("    操作タイムアウト"),
        Err(OperationError::Failed(msg)) => println!("    操作失敗: {}", msg),
    }
    
    // 複数操作のタイムアウト
    println!("\n  複数操作のタイムアウト:");
    
    async fn multi_step_operation() -> Result<String, OperationError> {
        // 各ステップに個別のタイムアウト
        let step1 = timeout(Duration::from_millis(100), async {
            sleep(Duration::from_millis(50)).await;
            "ステップ1完了"
        }).await.map_err(|_| OperationError::Timeout)?;
        
        println!("    {}", step1);
        
        let step2 = timeout(Duration::from_millis(100), async {
            sleep(Duration::from_millis(50)).await;
            "ステップ2完了"
        }).await.map_err(|_| OperationError::Timeout)?;
        
        println!("    {}", step2);
        
        Ok("全ステップ完了".to_string())
    }
    
    match multi_step_operation().await {
        Ok(result) => println!("    {}", result),
        Err(e) => println!("    エラー: {:?}", e),
    }
}

// キャンセレーションパターン
async fn cancellation_patterns() {
    println!("\n--- キャンセレーションパターン ---");
    
    // oneshotチャンネルによるキャンセル
    println!("  oneshotチャンネル:");
    
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
    
    let task = tokio::spawn(async move {
        select! {
            _ = async {
                for i in 0..u32::MAX {
                    println!("    作業中: {}", i);
                    sleep(Duration::from_millis(100)).await;
                }
            } => {
                println!("    タスク完了");
            }
            _ = cancel_rx => {
                println!("    タスクキャンセル受信");
            }
        }
    });
    
    sleep(Duration::from_millis(350)).await;
    cancel_tx.send(()).unwrap();
    task.await.unwrap();
    
    // AtomicBoolによるキャンセル
    println!("\n  AtomicBool:");
    
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = cancelled.clone();
    
    let task = tokio::spawn(async move {
        let mut i = 0;
        while !cancelled_clone.load(Ordering::Relaxed) {
            println!("    処理中: {}", i);
            sleep(Duration::from_millis(100)).await;
            i += 1;
        }
        println!("    キャンセルフラグ検出");
    });
    
    sleep(Duration::from_millis(350)).await;
    cancelled.store(true, Ordering::Relaxed);
    task.await.unwrap();
    
    // CancellationTokenパターン
    println!("\n  CancellationTokenパターン:");
    
    struct CancellationToken {
        cancelled: Arc<AtomicBool>,
        notify: Arc<tokio::sync::Notify>,
    }
    
    impl CancellationToken {
        fn new() -> Self {
            Self {
                cancelled: Arc::new(AtomicBool::new(false)),
                notify: Arc::new(tokio::sync::Notify::new()),
            }
        }
        
        fn cancel(&self) {
            self.cancelled.store(true, Ordering::SeqCst);
            self.notify.notify_waiters();
        }
        
        async fn cancelled(&self) {
            self.notify.notified().await;
        }
        
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }
        
        fn child(&self) -> Self {
            Self {
                cancelled: self.cancelled.clone(),
                notify: self.notify.clone(),
            }
        }
    }
    
    let token = CancellationToken::new();
    let child_token = token.child();
    
    let task = tokio::spawn(async move {
        select! {
            _ = async {
                for i in 0..u32::MAX {
                    if child_token.is_cancelled() {
                        break;
                    }
                    println!("    トークン作業: {}", i);
                    sleep(Duration::from_millis(100)).await;
                }
            } => {
                println!("    トークンタスク完了");
            }
            _ = child_token.cancelled() => {
                println!("    トークンキャンセル通知");
            }
        }
    });
    
    sleep(Duration::from_millis(350)).await;
    token.cancel();
    task.await.unwrap();
}

// グレースフルシャットダウン
async fn graceful_shutdown() {
    println!("\n--- グレースフルシャットダウン ---");
    
    // アプリケーション状態
    struct AppState {
        shutdown: CancellationToken,
        active_tasks: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
    }
    
    struct CancellationToken {
        cancelled: Arc<AtomicBool>,
        notify: Arc<tokio::sync::Notify>,
    }
    
    impl CancellationToken {
        fn new() -> Self {
            Self {
                cancelled: Arc::new(AtomicBool::new(false)),
                notify: Arc::new(tokio::sync::Notify::new()),
            }
        }
        
        fn cancel(&self) {
            self.cancelled.store(true, Ordering::SeqCst);
            self.notify.notify_waiters();
        }
        
        async fn cancelled(&self) {
            self.notify.notified().await;
        }
    }
    
    let app_state = Arc::new(AppState {
        shutdown: CancellationToken::new(),
        active_tasks: Arc::new(Mutex::new(Vec::new())),
    });
    
    // ワーカータスクの起動
    println!("  ワーカータスクを起動:");
    
    for i in 0..3 {
        let worker_state = app_state.clone();
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(200));
            
            loop {
                select! {
                    _ = interval.tick() => {
                        println!("    ワーカー{}: 作業中", i);
                    }
                    _ = worker_state.shutdown.cancelled() => {
                        println!("    ワーカー{}: シャットダウン開始", i);
                        // クリーンアップ処理
                        sleep(Duration::from_millis(100)).await;
                        println!("    ワーカー{}: クリーンアップ完了", i);
                        break;
                    }
                }
            }
        });
        
        app_state.active_tasks.lock().await.push(task);
    }
    
    // しばらく実行
    sleep(Duration::from_millis(500)).await;
    
    // シャットダウン開始
    println!("\n  シャットダウンを開始:");
    app_state.shutdown.cancel();
    
    // すべてのタスクの完了を待つ
    let tasks = {
        let mut tasks = app_state.active_tasks.lock().await;
        std::mem::take(&mut *tasks)
    };
    
    for task in tasks {
        task.await.unwrap();
    }
    
    println!("  すべてのタスクが正常に終了しました");
}

// エラーの伝播
async fn error_propagation() {
    println!("\n--- エラーの伝播 ---");
    
    use tokio::sync::mpsc;
    
    // パイプラインでのエラー処理
    println!("  パイプラインでのエラー処理:");
    
    #[derive(Debug)]
    enum PipelineError {
        Input(String),
        Processing(String),
        Output(String),
    }
    
    let (input_tx, mut input_rx) = mpsc::channel::<Result<i32, PipelineError>>(10);
    let (output_tx, mut output_rx) = mpsc::channel::<Result<i32, PipelineError>>(10);
    
    // 処理ステージ
    tokio::spawn(async move {
        while let Some(result) = input_rx.recv().await {
            let output = match result {
                Ok(value) => {
                    if value < 0 {
                        Err(PipelineError::Processing("負の値は処理できません".to_string()))
                    } else {
                        Ok(value * 2)
                    }
                }
                Err(e) => Err(e),
            };
            
            output_tx.send(output).await.unwrap();
        }
    });
    
    // データ送信
    let sender = tokio::spawn(async move {
        for i in -1..3 {
            let result = if i == -1 {
                Err(PipelineError::Input("無効な入力".to_string()))
            } else {
                Ok(i)
            };
            input_tx.send(result).await.unwrap();
        }
    });
    
    // 結果受信
    let receiver = tokio::spawn(async move {
        while let Some(result) = output_rx.recv().await {
            match result {
                Ok(value) => println!("    成功: {}", value),
                Err(PipelineError::Input(msg)) => println!("    入力エラー: {}", msg),
                Err(PipelineError::Processing(msg)) => println!("    処理エラー: {}", msg),
                Err(PipelineError::Output(msg)) => println!("    出力エラー: {}", msg),
            }
        }
    });
    
    // input_tx は sender タスクに move 済みで、タスク終了時の drop でチャンネルが閉じる
    sender.await.unwrap();
    receiver.await.unwrap();
    
    // エラーリカバリー
    println!("\n  エラーリカバリー:");
    
    async fn unreliable_service() -> Result<String, &'static str> {
        static COUNTER: AtomicBool = AtomicBool::new(false);
        
        if COUNTER.fetch_xor(true, Ordering::SeqCst) {
            Ok("成功".to_string())
        } else {
            Err("一時的な障害")
        }
    }
    
    async fn resilient_operation() -> String {
        let mut failures = 0;
        
        loop {
            match unreliable_service().await {
                Ok(result) => {
                    if failures > 0 {
                        println!("    {}回の失敗後に成功", failures);
                    }
                    return result;
                }
                Err(e) => {
                    failures += 1;
                    println!("    エラー: {} (試行 {})", e, failures);
                    
                    if failures >= 5 {
                        return "最大リトライ回数に達しました".to_string();
                    }
                    
                    // exponential backoff
                    let delay = Duration::from_millis(100 * 2u64.pow(failures - 1));
                    sleep(delay).await;
                }
            }
        }
    }
    
    let result = resilient_operation().await;
    println!("    最終結果: {}", result);
}

use tokio::sync::Mutex;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    // 各デモが完走すること（Pending のまま wake されないなどのハングも検出する）
    #[tokio::test]
    async fn shutdown_and_propagation_demos_complete() {
        tokio::time::timeout(Duration::from_secs(30), super::graceful_shutdown())
            .await
            .expect("graceful_shutdown がハングした");
        tokio::time::timeout(Duration::from_secs(30), super::error_propagation())
            .await
            .expect("error_propagation がハングした");
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
