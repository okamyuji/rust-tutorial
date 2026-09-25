# 第4章：非同期プログラミングとFuture

## 概要

Rustの非同期プログラミングを使うと、データ競合を防ぎながら多数のI/O処理を並行して進められます。本章では、async/await構文、Futureトレイト、Pin、非同期ランタイムの選択など、非同期プログラミングの深い理解を目指します。

## 4.1 async/awaitの内部動作

### 非同期関数の変換

コンパイラはasync関数をステートマシンに変換します。変換のイメージは次のとおりです。

```rust
// この関数は...
async fn fetch_data() -> Result<String, Error> {
    let response = fetch_url("https://example.com").await?;
    Ok(response)
}

// コンパイラによって以下のような構造に変換される
enum FetchDataFuture {
    Start,
    AwaitingFetch(FetchUrlFuture),
    Done,
}
```

### async/awaitの基本

```rust
async fn async_main() {
    let result = fetch_data().await;
    match result {
        Ok(data) => println!("Data: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 実行例

```bash
cargo run --bin async_await_internals
```

## 4.2 Futureトレイトとピン留め（Pinning）

### Futureトレイトの定義

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

### Pinの必要性

Pinは、自己参照構造体をメモリ上で動かさないことを保証する仕組みです。例を次に示します。

```rust
use std::pin::Pin;

struct SelfReferential {
    data: String,
    ptr: *const String,  // dataを指す
}

// Pinによって移動が防がれる
let pinned = Pin::new(Box::new(SelfReferential::new()));
```

### 実行例

```bash
cargo run --bin future_and_pin
```

## 4.3 非同期トレイトの現状と回避策

### ネイティブのasync fn in trait

Rust 1.75から、トレイトのメソッドに`async fn`を直接書けます。詳細は[Rust公式ブログの告知](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/)にあります。

```rust
trait AsyncTrait {
    async fn method(&self) -> Result<(), Error>;
}
```

ただし、このトレイトには制限が残っています。まず、`dyn AsyncTrait`として動的ディスパッチに使えません。また、呼び出し側は返されるFutureに`Send`境界を要求できません。`Send`が必要な場合は、`-> impl Future + Send`の形でメソッドを宣言します。公開トレイトでは[trait-variant](https://crates.io/crates/trait-variant)クレートを使う方法もあります。

```rust
use std::future::Future;

trait SendAsyncTrait {
    fn method(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
```

### async-traitクレートの使用

動的ディスパッチが必要な場合や、Rust 1.75より前の版を支える場合は、async-traitクレートを使います。

```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncProcessor {
    async fn process(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
}
```

### 実行例

```bash
cargo run --bin async_traits
```

## 4.4 タスクスポーンとエグゼキュータの選択

### 主要な非同期ランタイム

- Tokio 最も広く使われており、機能が豊富です。
- async-std 標準ライブラリに似たAPIを提供します。
- smol 軽量で、依存が少ない構成です。
- actix アクターモデルに基づいています。

### タスクのスポーン

```rust
use tokio::task;

async fn parallel_processing() {
    let task1 = task::spawn(async { process_data(1).await });
    let task2 = task::spawn(async { process_data(2).await });
    
    let (result1, result2) = tokio::join!(task1, task2);
}
```

### 実行例

```bash
cargo run --bin task_executors
```

## 4.5 非同期コードにおけるライフタイムの課題

### 借用チェッカーとの戦い

```rust
// 問題のあるコード
async fn process<'a>(data: &'a str) -> &'a str {
    tokio::time::sleep(Duration::from_secs(1)).await;
    data  // 借用が.awaitを跨ぐ
}

// 解決策：所有権を取る
async fn process_owned(data: String) -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    data
}
```

### 実行例

```bash
cargo run --bin async_lifetimes
```

## 高度な非同期パターン

### 4.6 ストリームと非同期イテレータ

```rust
use futures::stream::{Stream, StreamExt};

