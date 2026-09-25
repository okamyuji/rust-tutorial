// src/bin/async_traits.rs

use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== 非同期トレイトの現状と回避策 ===\n");
    
    trait_limitations().await;
    manual_future_return().await;
    async_trait_macro().await;
    generic_async_traits().await;
    future_directions();
}

// Rust 1.75 以降はトレイトに async fn を直接書ける
trait AsyncTrait {
    async fn async_method(&self) -> String;
}

struct NativeImpl;

impl AsyncTrait for NativeImpl {
    async fn async_method(&self) -> String {
        "ネイティブの async fn in trait".to_string()
    }
}

// トレイトの制限事項
async fn trait_limitations() {
    println!("--- 現在の制限事項 ---");
    
    println!("  {}", NativeImpl.async_method().await);
    println!("  Rust 1.75以降のasync fn in traitに残る制限:");
    println!("  1. dyn AsyncTraitとして使えない（動的ディスパッチ不可）");
    println!("  2. 返されるFutureにSend境界を付けられない");
    println!("     （-> impl Future + Send と書くか、trait-variantクレートを使う）");
    println!();
    
    // 回避策1: Futureを明示的に返す
    trait ManualAsyncTrait {
        fn async_method(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
    }
    
    struct MyStruct {
        data: String,
    }
    
    impl ManualAsyncTrait for MyStruct {
        fn async_method(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
            Box::pin(async move {
                sleep(Duration::from_millis(100)).await;
                format!("Manual async: {}", self.data)
            })
        }
    }
    
    println!("  回避策1: Box<dyn Future>を手動で返す");
    println!("  - 利点: 完全な制御");
    println!("  - 欠点: 冗長、ヒープアロケーション");
}

// 手動でFutureを返す方法
async fn manual_future_return() {
    println!("\n--- 手動でFutureを返す ---");
    
    // データベース接続の抽象化
    trait Database {
        fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Connection, Error>> + Send + '_>>;
        fn query(&self, conn: &Connection, sql: &str) 
            -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>>;
    }
    
    // 具体的な実装
    struct PostgresDB {
        host: String,
    }
    
    impl Database for PostgresDB {
        fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Connection, Error>> + Send + '_>> {
            let host = self.host.clone();
            Box::pin(async move {
                println!("  PostgreSQL {}に接続中...", host);
                sleep(Duration::from_millis(100)).await;
                Ok(Connection { id: 1 })
            })
        }
        
        fn query(&self, conn: &Connection, sql: &str) 
            -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>> {
            let conn_id = conn.id;
            let sql = sql.to_string();
            Box::pin(async move {
                println!("  接続{}でクエリ実行: {}", conn_id, sql);
                sleep(Duration::from_millis(50)).await;
                Ok(vec![
                    Row { data: "行1".to_string() },
                    Row { data: "行2".to_string() },
                ])
            })
        }
    }
    
    // 使用例
    let db = PostgresDB { host: "localhost".to_string() };
    
    match db.connect().await {
        Ok(conn) => {
            match db.query(&conn, "SELECT * FROM users").await {
                Ok(rows) => {
                    for row in rows {
                        println!("    {}", row.data);
                    }
                }
                Err(e) => println!("  クエリエラー: {}", e),
            }
        }
        Err(e) => println!("  接続エラー: {}", e),
    }
}

