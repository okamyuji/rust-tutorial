//! 実用的なマクロ集
//! 
//! 実際の開発で役立つマクロパターンを示します。

use std::time::{Duration, Instant};
use std::collections::HashMap;

// 時間計測マクロ
macro_rules! measure_time {
    ($name:expr, $body:expr) => {{
        let start = Instant::now();
        let result = $body;
        let duration = start.elapsed();
        eprintln!("{}: {:?}", $name, duration);
        result
    }};
}

// JSONライクなオブジェクト生成マクロ
macro_rules! json {
    // 空のオブジェクト
    ({}) => {
        JsonValue::Object(std::collections::HashMap::new())
    };
    // 値を expr で受けると [..] や {..} が不透明な式になり、再帰で配列・オブジェクトの分岐に一致しない。
    // そのため tt で受ける（ponytail: -5 のような複数トークンの値は未対応。必要なら tt muncher にする）
    // キー・バリューペア
    ({ $($key:literal : $value:tt),* $(,)? }) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), json!($value));
        )*
        JsonValue::Object(map)
    }};
    // 配列
    ([ $($element:tt),* $(,)? ]) => {
        JsonValue::Array(vec![$(json!($element)),*])
    };
    // プリミティブ値
    ($value:expr) => {
        JsonValue::from($value)
    };
}

#[derive(Debug, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl From<i32> for JsonValue {
    fn from(n: i32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<f64> for JsonValue {
    fn from(n: f64) -> Self {
        JsonValue::Number(n)
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

// エラーハンドリングマクロ
macro_rules! try_or_return {
    ($expr:expr, $err_msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}: {:?}", $err_msg, e);
                return Err(e.into());
            }
        }
    };
}

// 簡単なビルダーパターンマクロ（pasteなし）
macro_rules! simple_builder {
    (
        struct $name:ident {
            $($field:ident: $type:ty = $default:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($field: $default),*
                }
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };
}

// 遅延評価マクロ
macro_rules! lazy_static {
    ($name:ident : $type:ty = $init:expr) => {
        fn $name() -> &'static $type {
            // static mut と unsafe を使わず、初回だけ初期化する標準の OnceLock に任せる
            static VALUE: std::sync::OnceLock<$type> = std::sync::OnceLock::new();
            VALUE.get_or_init(|| $init)
        }
    };
}

// アサートマクロの拡張
macro_rules! assert_matches {
    ($expr:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expr {
            $pattern $(if $guard)? => {},
            ref val => panic!("assertion failed: `{:?}` does not match `{}`", 
                val, stringify!($pattern)),
        }
    };
}

// リトライマクロ
macro_rules! retry {
    ($max_attempts:expr, $delay:expr, $body:expr) => {{
        let mut attempts = 0;
        loop {
            attempts += 1;
            match $body {
                Ok(val) => break Ok(val),
                Err(e) if attempts >= $max_attempts => break Err(e),
                Err(_) => {
                    std::thread::sleep($delay);
                }
            }
        }
    }};
}

// メモ化マクロ
macro_rules! memoize {
    ($name:ident, $arg_type:ty, $ret_type:ty, $body:expr) => {
        fn $name(arg: $arg_type) -> $ret_type {
            use std::collections::HashMap;
            use std::cell::RefCell;
            
            thread_local! {
                static CACHE: RefCell<HashMap<$arg_type, $ret_type>> = RefCell::new(HashMap::new());
            }
            
            // 本体は自分自身を再帰呼び出しするため、借用を保持したまま評価すると二重借用で panic する
            if let Some(hit) = CACHE.with(|cache| cache.borrow().get(&arg).cloned()) {
                return hit;
            }
            let value = $body(arg.clone());
            CACHE.with(|cache| cache.borrow_mut().insert(arg, value.clone()));
            value
        }
    };
}

// 設定マクロ
macro_rules! config {
    (
        $(
            $name:ident: $type:ty = $default:expr
        ),* $(,)?
    ) => {
        pub struct Config {
            $(pub $name: $type),*
        }

        impl Default for Config {
            fn default() -> Self {
                Self {
                    $($name: $default),*
                }
            }
        }

        impl Config {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                pub fn $name(mut self, value: $type) -> Self {
                    self.$name = value;
                    self
                }
            )*
        }
    };
}

// パイプラインマクロ
macro_rules! pipe {
    ($value:expr $(,)?) => { $value };
    ($value:expr, $func:expr $(, $rest:expr)*) => {
        pipe!($func($value) $(, $rest)*)
    };
}

// スレッドセーフなカウンターマクロ
macro_rules! thread_safe_counter {
    ($name:ident) => {
        pub struct $name {
            count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
                }
            }

            pub fn increment(&self) -> usize {
                self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }

            pub fn get(&self) -> usize {
                self.count.load(std::sync::atomic::Ordering::SeqCst)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self {
                    count: self.count.clone(),
                }
            }
        }
    };
}

// デバッグプリントマクロ
macro_rules! dbg_vars {
    ($($var:expr),* $(,)?) => {
        {
            eprintln!("[{}:{}]", file!(), line!());
            $(
                eprintln!("  {} = {:?}", stringify!($var), $var);
            )*
        }
    };
}

