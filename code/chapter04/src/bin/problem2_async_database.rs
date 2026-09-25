// src/bin/problem2_async_database.rs
// 復習問題2: 非同期トレイトの実装（データベース接続の抽象化）

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 復習問題2: 非同期データベーストレイト ===\n");

    // 基本的なデータベース操作
    basic_database_operations().await?;

    // 高度なデータベース機能
    advanced_database_features().await?;

    // エラーハンドリングとタイムアウト
    error_handling_demo().await?;

    // トランザクション処理
    transaction_demo().await?;

    // 接続プーリング
    connection_pooling_demo().await?;

    Ok(())
}

// データベース接続の抽象化トレイト
#[async_trait]
pub trait DatabaseConnection: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    // 基本的な接続管理
    async fn connect(&mut self) -> Result<(), Self::Error>;
    async fn disconnect(&mut self) -> Result<(), Self::Error>;
    async fn is_connected(&self) -> bool;
    async fn ping(&self) -> Result<Duration, Self::Error>;

    // データ操作
    async fn query(&self, sql: &str) -> Result<QueryResult, Self::Error>;
    async fn execute(&self, sql: &str) -> Result<ExecuteResult, Self::Error>;
    async fn prepare(
        &self,
        sql: &str,
    ) -> Result<Box<dyn PreparedStatement<Error = Self::Error>>, Self::Error>;

    // トランザクション
    async fn begin_transaction(
        &self,
    ) -> Result<Box<dyn Transaction<Error = Self::Error>>, Self::Error>;

    // 高度な機能
    async fn query_with_timeout(
        &self,
        sql: &str,
        timeout: Duration,
    ) -> Result<QueryResult, Self::Error>;
    async fn bulk_insert(&self, table: &str, rows: Vec<Row>) -> Result<ExecuteResult, Self::Error>;
    async fn get_table_info(&self, table_name: &str) -> Result<TableInfo, Self::Error>;
}

// プリペアドステートメント
#[async_trait]
pub trait PreparedStatement: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn bind_parameter(&mut self, index: usize, value: Value) -> Result<(), Self::Error>;
    async fn execute(&self) -> Result<ExecuteResult, Self::Error>;
    async fn query(&self) -> Result<QueryResult, Self::Error>;
    async fn clear_parameters(&mut self) -> Result<(), Self::Error>;
}

// トランザクション
#[async_trait]
pub trait Transaction: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn commit(self: Box<Self>) -> Result<(), Self::Error>;
    async fn rollback(self: Box<Self>) -> Result<(), Self::Error>;
    async fn query(&self, sql: &str) -> Result<QueryResult, Self::Error>;
    async fn execute(&self, sql: &str) -> Result<ExecuteResult, Self::Error>;
    async fn savepoint(&self, name: &str) -> Result<(), Self::Error>;
    async fn rollback_to_savepoint(&self, name: &str) -> Result<(), Self::Error>;
}

// データ型の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub columns: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
    DateTime(String),
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<Row>,
    pub columns: Vec<ColumnInfo>,
    pub execution_time: Duration,
}

#[derive(Debug, Clone)]
pub struct ExecuteResult {
    pub affected_rows: u64,
    pub last_insert_id: Option<i64>,
    pub execution_time: Duration,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub row_count: u64,
    pub size_bytes: u64,
}

// エラー型
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryError(String),
    Timeout,
    TransactionFailed(String),
    PreparedStatementError(String),
    AuthenticationFailed,
    SchemaError(String),
    ConstraintViolation(String),
    NetworkError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::Timeout => write!(f, "Operation timed out"),
            DatabaseError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            DatabaseError::PreparedStatementError(msg) => {
                write!(f, "Prepared statement error: {}", msg)
            }
            DatabaseError::AuthenticationFailed => write!(f, "Authentication failed"),
            DatabaseError::SchemaError(msg) => write!(f, "Schema error: {}", msg),
            DatabaseError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            DatabaseError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

// SQLiteの模擬実装
pub struct SqliteConnection {
    connection_string: String,
    connected: bool,
    data: Arc<RwLock<HashMap<String, Vec<Row>>>>,
    connection_start: Option<Instant>,
}