async fn process_stream<S>(stream: S) 
where
    S: Stream<Item = i32>,
{
    pin_mut!(stream);
    
    while let Some(value) = stream.next().await {
        println!("Received: {}", value);
    }
}
```

### 実行例

```bash
cargo run --bin async_streams
```

### 4.7 並行性と並列性

```rust
// 並行実行（Concurrent）
async fn concurrent_tasks() {
    let (a, b, c) = tokio::join!(
        fetch_data_a(),
        fetch_data_b(),
        fetch_data_c()
    );
}

// 並列実行（Parallel）
async fn parallel_computation() {
    let handles: Vec<_> = (0..num_cpus::get())
        .map(|i| {
            tokio::task::spawn_blocking(move || {
                heavy_computation(i)
            })
        })
        .collect();
}
```

### 実行例

```bash
cargo run --bin concurrency_patterns
```

### 4.8 エラーハンドリングとキャンセレーション

```rust
use tokio::select;
use tokio::time::{timeout, Duration};

async fn with_timeout() -> Result<String, Error> {
    match timeout(Duration::from_secs(5), fetch_data()).await {
        Ok(Ok(data)) => Ok(data),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(Error::Timeout),
    }
}

async fn cancellable_operation() {
    select! {
        result = long_running_task() => {
            println!("Task completed: {:?}", result);
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Operation cancelled");
        }
    }
}
```

### 実行例

```bash
cargo run --bin error_cancellation
```

### 4.9 非同期プログラミングのパフォーマンス

```rust
// ゼロコピー非同期I/O
use tokio::io::{AsyncRead, AsyncWrite};

async fn zero_copy_transfer<R, W>(reader: &mut R, writer: &mut W) 
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    tokio::io::copy(reader, writer).await?;
}

// バッファリングとバッチ処理
async fn batched_processing<T>(receiver: mpsc::Receiver<T>) {
    let mut batch = Vec::with_capacity(100);
    
    while let Some(item) = receiver.recv().await {
        batch.push(item);
        
        if batch.len() >= 100 {
            process_batch(&batch).await;
            batch.clear();
        }
    }
}
```

## 復習問題

### 問題1：Futureの実装

独自のタイマーFutureを実装してください。満たす要件は次のとおりです。

- 指定時間後に完了する
- Pendingを返すときにWakerを登録し、完了時にwakeする
- Pinの安全性を保証する

### 問題2：非同期トレイト

非同期メソッドを持つトレイトを実装してください。

- データベース接続の抽象化
- エラーハンドリング
- タイムアウト機能

### 問題3：並行タスク管理

複数の非同期タスクを管理するシステムを実装してください。

- タスクの動的追加・削除
- 進捗状況の監視
- グレースフルシャットダウン

### 問題4：ストリーム処理

非同期ストリームを処理するパイプラインを実装してください。

- フィルタリング
- 変換
- バッファリング
- エラーハンドリング

### 問題5：実装課題

WebSocketサーバーの簡単な実装を作成してください。満たす要件は次のとおりです。

- 複数クライアントの同時接続
- メッセージのブロードキャスト
- 接続管理
- エラーリカバリー

## 模範解答

### 問題1の解答: 独自のタイマーFutureの実装

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

// タイマーの状態を管理する構造体
struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

// カスタムタイマーFuture
pub struct Timer {
    state: Arc<Mutex<TimerState>>,
    duration: Duration,
    start_time: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));

        let timer_state = Arc::clone(&state);
        let timer_duration = duration;

        // バックグラウンドスレッドでタイマーを実行
        thread::spawn(move || {
            thread::sleep(timer_duration);
            
            let mut state = timer_state.lock().unwrap();
            state.completed = true;
            
            // Wakerが登録されていれば呼び出し
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        Timer {
            state,
            duration,
            start_time: Instant::now(),
        }
    }
    
    // 残り時間を取得
    pub fn remaining(&self) -> Duration {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            Duration::from_secs(0)
        } else {
            self.duration - elapsed
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        if state.completed {
            Poll::Ready(())
        } else {
            // Wakerを登録して、タイマー完了時に起こしてもらう
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// より高度なタイマー：キャンセル可能
pub struct CancellableTimer {
    state: Arc<Mutex<TimerState>>,
    cancel_handle: Arc<Mutex<bool>>,
}

impl CancellableTimer {
    pub fn new(duration: Duration) -> (Self, CancelHandle) {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));
        
        let cancel_flag = Arc::new(Mutex::new(false));
        let timer_state = Arc::clone(&state);
        let timer_cancel = Arc::clone(&cancel_flag);

        thread::spawn(move || {
            let sleep_duration = Duration::from_millis(10);
            let mut elapsed = Duration::from_secs(0);
            
            while elapsed < duration {
                thread::sleep(sleep_duration);
                elapsed += sleep_duration;
                
                // キャンセルチェック
                if *timer_cancel.lock().unwrap() {
                    return;
                }
            }
            
            let mut state = timer_state.lock().unwrap();
            state.completed = true;
            
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        let timer = CancellableTimer {
            state,
            cancel_handle: Arc::clone(&cancel_flag),
        };
        
        let cancel_handle = CancelHandle { 
            cancel_flag 
        };

        (timer, cancel_handle)
    }
}

impl Future for CancellableTimer {
    type Output = Result<(), TimerCancelled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        // キャンセルされたかチェック
        if *self.cancel_handle.lock().unwrap() {
            return Poll::Ready(Err(TimerCancelled));
        }
        
        if state.completed {
            Poll::Ready(Ok(()))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[derive(Debug)]
pub struct TimerCancelled;

pub struct CancelHandle {
    cancel_flag: Arc<Mutex<bool>>,
}

impl CancelHandle {
    pub fn cancel(&self) {
        *self.cancel_flag.lock().unwrap() = true;
    }
}

// 使用例
#[tokio::main]
async fn main() {
    println!("Starting timer...");
    
    // 基本タイマー
    let timer = Timer::new(Duration::from_secs(2));
    timer.await;
    println!("Timer completed!");
    
    // キャンセル可能タイマー
    let (timer, cancel_handle) = CancellableTimer::new(Duration::from_secs(5));
    
    // 別のタスクでキャンセル
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        cancel_handle.cancel();
        println!("Timer cancelled!");
    });
    
    match timer.await {
        Ok(()) => println!("Timer completed normally"),
        Err(TimerCancelled) => println!("Timer was cancelled"),
    }
}
```

