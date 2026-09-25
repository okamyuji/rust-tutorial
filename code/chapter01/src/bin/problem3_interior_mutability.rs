// src/bin/problem3_interior_mutability.rs
// 復習問題3: 内部可変性パターンの解答

// Cell と RefCell は各関数内で個別にインポート

fn main() {
    println!("=== 復習問題3: 内部可変性（Cell vs RefCell）===\n");

    println!("【内部可変性とは】");
    println!("・不変参照を通じて内部データを変更する仕組み");
    println!("・通常の借用ルールを実行時チェックで置き換え");
    println!("・主にCell<T>とRefCell<T>が使用される");

    // Cell<T>の例
    println!("\n=== Cell<T>が適している場面 ===");
    cell_examples();

    // RefCell<T>の例
    println!("\n=== RefCell<T>が適している場面 ===");
    refcell_examples();

    // 使い分けの比較
    println!("\n=== Cell vs RefCell の比較 ===");
    comparison_examples();

    // エラーハンドリング
    println!("\n=== RefCellのエラーハンドリング ===");
    error_handling_examples();
}

// Cell<T>の使用例
fn cell_examples() {
    use std::cell::Cell;

    // カウンターの実装
    struct Counter {
        value: Cell<i32>, // Copyトレイトを実装する型
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                value: Cell::new(0),
            }
        }

        // 不変参照でも更新可能
        fn increment(&self) {
            let current = self.value.get();
            self.value.set(current + 1);
        }

        fn get(&self) -> i32 {
            self.value.get()
        }

        fn reset(&self) {
            self.value.set(0);
        }
    }

    println!("【Cellを使ったカウンター】");
    let counter = Counter::new();

    println!("初期値: {}", counter.get());
    counter.increment();
    counter.increment();
    println!("インクリメント後: {}", counter.get());
    counter.reset();
    println!("リセット後: {}", counter.get());

    // 設定値の管理
    struct Config {
        debug_mode: Cell<bool>,
        max_connections: Cell<u32>,
    }

    impl Config {
        fn new() -> Self {
            Config {
                debug_mode: Cell::new(false),
                max_connections: Cell::new(100),
            }
        }

        fn enable_debug(&self) {
            self.debug_mode.set(true);
        }

        fn set_max_connections(&self, max: u32) {
            self.max_connections.set(max);
        }

        fn is_debug_enabled(&self) -> bool {
            self.debug_mode.get()
        }
    }

    println!("\n【Cellを使った設定管理】");
    let config = Config::new();

    println!("デバッグモード: {}", config.is_debug_enabled());
    config.enable_debug();
    config.set_max_connections(500);
    println!("設定変更後 - デバッグモード: {}", config.is_debug_enabled());

    println!("\n✓ Cell<T>は Copyトレイトを実装する単純な型に適している");
    println!("✓ get()とset()で全体の置き換えのみ可能");
    println!("✓ 借用は発生せず、常に値のコピーを取得");
}

// RefCell<T>の使用例
fn refcell_examples() {
    use std::cell::RefCell;

    // 図書館システム
    struct Library {
        books: RefCell<Vec<String>>, // 複雑な型、借用が必要
    }

    impl Library {
        fn new() -> Self {
            Library {
                books: RefCell::new(Vec::new()),
            }
        }

        fn add_book(&self, book: String) {
            self.books.borrow_mut().push(book);
        }

        fn remove_book(&self, title: &str) -> bool {
            let mut books = self.books.borrow_mut();
            if let Some(pos) = books.iter().position(|x| x == title) {
                books.remove(pos);
                true
            } else {
                false
            }
        }

        fn get_books(&self) -> Vec<String> {
            self.books.borrow().clone()
        }

        fn book_count(&self) -> usize {
            self.books.borrow().len()
        }

        fn search_books(&self, query: &str) -> Vec<String> {
            self.books
                .borrow()
                .iter()
                .filter(|book| book.contains(query))
                .cloned()
                .collect()
        }
    }

    println!("【RefCellを使った図書館システム】");
    let library = Library::new();

    library.add_book("Rust Programming".to_string());
    library.add_book("Systems Programming".to_string());
    library.add_book("Web Development".to_string());

    println!("蔵書数: {}", library.book_count());
    println!("全ての本: {:?}", library.get_books());

    let programming_books = library.search_books("Programming");
    println!("Programmingを含む本: {:?}", programming_books);

    library.remove_book("Web Development");
    println!("削除後の蔵書: {:?}", library.get_books());

    // キャッシュシステム
    struct Cache {
        data: RefCell<std::collections::HashMap<String, String>>,
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: RefCell::new(std::collections::HashMap::new()),
            }
        }

        fn get(&self, key: &str) -> Option<String> {
            self.data.borrow().get(key).cloned()
        }

        fn set(&self, key: String, value: String) {
            self.data.borrow_mut().insert(key, value);
        }

        fn contains(&self, key: &str) -> bool {
            self.data.borrow().contains_key(key)
        }

        fn clear(&self) {
            self.data.borrow_mut().clear();
        }
    }

    println!("\n【RefCellを使ったキャッシュシステム】");
    let cache = Cache::new();

    cache.set("user:123".to_string(), "Alice".to_string());
    cache.set("user:456".to_string(), "Bob".to_string());

    if let Some(user) = cache.get("user:123") {
        println!("ユーザー123: {}", user);
    }

    println!("user:999は存在する？ {}", cache.contains("user:999"));

    // clearメソッドの使用例
    cache.clear();
    println!("\\nキャッシュをクリア後:");
    println!("user:123は存在する？ {}", cache.contains("user:123"));

    println!("\n✓ RefCell<T>は複雑な型や部分的な変更が必要な場合に適している");
    println!("✓ borrow()とborrow_mut()で借用を取得");
    println!("✓ 実行時に借用ルールをチェック");
}

