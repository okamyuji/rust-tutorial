// src/bin/static_lifetime.rs

use std::fmt::Display;

fn main() {
    println!("=== 'staticライフタイムの真の意味 ===\n");

    static_references();
    static_trait_bound();
    common_misconceptions();
    practical_static_usage();
    static_vs_owned();
    summary();
}

fn static_references() {
    println!("--- 'static参照 ---");

    // 文字列リテラルは'static
    let literal: &'static str = "これは'static文字列リテラル";
    println!("リテラル: {}", literal);

    // 静的変数への参照
    static GLOBAL_NUMBER: i32 = 42;
    let static_ref: &'static i32 = &GLOBAL_NUMBER;
    println!("静的変数: {}", static_ref);

    // 定数への参照
    const CONST_VALUE: &str = "定数文字列";
    let const_ref: &'static str = CONST_VALUE;
    println!("定数: {}", const_ref);

    // Box::leakで'static参照を作成
    demonstrate_box_leak();
}

fn demonstrate_box_leak() {
    println!("\n[Box::leakによる'static参照の作成]");

    // 実行時に'static参照を作成
    let boxed = Box::new(String::from("動的に作成された文字列"));
    let leaked: &'static String = Box::leak(boxed);
    println!("リークされた値: {}", leaked);

    // 注意：これはメモリリークを意図的に起こしている
    // 通常のコードでは避けるべき
}

fn static_trait_bound() {
    println!("\n--- 'staticトレイト境界 ---");

    // T: 'static の意味
    demonstrate_static_bound();

    // スレッドとの関係
    demonstrate_thread_static();

    // 動的な値でも'static境界を満たせる
    demonstrate_owned_values();
}

fn demonstrate_static_bound() {
    println!("\n[T: 'static の意味]");

    // 'static境界は「参照を含まない」または「'static参照のみ含む」
    fn requires_static<T: 'static>(_value: T) {
        // valueは借用を含まない、または'static借用のみ
        println!("'static境界を満たす値を受け取りました");
    }

    // OK: 所有された値
    requires_static(String::from("所有された文字列"));
    requires_static(vec![1, 2, 3]);
    requires_static(42);

    // OK: 'static参照
    requires_static("静的文字列");

    // NG: 非'static参照を含む
    // let temp = String::from("一時的");
    // requires_static(&temp); // エラー！
}

fn demonstrate_thread_static() {
    println!("\n[スレッドと'static]");

    use std::thread;

    // スレッドに送るデータは'static境界が必要
    let data = String::from("スレッドに送るデータ");

    let handle = thread::spawn(move || {
        // dataの所有権がスレッドに移動
        println!("スレッド内: {}", data);
    });

    handle.join().unwrap();

    // 'static参照も送れる
    let static_data = "静的データ";
    let handle = thread::spawn(move || {
        println!("スレッド内の静的データ: {}", static_data);
    });

    handle.join().unwrap();
}

fn demonstrate_owned_values() {
    println!("\n[所有された値と'static境界]");

    // 構造体の例
    #[derive(Debug)]
    struct Container<T> {
        value: T,
    }

    impl<T: 'static> Container<T> {
        fn new(value: T) -> Self {
            Container { value }
        }

        fn process(&self)
        where
            T: Display,
        {
            println!("処理中: {}", self.value);
        }
    }

    // 所有された値はOK
    let container1 = Container::new(String::from("動的文字列"));
    let container2 = Container::new(vec![1, 2, 3, 4, 5]);
    let container3 = Container::new(42);

    println!("コンテナ1: {:?}", container1);
    println!("コンテナ2: {:?}", container2);
    println!("コンテナ3: {:?}", container3);

    // processメソッドを使用
    container1.process();
    container3.process();
}

fn common_misconceptions() {
    println!("\n--- 'staticの一般的な誤解 ---");

    misconception_lifetime_duration();
    misconception_static_memory();
    correct_understanding();
}

fn misconception_lifetime_duration() {
    println!("\n[誤解1: 'static = プログラム全体で生きる]");

    // 'static境界は値の実際の寿命を意味しない
    fn takes_static<T: 'static>(value: T) {
        // valueはこの関数の終了時に破棄される
        drop(value);
        println!("値は破棄されました");
    }

    let owned = String::from("すぐに破棄される文字列");
    takes_static(owned);
    // ownedはもう存在しない

    println!("T: 'static は「Tが'staticでない借用を含まない」という意味");
}

fn misconception_static_memory() {
    println!("\n[誤解2: 'static = 静的メモリ領域]");

    // 'static境界を満たす動的割り当て
    let heap_allocated: Box<i32> = Box::new(42);
    check_static(heap_allocated);

    let vec_allocated: Vec<i32> = vec![1, 2, 3];
    check_static(vec_allocated);

    fn check_static<T: 'static>(_value: T) {
        println!("ヒープ上の値も'static境界を満たせる");
    }
}