この実装の要点は次のとおりです。

- `Timer`構造体は、Pendingを返すときにWakerを保存し、完了時にwakeしてポーリングを再開させる
- `Pin`の安全性は、構造体に自己参照がないため自動的に保証
- キャンセル可能なバージョンも提供し、実用的な機能を追加

### 問題2の解答: 非同期トレイトの実装

```rust
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::timeout;
use std::collections::HashMap;

// データベース接続の抽象化
#[async_trait]
pub trait DatabaseConnection: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    async fn connect(&mut self) -> Result<(), Self::Error>;
    async fn disconnect(&mut self) -> Result<(), Self::Error>;
    async fn query(&self, sql: &str) -> Result<Vec<Row>, Self::Error>;
    async fn execute(&self, sql: &str) -> Result<u64, Self::Error>;
    async fn transaction<F, R>(&self, f: F) -> Result<R, Self::Error>
    where
        F: for<'a> FnOnce(&'a mut Transaction) -> Pin<Box<dyn Future<Output = Result<R, Self::Error>> + Send + 'a>> + Send,
        R: Send;
}

// 行データの表現
#[derive(Debug, Clone)]
pub struct Row {
    pub columns: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Text(String),
    Real(f64),
    Blob(Vec<u8>),
}

// トランザクション
pub struct Transaction {
    committed: bool,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction { committed: false }
    }
    
    pub fn commit(&mut self) {
        self.committed = true;
    }
}

// エラー型
#[derive(Debug)]
pub enum DbError {
    ConnectionFailed(String),
    QueryError(String),
    Timeout,
    TransactionFailed(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DbError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DbError::Timeout => write!(f, "Operation timed out"),
            DbError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

// 具体的な実装例（SQLite風）
pub struct SqliteConnection {
    connected: bool,
    connection_string: String,
}

impl SqliteConnection {
    pub fn new(connection_string: String) -> Self {
        SqliteConnection {
            connected: false,
            connection_string,
        }
    }
}

#[async_trait]
impl DatabaseConnection for SqliteConnection {
    type Error = DbError;
    
    async fn connect(&mut self) -> Result<(), Self::Error> {
        // タイムアウト付きで接続
        match timeout(Duration::from_secs(10), async {
            // 実際のデータベース接続処理をシミュレート
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            if self.connection_string.is_empty() {
                return Err(DbError::ConnectionFailed("Empty connection string".to_string()));
            }
            
            self.connected = true;
            println!("Connected to database: {}", self.connection_string);
            Ok(())
        }).await {
            Ok(result) => result,
            Err(_) => Err(DbError::Timeout),
        }
    }
    
    async fn disconnect(&mut self) -> Result<(), Self::Error> {
        if !self.connected {
            return Ok(());
        }
        
        // 切断処理をシミュレート
        tokio::time::sleep(Duration::from_millis(50)).await;
        self.connected = false;
        println!("Disconnected from database");
        Ok(())
    }
    
    async fn query(&self, sql: &str) -> Result<Vec<Row>, Self::Error> {
        if !self.connected {
            return Err(DbError::QueryError("Not connected".to_string()));
        }
        
        // タイムアウト付きクエリ実行
        match timeout(Duration::from_secs(30), async {
            // クエリ実行をシミュレート
            tokio::time::sleep(Duration::from_millis(200)).await;
            
            if sql.trim().is_empty() {
                return Err(DbError::QueryError("Empty query".to_string()));
            }
            
            // サンプルデータを返す
            let mut row = Row {
                columns: HashMap::new(),
            };
            row.columns.insert("id".to_string(), Value::Integer(1));
            row.columns.insert("name".to_string(), Value::Text("Sample".to_string()));
            
            println!("Executed query: {}", sql);
            Ok(vec![row])
        }).await {
            Ok(result) => result,
            Err(_) => Err(DbError::Timeout),
        }
    }
    
    async fn execute(&self, sql: &str) -> Result<u64, Self::Error> {
        if !self.connected {
            return Err(DbError::QueryError("Not connected".to_string()));
        }
        
        // タイムアウト付き実行
        match timeout(Duration::from_secs(30), async {
            tokio::time::sleep(Duration::from_millis(150)).await;
            
            if sql.trim().is_empty() {
                return Err(DbError::QueryError("Empty statement".to_string()));
            }
            
            println!("Executed statement: {}", sql);
            // 影響を受けた行数を返す（模擬）
            Ok(1)
        }).await {
            Ok(result) => result,
            Err(_) => Err(DbError::Timeout),
        }
    }
    
    async fn transaction<F, R>(&self, f: F) -> Result<R, Self::Error>
    where
        F: for<'a> FnOnce(&'a mut Transaction) -> Pin<Box<dyn Future<Output = Result<R, Self::Error>> + Send + 'a>> + Send,
        R: Send,
    {
        if !self.connected {
            return Err(DbError::TransactionFailed("Not connected".to_string()));
        }
        
        println!("Starting transaction");
        let mut tx = Transaction::new();
        
        match f(&mut tx).await {
            Ok(result) => {
                if tx.committed {
                    println!("Transaction committed");
                } else {
                    println!("Transaction auto-committed");
                }
                Ok(result)
            }
            Err(e) => {
                println!("Transaction rolled back");
                Err(DbError::TransactionFailed(format!("Transaction rolled back: {}", e)))
            }
        }
    }
}

// 使用例
#[tokio::main]
async fn main() -> Result<(), DbError> {
    let mut conn = SqliteConnection::new("sqlite:memory:".to_string());
    
    // 接続
    conn.connect().await?;
    
    // クエリ実行
    let rows = conn.query("SELECT * FROM users").await?;
    println!("Query result: {:?}", rows);
    
    // 更新実行
    let affected = conn.execute("INSERT INTO users (name) VALUES ('Alice')").await?;
    println!("Affected rows: {}", affected);
    
    // トランザクション
    let result = conn.transaction(|tx| {
        Box::pin(async move {
            // トランザクション内の処理
            tx.commit();
            Ok(42)
        })
    }).await?;
    
    println!("Transaction result: {}", result);
    
    // 切断
    conn.disconnect().await?;
    
    Ok(())
}
```