fn main() {
    println!("=== 実用的なマクロ集 ===\n");

    // 時間計測
    println!("--- 時間計測 ---");
    let result = measure_time!("Heavy computation", {
        // 合計は約 5×10^13 になり、既定の i32 では溢れる
        let mut sum: u64 = 0;
        for i in 0..10_000_000 {
            sum += i;
        }
        sum
    });
    println!("計算結果: {}", result);

    // JSON生成
    println!("\n--- JSON生成 ---");
    let data = json!({
        "name": "Alice",
        "age": 30,
        "active": true,
        "scores": [95, 87, 92],
        "address": {
            "city": "Tokyo",
            "country": "Japan"
        }
    });
    println!("生成されたJSON: {:#?}", data);

    // アサートマッチ
    println!("\n--- パターンマッチアサート ---");
    let value = Some(42);
    assert_matches!(value, Some(x) if x > 0);
    println!("アサート成功: {:?}", value);

    // 遅延評価
    println!("\n--- 遅延評価 ---");
    lazy_static!(expensive_data: Vec<i32> = {
        println!("高コストな初期化を実行");
        vec![1, 2, 3, 4, 5]
    });
    
    println!("初回アクセス: {:?}", expensive_data());
    println!("2回目アクセス: {:?}", expensive_data());

    // リトライ処理
    println!("\n--- リトライ処理 ---");
    let mut counter = 0;
    let result = retry!(3, Duration::from_millis(100), {
        counter += 1;
        if counter < 3 {
            println!("試行 {}: 失敗", counter);
            Err("まだ準備できていません")
        } else {
            println!("試行 {}: 成功", counter);
            Ok("成功！")
        }
    });
    println!("リトライ結果: {:?}", result);

    // 設定
    println!("\n--- 設定マクロ ---");
    config! {
        host: &'static str = "localhost",
        port: u16 = 8080,
        debug: bool = false,
        timeout: Duration = Duration::from_secs(30),
    }

    let config = Config::new()
        .port(3000)
        .debug(true);
    
    println!("設定: host={}, port={}, debug={}", 
        config.host, config.port, config.debug);

    // パイプライン
    println!("\n--- パイプライン ---");
    let result = pipe!(
        5,
        |x| x * 2,
        |x| x + 10,
        |x| x as f64,
        |x: f64| x.sqrt()
    );
    println!("パイプライン結果: {}", result);

    // メモ化（フィボナッチ）
    println!("\n--- メモ化 ---");
    memoize!(fib_memo, u32, u32, |n: u32| -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib_memo(n - 1) + fib_memo(n - 2)
        }
    });

    let start = Instant::now();
    println!("fib(35) = {}", fib_memo(35));
    println!("1回目の実行時間: {:?}", start.elapsed());

    let start = Instant::now();
    println!("fib(35) = {}", fib_memo(35));
    println!("2回目の実行時間: {:?}", start.elapsed());

    // 簡単なビルダーの使用例
    println!("\n--- 簡単なビルダー ---");
    simple_builder! {
        struct Person {
            name: String = String::new(),
            age: u32 = 0,
            email: String = String::new(),
        }
    }

    let person = Person::new()
        .name("Bob".to_string())
        .age(25)
        .email("bob@example.com".to_string());
    
    println!("Person: {:?}", person);

    // スレッドセーフカウンター
    println!("\n--- スレッドセーフカウンター ---");
    thread_safe_counter!(Counter);
    
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("カウンター: {}", counter.get());

    // デバッグ変数
    println!("\n--- デバッグ変数 ---");
    let x = 42;
    let y = "Hello";
    let z = vec![1, 2, 3];
    dbg_vars!(x, y, z);

    println!("\n実用的なマクロパターンをマスターしました！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_builds_nested_arrays_and_objects() {
        let value = json!({
            "scores": [1, 2],
            "address": { "city": "Tokyo" },
            "ok": true
        });

        let JsonValue::Object(map) = value else { panic!("object expected") };
        assert!(matches!(&map["scores"], JsonValue::Array(items)
            if matches!(items.as_slice(), [JsonValue::Number(a), JsonValue::Number(b)] if *a == 1.0 && *b == 2.0)));
        assert!(matches!(&map["address"], JsonValue::Object(inner)
            if matches!(&inner["city"], JsonValue::String(s) if s == "Tokyo")));
        assert!(matches!(map["ok"], JsonValue::Bool(true)));
    }

    #[test]
    fn json_handles_empty_object_and_array() {
        assert!(matches!(json!({}), JsonValue::Object(m) if m.is_empty()));
        assert!(matches!(json!([]), JsonValue::Array(v) if v.is_empty()));
    }

    memoize!(memo_fib, u64, u64, |n: u64| -> u64 {
        if n < 2 { n } else { memo_fib(n - 1) + memo_fib(n - 2) }
    });

    #[test]
    fn memoize_supports_recursive_bodies_and_caches_results() {
        assert_eq!(memo_fib(0), 0);
        assert_eq!(memo_fib(1), 1);
        assert_eq!(memo_fib(50), 12_586_269_025);
        assert_eq!(memo_fib(50), 12_586_269_025);
    }

    #[test]
    fn main_runs_without_panicking() {
        main();
    }
}