// async-traitマクロの使用
async fn async_trait_macro() {
    println!("\n--- async-traitマクロ ---");
    
    // async-traitを使った定義
    #[async_trait]
    trait AsyncProcessor {
        async fn initialize(&mut self) -> Result<(), Error>;
        async fn process(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
        async fn shutdown(self) -> Result<(), Error>;
    }
    
    // 実装
    struct DataProcessor {
        name: String,
        initialized: bool,
    }
    
    #[async_trait]
    impl AsyncProcessor for DataProcessor {
        async fn initialize(&mut self) -> Result<(), Error> {
            println!("  {} を初期化中...", self.name);
            sleep(Duration::from_millis(100)).await;
            self.initialized = true;
            Ok(())
        }
        
        async fn process(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
            if !self.initialized {
                return Err(Error::NotInitialized);
            }
            
            println!("  {} でデータ処理中 ({}バイト)", self.name, data.len());
            sleep(Duration::from_millis(50)).await;
            
            // 簡単な変換
            Ok(data.iter().map(|&b| b.wrapping_add(1)).collect())
        }
        
        async fn shutdown(self) -> Result<(), Error> {
            println!("  {} をシャットダウン中...", self.name);
            sleep(Duration::from_millis(50)).await;
            Ok(())
        }
    }
    
    // 使用例
    let mut processor = DataProcessor {
        name: "プロセッサ1".to_string(),
        initialized: false,
    };
    
    processor.initialize().await.unwrap();
    
    let input = vec![1, 2, 3, 4, 5];
    match processor.process(&input).await {
        Ok(output) => println!("  処理結果: {:?}", output),
        Err(e) => println!("  処理エラー: {}", e),
    }
    
    processor.shutdown().await.unwrap();
}

// ジェネリックな非同期トレイト
async fn generic_async_traits() {
    println!("\n--- ジェネリックな非同期トレイト ---");
    
    // ジェネリックパラメータを持つ非同期トレイト
    #[async_trait]
    trait AsyncContainer<T: Send + Sync> {
        async fn insert(&mut self, item: T) -> Result<(), Error>;
        async fn get(&self, index: usize) -> Result<Option<T>, Error>;
        async fn remove(&mut self, index: usize) -> Result<Option<T>, Error>;
    }
    
    // 実装
    struct AsyncVec<T> {
        data: Vec<T>,
        name: String,
    }
    
    #[async_trait]
    impl<T: Send + Sync + Clone> AsyncContainer<T> for AsyncVec<T> {
        async fn insert(&mut self, item: T) -> Result<(), Error> {
            println!("  {} に要素を挿入中...", self.name);
            sleep(Duration::from_millis(10)).await;
            self.data.push(item);
            Ok(())
        }
        
        async fn get(&self, index: usize) -> Result<Option<T>, Error> {
            println!("  {} から要素{}を取得中...", self.name, index);
            sleep(Duration::from_millis(10)).await;
            Ok(self.data.get(index).cloned())
        }
        
        async fn remove(&mut self, index: usize) -> Result<Option<T>, Error> {
            println!("  {} から要素{}を削除中...", self.name, index);
            sleep(Duration::from_millis(10)).await;
            
            if index < self.data.len() {
                Ok(Some(self.data.remove(index)))
            } else {
                Ok(None)
            }
        }
    }
    
    // 使用例
    let mut vec = AsyncVec {
        data: Vec::new(),
        name: "非同期ベクタ".to_string(),
    };
    
    vec.insert("要素1".to_string()).await.unwrap();
    vec.insert("要素2".to_string()).await.unwrap();
    vec.insert("要素3".to_string()).await.unwrap();
    
    if let Ok(Some(item)) = vec.get(1).await {
        println!("  取得した要素: {}", item);
    }
    
    if let Ok(Some(removed)) = vec.remove(0).await {
        println!("  削除した要素: {}", removed);
    }
    
    // トレイトオブジェクトとしての使用
    println!("\n  トレイトオブジェクトとして:");
    
    let container: Box<dyn AsyncContainer<String>> = Box::new(vec);
    // 注: async-traitはSendを自動的に追加しない
}

// 将来の方向性
fn future_directions() {
    println!("\n--- 将来の方向性 ---");
    
    println!("  1. async fn in traits (Rust 1.75で安定化済み)");
    println!("     - impl Trait in trait positionも同時に安定化");
    println!("     - dyn対応は未解決のため、動的ディスパッチにはasync-traitを使う");
    
    println!("\n  2. GAT (Generic Associated Types, Rust 1.65で安定化済み)");
    println!("     - より柔軟な非同期トレイトが可能に");
    
    // GATを使った例
    /*
    trait AsyncIterator {
        type Item;
        type NextFuture<'a>: Future<Output = Option<Self::Item>>
        where
            Self: 'a;
        
        fn next<'a>(&'a mut self) -> Self::NextFuture<'a>;
    }
    */
    
    println!("\n  3. 現在のベストプラクティス:");
    println!("     - 静的ディスパッチ: ネイティブのasync fn in trait");
    println!("     - 動的ディスパッチ: async-traitマクロ");
    println!("     - Send境界が必要な公開トレイト: trait-variantクレート");
    
    println!("\n  4. 代替アプローチ:");
    println!("     - アクターモデル (actix)");
    println!("     - チャンネルベースの通信");
    println!("     - 同期的なAPIでラップ");
}

// ヘルパー型
#[derive(Debug)]
struct Connection {
    id: u32,
}

#[derive(Debug)]
struct Row {
    data: String,
}

#[derive(Debug)]
enum Error {
    ConnectionFailed,
    QueryFailed,
    NotInitialized,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionFailed => write!(f, "接続失敗"),
            Error::QueryFailed => write!(f, "クエリ失敗"),
            Error::NotInitialized => write!(f, "未初期化"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn trait_limitations_runs_native_async_fn_in_trait() {
        super::trait_limitations().await;
        super::future_directions();
    }

    #[tokio::test]
    async fn native_async_fn_in_trait_returns_value() {
        use super::AsyncTrait;
        assert_eq!(super::NativeImpl.async_method().await, "ネイティブの async fn in trait");
    }
}