// 使い分けの比較例
fn comparison_examples() {
    use std::cell::{Cell, RefCell};

    println!("【パフォーマンス比較】");

    // Cell<T>の場合
    let cell_counter = Cell::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        let current = cell_counter.get();
        cell_counter.set(current + 1);
    }
    let cell_time = start.elapsed();

    // RefCell<T>の場合
    let refcell_counter = RefCell::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1000000 {
        *refcell_counter.borrow_mut() += 1;
    }
    let refcell_time = start.elapsed();

    println!("Cell<T>: {:?}", cell_time);
    println!("RefCell<T>: {:?}", refcell_time);
    println!("Cell<T>の方が高速（借用チェックのオーバーヘッドなし）");

    println!("\n【メモリ使用量】");
    println!("Cell<T>: {}バイト", std::mem::size_of::<Cell<i32>>());
    println!("RefCell<T>: {}バイト", std::mem::size_of::<RefCell<i32>>());
    println!("RefCell<T>は借用状態を追跡するため若干大きい");

    println!("\n【機能比較表】");
    println!("┌─────────────┬─────────┬───────────┐");
    println!("│    機能     │ Cell<T> │ RefCell<T> │");
    println!("├─────────────┼─────────┼───────────┤");
    println!("│ Copyトレイト │ 必要    │ 不要       │");
    println!("│ 部分的変更   │ 不可    │ 可能       │");
    println!("│ 複数借用     │ N/A     │ 可能       │");
    println!("│ パニック     │ なし    │ あり       │");
    println!("│ パフォーマンス│ 高速    │ やや低速   │");
    println!("└─────────────┴─────────┴───────────┘");
}

// エラーハンドリングの例
fn error_handling_examples() {
    use std::cell::RefCell;

    let data = RefCell::new(vec![1, 2, 3]);

    println!("【借用ルール違反でパニックする例】");

    // 正常なケース
    {
        let borrowed = data.borrow();
        println!("正常な借用: {:?}", *borrowed);
    } // ここで借用が終了

    // try_borrow を使った安全な借用
    println!("\n【try_borrowを使った安全な借用】");

    let _borrow1 = data.borrow(); // 不変借用

    // 追加の不変借用（成功）
    match data.try_borrow() {
        Ok(borrow) => println!("追加の不変借用成功: {:?}", *borrow),
        Err(e) => println!("借用失敗: {:?}", e),
    }

    // 可変借用の試行（失敗）
    match data.try_borrow_mut() {
        Ok(_) => println!("可変借用成功"),
        Err(e) => println!("可変借用失敗: {:?}", e),
    }

    drop(_borrow1); // 明示的に借用を終了

    // 今度は可変借用が成功
    match data.try_borrow_mut() {
        Ok(mut borrow) => {
            borrow.push(4);
            println!("可変借用成功、追加後: {:?}", *borrow);
        }
        Err(e) => println!("可変借用失敗: {:?}", e),
    }

    println!("\n✓ try_borrow/try_borrow_mutを使用してパニックを回避");
    println!("✓ 借用の状態を事前にチェック可能");
    println!("✓ エラーハンドリングにより堅牢なコードが書ける");
}
