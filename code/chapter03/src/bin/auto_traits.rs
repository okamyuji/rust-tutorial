// src/bin/auto_traits.rs

use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// 自動的にSendを実装
fn is_send<T: Send>(_: &T) {
    println!("  ✓ Send を実装");
}

// 自動的にSyncを実装
fn is_sync<T: Sync>(_: &T) {
    println!("  ✓ Sync を実装");
}

fn main() {
    println!("=== トレイトの自動実装 ===\n");

    derive_traits();
    auto_traits_demo();
    marker_traits();
    negative_reasoning();
}

// 派生マクロによる自動実装
fn derive_traits() {
    println!("--- 派生（Derive）マクロ ---");

    // 基本的な派生
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    let p3 = Point { x: 10, y: 20 };

    println!("p1: {:?}", p1);
    println!("p2 (cloned): {:?}", p2);
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);

    // より複雑な構造体
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,
    }

    impl Person {
        fn new(name: &str, age: u32) -> Self {
            Person {
                name: name.to_string(),
                age,
                email: None,
            }
        }

        fn with_email(mut self, email: &str) -> Self {
            self.email = Some(email.to_string());
            self
        }
    }

    let person1 = Person::new("Alice", 30).with_email("alice@example.com");
    let person2 = person1.clone();

    println!("\n{:?}", person1);
    println!("クローン: {:?}", person2);
    println!("同一性: {}", person1 == person2);

    // 列挙型への派生
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
        Critical = 4,
    }

    let mut priorities = vec![
        Priority::Medium,
        Priority::Critical,
        Priority::Low,
        Priority::High,
    ];

    println!("\nソート前: {:?}", priorities);
    priorities.sort();
    println!("ソート後: {:?}", priorities);

    // カスタム派生の制限
    #[derive(Debug)]
    struct Container<T> {
        data: Vec<T>,
    }

    // Debugは、TがDebugを実装している場合のみ派生される
    let int_container = Container {
        data: vec![1, 2, 3],
    };
    println!("\n{:?}", int_container);
}

// 自動トレイト（Auto Traits）
fn auto_traits_demo() {
    println!("\n--- 自動トレイト（Send, Sync） ---");

    // Send: 他のスレッドに移動可能
    // Sync: 他のスレッドから参照可能

    // Sendを実装する型
    struct SendableData {
        value: i32,
        text: String,
    }

    // 関数は既にファイル先頭で定義済み

    let sendable = SendableData {
        value: 42,
        text: "Hello".to_string(),
    };

    println!("SendableData:");
    is_send(&sendable);
    is_sync(&sendable);

    // Sendを実装しない型（Rcを含む）
    struct NotSendable {
        data: Rc<String>,
    }

    let not_sendable = NotSendable {
        data: Rc::new("shared".to_string()),
    };

    println!("\nNotSendable:");
    // is_send(&not_sendable); // コンパイルエラー
    // is_sync(&not_sendable); // コンパイルエラー（Rc は Sync も実装しない）

    // Syncを実装しない型（RefCellを含む）
    struct NotSync {
        data: RefCell<i32>,
    }

    let not_sync = NotSync {
        data: RefCell::new(42),
    };

    println!("\nNotSync:");
    is_send(&not_sync);
    // is_sync(&not_sync); // コンパイルエラー

    // Send + Syncを実装する型（Arcを含む）
    struct ThreadSafe {
        data: Arc<Mutex<Vec<i32>>>,
    }

    let thread_safe = ThreadSafe {
        data: Arc::new(Mutex::new(vec![1, 2, 3])),
    };

    println!("\nThreadSafe:");
    is_send(&thread_safe);
    is_sync(&thread_safe);

    // PhantomDataによる自動トレイトの制御
    use std::marker::PhantomData;

    struct CustomSend<T> {
        _phantom: PhantomData<T>,
    }

    struct CustomNotSend<T> {
        _phantom: PhantomData<*const T>, // 生ポインタはSendではない
    }

    let custom_send = CustomSend::<i32> {
        _phantom: PhantomData,
    };
    let custom_not_send = CustomNotSend::<i32> {
        _phantom: PhantomData,
    };

    println!("\nCustomSend:");
    is_send(&custom_send);

    println!("\nCustomNotSend:");
    // is_send(&custom_not_send); // コンパイルエラー
}