fn correct_understanding() {
    println!("\n[正しい理解]");

    // 'staticライフタイムを持つ参照
    let _static_ref: &'static str = "本当に永続的";
    println!("static参照: {}", _static_ref);

    // 'static境界を満たす型
    fn demonstrate_bound<T: 'static>(value: T) -> T {
        value
    }

    // これらはすべてOK
    let _ = demonstrate_bound(42);
    let _ = demonstrate_bound(String::from("所有"));
    let _ = demonstrate_bound("リテラル");

    println!("'static参照 != 'static境界");
}

fn practical_static_usage() {
    println!("\n--- 実践的な'staticの使用 ---");

    lazy_static_pattern();
    type_erasure_with_static();
    global_state_example();
}

fn lazy_static_pattern() {
    println!("\n[遅延初期化パターン]");

    // OnceCell/LazyLockのような遅延初期化
    use std::sync::Mutex;

    static COUNTER: Mutex<i32> = Mutex::new(0);

    {
        let mut count = COUNTER.lock().unwrap();
        *count += 1;
        println!("カウンター: {}", *count);
    }

    {
        let mut count = COUNTER.lock().unwrap();
        *count += 1;
        println!("カウンター: {}", *count);
    }
}

fn type_erasure_with_static() {
    println!("\n[型消去と'static]");

    // トレイトオブジェクトで'static境界
    let handlers: Vec<Box<dyn Handler + 'static>> = vec![
        Box::new(StringHandler::new("ハンドラ1")),
        Box::new(NumberHandler::new(42)),
    ];

    for handler in &handlers {
        handler.handle();
    }
}

trait Handler {
    fn handle(&self);
}

struct StringHandler {
    data: String,
}

impl StringHandler {
    fn new(data: &str) -> Self {
        StringHandler {
            data: data.to_string(),
        }
    }
}

impl Handler for StringHandler {
    fn handle(&self) {
        println!("文字列ハンドラ: {}", self.data);
    }
}

struct NumberHandler {
    value: i32,
}

impl NumberHandler {
    fn new(value: i32) -> Self {
        NumberHandler { value }
    }
}

impl Handler for NumberHandler {
    fn handle(&self) {
        println!("数値ハンドラ: {}", self.value);
    }
}

fn global_state_example() {
    println!("\n[グローバル状態の例]");

    // 定数
    const MAX_SIZE: usize = 100;
    println!("最大サイズ: {}", MAX_SIZE);

    // 静的変数（unsafe）
    static mut MUTABLE_STATIC: i32 = 0;

    unsafe {
        MUTABLE_STATIC += 1;
        // 可変静的変数への安全でない参照を避ける
        let value = MUTABLE_STATIC;
        println!("可変静的変数: {}", value);
    }

    // より安全な方法：Mutex
    use std::sync::Mutex;
    static SAFE_STATIC: Mutex<Vec<String>> = Mutex::new(Vec::new());

    {
        let mut vec = SAFE_STATIC.lock().unwrap();
        vec.push(String::from("安全に追加"));
        println!("安全な静的ベクタ: {:?}", *vec);
    }
}

fn static_vs_owned() {
    println!("\n--- 'static参照 vs 所有された値 ---");

    // いつ'static参照を使うか
    when_to_use_static_ref();

    // いつ所有された値を使うか
    when_to_use_owned();

    // パフォーマンスの考慮
    performance_considerations();
}

fn when_to_use_static_ref() {
    println!("\n['static参照を使う場合]");

    // 設定値やエラーメッセージ
    const ERROR_MESSAGE: &str = "エラーが発生しました";
    const CONFIG_PATH: &str = "/etc/myapp/config.toml";

    fn get_error() -> &'static str {
        ERROR_MESSAGE
    }

    println!("エラーメッセージ: {}", get_error());
    println!("設定パス: {}", CONFIG_PATH);
}

fn when_to_use_owned() {
    println!("\n[所有された値を使う場合]");

    // 動的に生成される値
    fn create_message(name: &str) -> String {
        format!("こんにちは、{}さん！", name)
    }

    let message = create_message("太郎");
    println!("{}", message);

    // 可変性が必要な場合
    let mut data = vec![1, 2, 3];
    data.push(4);
    println!("可変データ: {:?}", data);
}

fn performance_considerations() {
    println!("\n[パフォーマンスの考慮]");

    // 'static参照：コピーが安い
    let static_ref = "static";
    let copy1 = static_ref;
    let copy2 = static_ref;
    println!("参照のコピーは安い: {}, {}", copy1, copy2);

    // 所有された値：柔軟性が高い
    let owned = String::from("owned");
    let modified = format!("{} - modified", owned);
    println!("所有された値の変更: {}", modified);
}

// まとめ
fn summary() {
    println!("\n=== まとめ ===");
    println!("1. &'static T: プログラム全体で有効な参照");
    println!("2. T: 'static: Tは'staticでない借用を含まない");
    println!("3. 'static != 永遠に生きる");
    println!("4. 'static != 静的メモリ領域");
}