impl SqliteConnection {
    pub fn new(connection_string: String) -> Self {
        let mut initial_data = HashMap::new();

        // サンプルデータの初期化
        initial_data.insert(
            "users".to_string(),
            vec![
                Row {
                    columns: {
                        let mut cols = HashMap::new();
                        cols.insert("id".to_string(), Value::Integer(1));
                        cols.insert("name".to_string(), Value::Text("Alice".to_string()));
                        cols.insert(
                            "email".to_string(),
                            Value::Text("alice@example.com".to_string()),
                        );
                        cols.insert("age".to_string(), Value::Integer(30));
                        cols
                    },
                },
                Row {
                    columns: {
                        let mut cols = HashMap::new();
                        cols.insert("id".to_string(), Value::Integer(2));
                        cols.insert("name".to_string(), Value::Text("Bob".to_string()));
                        cols.insert(
                            "email".to_string(),
                            Value::Text("bob@example.com".to_string()),
                        );
                        cols.insert("age".to_string(), Value::Integer(25));
                        cols
                    },
                },
            ],
        );

        SqliteConnection {
            connection_string,
            connected: false,
            data: Arc::new(RwLock::new(initial_data)),
            connection_start: None,
        }
    }

    // SQLクエリの簡単なパース（実用的ではないが、デモ用）
    fn parse_select_query(&self, sql: &str) -> Result<QueryResult, DatabaseError> {
        let sql_lower = sql.to_lowercase();
        let start_time = Instant::now();

        if sql_lower.contains("select") && sql_lower.contains("from") {
            let data = self.data.read().unwrap();

            // 非常に簡単なテーブル名抽出
            if sql_lower.contains("users") {
                if let Some(users) = data.get("users") {
                    let columns = vec![
                        ColumnInfo {
                            name: "id".to_string(),
                            data_type: "INTEGER".to_string(),
                            nullable: false,
                        },
                        ColumnInfo {
                            name: "name".to_string(),
                            data_type: "TEXT".to_string(),
                            nullable: false,
                        },
                        ColumnInfo {
                            name: "email".to_string(),
                            data_type: "TEXT".to_string(),
                            nullable: false,
                        },
                        ColumnInfo {
                            name: "age".to_string(),
                            data_type: "INTEGER".to_string(),
                            nullable: true,
                        },
                    ];

                    return Ok(QueryResult {
                        rows: users.clone(),
                        columns,
                        execution_time: start_time.elapsed(),
                    });
                }
            }
        }

        Err(DatabaseError::QueryError(
            "Invalid or unsupported query".to_string(),
        ))
    }

    fn parse_insert_query(&self, sql: &str) -> Result<ExecuteResult, DatabaseError> {
        let start_time = Instant::now();

        if sql.to_lowercase().contains("insert into users") {
            // 簡単なINSERT文の処理（実際のパーサーではない）
            let mut data = self.data.write().unwrap();
            let users = data.get_mut("users").unwrap();

            let new_id = users.len() as i64 + 1;
            let new_row = Row {
                columns: {
                    let mut cols = HashMap::new();
                    cols.insert("id".to_string(), Value::Integer(new_id));
                    cols.insert("name".to_string(), Value::Text("NewUser".to_string()));
                    cols.insert(
                        "email".to_string(),
                        Value::Text("new@example.com".to_string()),
                    );
                    cols.insert("age".to_string(), Value::Integer(20));
                    cols
                },
            };

            users.push(new_row);

            return Ok(ExecuteResult {
                affected_rows: 1,
                last_insert_id: Some(new_id),
                execution_time: start_time.elapsed(),
            });
        }

        Err(DatabaseError::QueryError(
            "Invalid INSERT statement".to_string(),
        ))
    }
}

#[async_trait]
impl DatabaseConnection for SqliteConnection {
    type Error = DatabaseError;