この設計のポイントは次のとおりです。

- `async_trait`を使用して非同期トレイトメソッドを実装
- タイムアウト機能を全ての操作に統合
- エラーハンドリングを包括的に実装
- トランザクション処理の高度な抽象化

### 問題3の解答: 並行タスク管理システム

実装全体は長いため、ここでは概要だけを示します。

```rust
// タスクマネージャーの主要機能
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<TaskId, TaskInfo>>>,
    next_id: Arc<RwLock<TaskId>>,
    shutdown_notify: Arc<Notify>,
}

// 主要メソッド:
// - spawn_task(): 非同期タスクの動的追加
// - cancel_task(): 個別タスクのキャンセル
// - get_progress(): リアルタイム進捗監視
// - shutdown(): グレースフルシャットダウン
```

この設計のポイントは次のとおりです。

- 動的なタスク追加・削除機能
- 進捗状況のリアルタイム監視
- タイムアウト付きグレースフルシャットダウン
- キャンセレーション機能
- エラー回復とリトライ機構

### 問題4の解答: 非同期ストリーム処理パイプライン

```rust
// ストリーム処理の基本トレイト
pub trait StreamProcessor<I, O>: Send + 'static {
    fn process(&mut self, item: I) -> impl Future<Output = ProcessingResult<O>> + Send;
}

// パイプライン処理システム
pub struct ProcessingPipeline {
    buffer_size: usize,
    error_threshold: usize,
    retry_count: usize,
}

// 主要機能:
// - フィルタリング（条件に基づく要素除外）
// - 変換処理（型変換と値の変更）
// - バッファリング（効率的なバッチ処理）
// - エラーハンドリング（リトライとエラー閾値）
```