// マーカートレイト
fn marker_traits() {
    println!("\n--- マーカートレイト ---");

    // カスタムマーカートレイト
    trait Serializable {}
    trait Cacheable {}
    trait Validated {}

    // 型にマーカーを付与
    #[derive(Debug)]
    struct User {
        id: u64,
        name: String,
    }

    impl Serializable for User {}
    impl Cacheable for User {}

    #[derive(Debug)]
    struct TempData {
        value: String,
    }

    // TempDataはSerializableでもCacheableでもない

    // マーカートレイトによる条件付き実装
    trait Storage {
        fn store<T: Serializable>(&mut self, key: &str, value: &T);
        fn cache<T: Cacheable>(&mut self, key: &str, value: &T);
    }

    struct MemoryStorage {
        data: std::collections::HashMap<String, String>,
    }

    impl Storage for MemoryStorage {
        fn store<T: Serializable>(&mut self, key: &str, _value: &T) {
            self.data
                .insert(key.to_string(), "serialized_data".to_string());
            println!("データを永続化: {}", key);
        }

        fn cache<T: Cacheable>(&mut self, key: &str, _value: &T) {
            self.data.insert(key.to_string(), "cached_data".to_string());
            println!("データをキャッシュ: {}", key);
        }
    }

    let mut storage = MemoryStorage {
        data: std::collections::HashMap::new(),
    };

    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    storage.store("user:1", &user);
    storage.cache("user:1", &user);

    let _temp = TempData {
        value: "temporary".to_string(),
    };

    // storage.store("temp", &_temp); // コンパイルエラー：Serializableではない
    // storage.cache("temp", &_temp); // コンパイルエラー：Cacheableではない

    // 複合マーカー
    trait Premium: Serializable + Cacheable + Validated {}

    // ブランケット実装
    impl<T: Serializable + Cacheable + Validated> Premium for T {}

    impl Validated for User {}

    // これでUserはPremiumを自動的に実装
    fn process_premium<T: Premium>(_item: &T) {
        println!("プレミアムアイテムを処理");
    }

    process_premium(&user);
}

// 否定的推論（Negative Reasoning）
fn negative_reasoning() {
    println!("\n--- 否定的推論 ---");

    // Sized トレイト
    fn sized_value<T>(_val: T) {
        println!("Sized型の値");
    }

    fn unsized_value<T: ?Sized>(_val: &T) {
        println!("Sized または !Sized型の参照");
    }

    sized_value(42);
    sized_value(String::from("hello"));

    unsized_value(&42);
    unsized_value("string slice"); // strは!Sized
    unsized_value(&[1, 2, 3] as &[i32]); // スライスは!Sized

    // Unpin トレイト（非同期コンテキストで重要）
    use std::pin::Pin;

    struct Movable {
        data: String,
    }

    struct NotMovable {
        data: String,
        _pin: PhantomData<Pin<()>>,
    }

    // impl !Unpin for NotMovable {} // 実際にはできない（例示のみ）

    fn requires_unpin<T: Unpin>(_val: &T) {
        println!("Unpin型");
    }

    let movable = Movable {
        data: "can move".to_string(),
    };

    requires_unpin(&movable);

    // Copy の自動実装条件
    #[derive(Clone, Debug)]
    struct CanCopy {
        x: i32,
        y: i32,
    }

    // すべてのフィールドがCopyなので、Copyを実装可能
    impl Copy for CanCopy {}

    #[derive(Clone)]
    struct CannotCopy {
        x: i32,
        data: String, // StringはCopyではない
    }

    // impl Copy for CannotCopy {} // エラー：StringがCopyではない

    fn test_copy<T: Copy>(val: T) -> (T, T) {
        (val, val) // Copyなので2回使える
    }

    let copyable = CanCopy { x: 10, y: 20 };
    let (c1, c2) = test_copy(copyable);
    println!("\nCopy型: {:?}, {:?}", c1, c2);

    // 自動トレイト境界の伝播
    struct Wrapper<T> {
        inner: T,
    }

    // WrapperはTがSend/Syncの場合のみSend/Sync
    impl<T> Wrapper<T> {
        fn new(inner: T) -> Self {
            Wrapper { inner }
        }
    }

    let send_wrapper = Wrapper::new(42);
    is_send(&send_wrapper);

    let _not_send_wrapper = Wrapper::new(Rc::new(42));
    // is_send(&not_send_wrapper); // エラー：RcはSendではない

    println!("\n自動トレイトは構造体のフィールドに基づいて自動的に決定されます");
}

// ヘルパー構造体

// Unpinを実装しない型（仮想的な例）
struct PinnedData {
    data: String,
    _pin: PhantomData<std::marker::PhantomPinned>,
}