    async fn connect(&mut self) -> Result<(), Self::Error> {
        // 接続処理をシミュレート
        match timeout(Duration::from_secs(10), async {
            tokio::time::sleep(Duration::from_millis(100)).await;

            if self.connection_string.is_empty() {
                return Err(DatabaseError::ConnectionFailed(
                    "Empty connection string".to_string(),
                ));
            }

            if self.connection_string.contains("invalid") {
                return Err(DatabaseError::AuthenticationFailed);
            }

            self.connected = true;
            self.connection_start = Some(Instant::now());
            println!("  ✓ Connected to SQLite: {}", self.connection_string);
            Ok(())
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(DatabaseError::Timeout),
        }
    }

    async fn disconnect(&mut self) -> Result<(), Self::Error> {
        if !self.connected {
            return Ok(());
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
        self.connected = false;
        self.connection_start = None;
        println!("  ✓ Disconnected from SQLite");
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    async fn ping(&self) -> Result<Duration, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::ConnectionFailed("Not connected".to_string()));
        }

        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(5)).await; // ネットワーク遅延をシミュレート
        Ok(start.elapsed())
    }

    async fn query(&self, sql: &str) -> Result<QueryResult, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::QueryError("Not connected".to_string()));
        }

        // クエリ実行をシミュレート
        tokio::time::sleep(Duration::from_millis(50)).await;

        self.parse_select_query(sql)
    }

    async fn execute(&self, sql: &str) -> Result<ExecuteResult, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::QueryError("Not connected".to_string()));
        }

        tokio::time::sleep(Duration::from_millis(30)).await;

        if sql.to_lowercase().contains("insert") {
            self.parse_insert_query(sql)
        } else if sql.to_lowercase().contains("update") {
            Ok(ExecuteResult {
                affected_rows: 1,
                last_insert_id: None,
                execution_time: Duration::from_millis(30),
            })
        } else if sql.to_lowercase().contains("delete") {
            Ok(ExecuteResult {
                affected_rows: 1,
                last_insert_id: None,
                execution_time: Duration::from_millis(25),
            })
        } else {
            Err(DatabaseError::QueryError(
                "Unsupported SQL statement".to_string(),
            ))
        }
    }

    async fn prepare(
        &self,
        sql: &str,
    ) -> Result<Box<dyn PreparedStatement<Error = Self::Error>>, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::PreparedStatementError(
                "Not connected".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(20)).await;

        Ok(Box::new(SqlitePreparedStatement::new(sql.to_string())))
    }

    async fn begin_transaction(
        &self,
    ) -> Result<Box<dyn Transaction<Error = Self::Error>>, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::TransactionFailed(
                "Not connected".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("  ✓ Transaction started");

        Ok(Box::new(SqliteTransaction::new()))
    }

    async fn query_with_timeout(
        &self,
        sql: &str,
        timeout_duration: Duration,
    ) -> Result<QueryResult, Self::Error> {
        match timeout(timeout_duration, self.query(sql)).await {
            Ok(result) => result,
            Err(_) => Err(DatabaseError::Timeout),
        }
    }

    async fn bulk_insert(&self, table: &str, rows: Vec<Row>) -> Result<ExecuteResult, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::QueryError("Not connected".to_string()));
        }

        // バルクインサートをシミュレート
        let start_time = Instant::now();
        tokio::time::sleep(Duration::from_millis(rows.len() as u64 * 5)).await;

        let mut data = self.data.write().unwrap();
        if let Some(table_data) = data.get_mut(table) {
            table_data.extend(rows.clone());
        }

        Ok(ExecuteResult {
            affected_rows: rows.len() as u64,
            last_insert_id: None,
            execution_time: start_time.elapsed(),
        })
    }

    async fn get_table_info(&self, table_name: &str) -> Result<TableInfo, Self::Error> {
        if !self.connected {
            return Err(DatabaseError::SchemaError("Not connected".to_string()));
        }

        tokio::time::sleep(Duration::from_millis(20)).await;

        let data = self.data.read().unwrap();
        if let Some(table_data) = data.get(table_name) {
            let columns = vec![
                ColumnInfo {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                },
                ColumnInfo {
                    name: "name".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: false,
                },
                ColumnInfo {
                    name: "email".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: false,
                },
                ColumnInfo {
                    name: "age".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: true,
                },
            ];

            Ok(TableInfo {
                name: table_name.to_string(),
                columns,
                row_count: table_data.len() as u64,
                size_bytes: table_data.len() as u64 * 100, // 概算
            })
        } else {
            Err(DatabaseError::SchemaError(format!(
                "Table '{}' not found",
                table_name
            )))
        }
    }
}