この設計の特徴は次のとおりです。

- フィルタリング 条件に基づいて要素を除外します。
- 変換処理 型の変換と値の変更を行います。
- バッファリング 要素をまとめてバッチで処理します。
- エラーハンドリング リトライ機構とエラーの閾値を設定できます。
- 並行処理 バッチ内の要素を並列に実行します。

### 問題5の解答: WebSocketサーバーの実装

```rust
// WebSocketサーバーの主要構造
pub struct WebSocketServer {
    clients: Arc<RwLock<HashMap<ClientId, ClientHandle>>>,
    next_client_id: Arc<RwLock<ClientId>>,
    broadcast_sender: mpsc::UnboundedSender<ServerMessage>,
    shutdown_notify: Arc<Notify>,
}

// 主要機能:
// - handle_client(): 個別クライアント接続管理
// - broadcast_message(): 全クライアントへのメッセージ配信
// - send_to_client(): 特定クライアントへの直接送信
// - shutdown(): グレースフルな全接続終了
```

この設計の特徴は次のとおりです。

1. 並行接続管理 複数のクライアントに同時に対応します。
2. メッセージブロードキャスト 全クライアントへメッセージを配信します。
3. 接続ライフサイクル管理 接続と切断を自動で処理します。
4. エラーリカバリー 1つの接続のエラーを他の接続から切り離します。
5. グレースフルシャットダウン 全接続を安全に終了させます。
6. リアルタイム通信 WebSocketプロトコルで双方向に通信します。

この章の解答から学べる点は次のとおりです。

- 各実装は実際のプロダクション環境で使用可能なレベルの品質
- 非同期プログラミングの重要パターンを網羅的に実装
- エラーハンドリング、リソース管理、パフォーマンス最適化を考慮
- Rustの所有権システムと非同期処理の組み合わせ方を実例で示す

## まとめ

非同期プログラミングは、I/Oバウンドなタスクを効率的に処理するための強力な手法です。Futureトレイト、async/await構文、Pinの仕組みを理解すると、安全で高速な非同期コードを書けます。実践では、用途に合うランタイムを選び、awaitをまたぐ借用の問題に対処する必要があります。