// プリペアドステートメントの実装
pub struct SqlitePreparedStatement {
    sql: String,
    parameters: HashMap<usize, Value>,
}

impl SqlitePreparedStatement {
    fn new(sql: String) -> Self {
        SqlitePreparedStatement {
            sql,
            parameters: HashMap::new(),
        }
    }
}

#[async_trait]
impl PreparedStatement for SqlitePreparedStatement {
    type Error = DatabaseError;

    async fn bind_parameter(&mut self, index: usize, value: Value) -> Result<(), Self::Error> {
        self.parameters.insert(index, value);
        Ok(())
    }

    async fn execute(&self) -> Result<ExecuteResult, Self::Error> {
        tokio::time::sleep(Duration::from_millis(30)).await;

        Ok(ExecuteResult {
            affected_rows: 1,
            last_insert_id: Some(1),
            execution_time: Duration::from_millis(30),
        })
    }

    async fn query(&self) -> Result<QueryResult, Self::Error> {
        tokio::time::sleep(Duration::from_millis(40)).await;

        Ok(QueryResult {
            rows: vec![],
            columns: vec![],
            execution_time: Duration::from_millis(40),
        })
    }

    async fn clear_parameters(&mut self) -> Result<(), Self::Error> {
        self.parameters.clear();
        Ok(())
    }
}

// トランザクションの実装
pub struct SqliteTransaction {
    committed: bool,
    rolled_back: bool,
    savepoints: Vec<String>,
}

impl SqliteTransaction {
    fn new() -> Self {
        SqliteTransaction {
            committed: false,
            rolled_back: false,
            savepoints: Vec::new(),
        }
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    type Error = DatabaseError;

    async fn commit(mut self: Box<Self>) -> Result<(), Self::Error> {
        if self.rolled_back {
            return Err(DatabaseError::TransactionFailed(
                "Transaction already rolled back".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(20)).await;
        self.committed = true;
        println!("    ✓ Transaction committed");
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> Result<(), Self::Error> {
        if self.committed {
            return Err(DatabaseError::TransactionFailed(
                "Transaction already committed".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(15)).await;
        self.rolled_back = true;
        println!("    ✓ Transaction rolled back");
        Ok(())
    }

    async fn query(&self, _sql: &str) -> Result<QueryResult, Self::Error> {
        if self.rolled_back || self.committed {
            return Err(DatabaseError::TransactionFailed(
                "Transaction not active".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(50)).await;

        Ok(QueryResult {
            rows: vec![],
            columns: vec![],
            execution_time: Duration::from_millis(50),
        })
    }

    async fn execute(&self, _sql: &str) -> Result<ExecuteResult, Self::Error> {
        if self.rolled_back || self.committed {
            return Err(DatabaseError::TransactionFailed(
                "Transaction not active".to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_millis(35)).await;

        Ok(ExecuteResult {
            affected_rows: 1,
            last_insert_id: None,
            execution_time: Duration::from_millis(35),
        })
    }

    async fn savepoint(&self, name: &str) -> Result<(), Self::Error> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        println!("    ✓ Savepoint '{}' created", name);
        Ok(())
    }

    async fn rollback_to_savepoint(&self, name: &str) -> Result<(), Self::Error> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("    ✓ Rolled back to savepoint '{}'", name);
        Ok(())
    }
}

// テスト関数群
async fn basic_database_operations() -> Result<(), DatabaseError> {
    println!("【基本的なデータベース操作】");

    let mut conn = SqliteConnection::new("sqlite://memory:test.db".to_string());

    // 接続
    conn.connect().await?;
    println!("  接続状態: {}", conn.is_connected().await);

    // Ping
    let ping_time = conn.ping().await?;
    println!("  Ping時間: {:?}", ping_time);

    // クエリ実行
    let result = conn.query("SELECT * FROM users").await?;
    println!(
        "  クエリ結果: {} rows, 実行時間: {:?}",
        result.rows.len(),
        result.execution_time
    );

    for (i, row) in result.rows.iter().enumerate() {
        println!("    Row {}: {:?}", i + 1, row.columns);
    }

    // 更新実行
    let exec_result = conn
        .execute(
            "INSERT INTO users (name, email, age) VALUES ('Charlie', 'charlie@example.com', 35)",
        )
        .await?;
    println!(
        "  実行結果: {} rows affected, last_id: {:?}, 実行時間: {:?}",
        exec_result.affected_rows, exec_result.last_insert_id, exec_result.execution_time
    );

    // 切断
    conn.disconnect().await?;

    Ok(())
}

async fn advanced_database_features() -> Result<(), DatabaseError> {
    println!("\n【高度なデータベース機能】");

    let mut conn = SqliteConnection::new("sqlite://advanced.db".to_string());
    conn.connect().await?;

    // プリペアドステートメント
    println!("  プリペアドステートメント:");
    let mut stmt = conn
        .prepare("INSERT INTO users (name, email, age) VALUES (?, ?, ?)")
        .await?;
    stmt.bind_parameter(0, Value::Text("David".to_string()))
        .await?;
    stmt.bind_parameter(1, Value::Text("david@example.com".to_string()))
        .await?;
    stmt.bind_parameter(2, Value::Integer(28)).await?;

    let prep_result = stmt.execute().await?;
    println!("    実行結果: {} rows affected", prep_result.affected_rows);

    // バルクインサート
    println!("  バルクインサート:");
    let bulk_rows = vec![
        Row {
            columns: {
                let mut cols = HashMap::new();
                cols.insert("name".to_string(), Value::Text("Eve".to_string()));
                cols.insert(
                    "email".to_string(),
                    Value::Text("eve@example.com".to_string()),
                );
                cols.insert("age".to_string(), Value::Integer(32));
                cols
            },
        },
        Row {
            columns: {
                let mut cols = HashMap::new();
                cols.insert("name".to_string(), Value::Text("Frank".to_string()));
                cols.insert(
                    "email".to_string(),
                    Value::Text("frank@example.com".to_string()),
                );
                cols.insert("age".to_string(), Value::Integer(29));
                cols
            },
        },
    ];

    let bulk_result = conn.bulk_insert("users", bulk_rows).await?;
    println!(
        "    バルク結果: {} rows inserted, 実行時間: {:?}",
        bulk_result.affected_rows, bulk_result.execution_time
    );

    // テーブル情報取得
    println!("  テーブル情報:");
    let table_info = conn.get_table_info("users").await?;
    println!("    テーブル: {}", table_info.name);
    println!("    行数: {}", table_info.row_count);
    println!("    サイズ: {} bytes", table_info.size_bytes);
    println!("    カラム数: {}", table_info.columns.len());

    conn.disconnect().await?;
    Ok(())
}

async fn error_handling_demo() -> Result<(), DatabaseError> {
    println!("\n【エラーハンドリングとタイムアウト】");

    // 接続エラー
    println!("  接続エラーテスト:");
    let mut invalid_conn = SqliteConnection::new("".to_string());
    match invalid_conn.connect().await {
        Ok(_) => println!("    予期しない成功"),
        Err(e) => println!("    期待されたエラー: {}", e),
    }

    // 認証エラー
    let mut auth_conn = SqliteConnection::new("sqlite://invalid_auth".to_string());
    match auth_conn.connect().await {
        Ok(_) => println!("    予期しない成功"),
        Err(e) => println!("    認証エラー: {}", e),
    }

    // タイムアウトテスト
    println!("  タイムアウトテスト:");
    let mut conn = SqliteConnection::new("sqlite://timeout_test.db".to_string());
    conn.connect().await?;

    match conn
        .query_with_timeout("SELECT * FROM users", Duration::from_millis(1))
        .await
    {
        Ok(_) => println!("    予期しない成功"),
        Err(DatabaseError::Timeout) => println!("    期待されたタイムアウト"),
        Err(e) => println!("    その他のエラー: {}", e),
    }

    // 正常なタイムアウト付きクエリ
    let result = conn
        .query_with_timeout("SELECT * FROM users", Duration::from_secs(1))
        .await?;
    println!("    タイムアウト内で成功: {} rows", result.rows.len());

    conn.disconnect().await?;
    Ok(())
}

async fn transaction_demo() -> Result<(), DatabaseError> {
    println!("\n【トランザクション処理】");

    let mut conn = SqliteConnection::new("sqlite://transaction_test.db".to_string());
    conn.connect().await?;

    // 正常なトランザクション
    println!("  正常なトランザクション:");
    let tx = conn.begin_transaction().await?;
    tx.execute("INSERT INTO users (name) VALUES ('Transaction User 1')")
        .await?;
    tx.execute("INSERT INTO users (name) VALUES ('Transaction User 2')")
        .await?;
    tx.commit().await?;

    // ロールバックトランザクション
    println!("  ロールバックトランザクション:");
    let tx = conn.begin_transaction().await?;
    tx.execute("INSERT INTO users (name) VALUES ('Rollback User')")
        .await?;
    tx.savepoint("before_error").await?;
    tx.execute("UPDATE users SET name = 'Updated'").await?;
    tx.rollback_to_savepoint("before_error").await?;
    tx.rollback().await?;

    // ネストしたトランザクション（セーブポイント）
    println!("  セーブポイント付きトランザクション:");
    let tx = conn.begin_transaction().await?;
    tx.execute("INSERT INTO users (name) VALUES ('Savepoint User')")
        .await?;
    tx.savepoint("checkpoint1").await?;
    tx.execute("UPDATE users SET age = 40 WHERE name = 'Savepoint User'")
        .await?;
    tx.savepoint("checkpoint2").await?;
    tx.rollback_to_savepoint("checkpoint1").await?;
    tx.commit().await?;

    conn.disconnect().await?;
    Ok(())
}

async fn connection_pooling_demo() -> Result<(), DatabaseError> {
    println!("\n【接続プーリング概念デモ】");

    // 複数の接続を並行で作成
    let connections = futures::future::try_join_all((0..3).map(|i| async move {
        let mut conn = SqliteConnection::new(format!("sqlite://pool_{}.db", i));
        conn.connect().await?;
        println!("    接続 {} 完了", i);
        Ok::<_, DatabaseError>(conn)
    }))
    .await?;

    println!("  {} 個の接続がプールに作成されました", connections.len());

    // 並行クエリ実行
    let query_results =
        futures::future::join_all(connections.iter().enumerate().map(|(i, conn)| async move {
            let start = Instant::now();
            let result = conn.query("SELECT * FROM users").await;
            let duration = start.elapsed();
            (i, result, duration)
        }))
        .await;

    for (i, result, duration) in query_results {
        match result {
            Ok(query_result) => println!(
                "    接続 {}: {} rows, {:?}",
                i,
                query_result.rows.len(),
                duration
            ),
            Err(e) => println!("    接続 {}: エラー {}", i, e),
        }
    }

    // 接続のクリーンアップ
    for (i, mut conn) in connections.into_iter().enumerate() {
        conn.disconnect().await?;
        println!("    接続 {} 切断完了", i);
    }

    println!("\n【実装のポイント】");
    println!("✓ async_traitによる非同期トレイトメソッド");
    println!("✓ 包括的なエラーハンドリング");
    println!("✓ タイムアウト機能の統合");
    println!("✓ プリペアドステートメントのサポート");
    println!("✓ トランザクションとセーブポイント");
    println!("✓ バルク操作の効率的実装");
    println!("✓ 接続プーリングパターン");

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn begin_transaction_fails_when_not_connected() {
        let conn = SqliteConnection::new("sqlite://test.db".to_string());

        let result = conn.begin_transaction().await;

        assert!(matches!(result, Err(DatabaseError::TransactionFailed(_))));
    }

    #[tokio::test]
    async fn begin_transaction_returns_transaction_sharing_connection_error_type() {
        let mut conn = SqliteConnection::new("sqlite://test.db".to_string());
        conn.connect().await.unwrap();

        let tx: Box<dyn Transaction<Error = DatabaseError>> =
            conn.begin_transaction().await.unwrap();

        assert!(tx.commit().await.is_ok());
    }

    #[tokio::test]
    async fn prepare_returns_statement_sharing_connection_error_type() {
        let mut conn = SqliteConnection::new("sqlite://test.db".to_string());
        conn.connect().await.unwrap();

        let stmt: Result<Box<dyn PreparedStatement<Error = DatabaseError>>, _> =
            conn.prepare("SELECT 1").await;

        assert!(stmt.is_ok());
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main().unwrap();
    }
}
