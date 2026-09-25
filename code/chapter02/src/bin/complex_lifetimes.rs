// src/bin/complex_lifetimes.rs

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== 複雑なライフタイムパターン ===\n");

    arena_pattern();
    cyclic_data_structures();
    lifetime_intersection();
    higher_order_lifetimes();
    async_lifetime_concepts();
}

fn arena_pattern() {
    println!("--- アリーナパターン ---");

    // アリーナパターンの実装
    demonstrate_arena_pattern();
}

fn demonstrate_arena_pattern() {
    println!("\n[アリーナパターンの実装]");

    // 簡単なアリーナ実装
    struct Arena<T> {
        storage: Vec<T>,
    }

    impl<T> Arena<T> {
        fn new() -> Self {
            Arena {
                storage: Vec::new(),
            }
        }

        fn alloc(&mut self, value: T) -> usize {
            let index = self.storage.len();
            self.storage.push(value);
            index
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.storage.get(index)
        }
    }

    // ツリーノード（インデックスベース）
    struct TreeNode {
        value: i32,
        children: Vec<usize>,
    }

    let mut arena = Arena::new();

    // アリーナパターンでは、先にすべてのノードを作成してから関係を構築
    let leaf1_idx = arena.alloc(TreeNode {
        value: 1,
        children: vec![],
    });

    let leaf2_idx = arena.alloc(TreeNode {
        value: 2,
        children: vec![],
    });

    let root_idx = arena.alloc(TreeNode {
        value: 0,
        children: vec![leaf1_idx, leaf2_idx],
    });

    // より安全なアプローチ：インデックスベースの参照
    if let Some(leaf1) = arena.get(leaf1_idx) {
        println!("リーフ1の値: {}", leaf1.value);
    }
    if let Some(leaf2) = arena.get(leaf2_idx) {
        println!("リーフ2の値: {}", leaf2.value);
    }
    if let Some(root) = arena.get(root_idx) {
        println!("ルートの値: {}", root.value);
        println!("子ノード数: {}", root.children.len());
    }
}

fn cyclic_data_structures() {
    println!("\n--- 循環データ構造 ---");

    // Rc/Weakを使った循環構造
    demonstrate_rc_weak_cycle();

    // RefCellを使った循環構造
    demonstrate_refcell_cycle();
}

fn demonstrate_rc_weak_cycle() {
    println!("\n[Rc/Weakによる循環構造]");

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }

        fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
            parent.children.borrow_mut().push(child.clone());
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }
    }

    let parent = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&parent, &child1);
    Node::add_child(&parent, &child2);

    println!("親ノード: {}", parent.value);
    println!("子ノード数: {}", parent.children.borrow().len());

    // 親への参照を確認
    if let Some(parent_ref) = child1.parent.borrow().upgrade() {
        println!("子1の親: {}", parent_ref.value);
    };
}

fn demonstrate_refcell_cycle() {
    println!("\n[RefCellによる循環構造]");

    #[derive(Debug)]
    struct RefNode {
        value: i32,
        next: RefCell<Option<Rc<RefNode>>>,
    }

    impl RefNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(RefNode {
                value,
                next: RefCell::new(None),
            })
        }
    }

    let node1 = RefNode::new(1);
    let node2 = RefNode::new(2);
    let node3 = RefNode::new(3);

    // 循環リストを作成
    *node1.next.borrow_mut() = Some(node2.clone());
    *node2.next.borrow_mut() = Some(node3.clone());
    *node3.next.borrow_mut() = Some(node1.clone());

    println!("ノード1: {}", node1.value);
    if let Some(ref next) = *node1.next.borrow() {
        println!("ノード1の次: {}", next.value);
    };
}

fn lifetime_intersection() {
    println!("\n--- ライフタイムの交差 ---");

    // ライフタイムの最小値
    demonstrate_lifetime_minimum();

    // ライフタイムの制約
    demonstrate_lifetime_constraints();
}

fn demonstrate_lifetime_minimum() {
    println!("\n[ライフタイムの最小値]");

    // 複数の参照の最小ライフタイム
    fn choose_shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() < y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("short");
        result = choose_shorter(&string1, &string2);
        println!("短い方: {}", result);
        // resultはここでしか使えない
    }
    // println!("外側: {}", result); // エラー！
}

fn demonstrate_lifetime_constraints() {
    println!("\n[ライフタイムの制約]");

    // ライフタイム制約の例
    struct Container<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }

    impl<'a, 'b> Container<'a, 'b> {
        fn new(first: &'a str, second: &'b str) -> Self {
            Container { first, second }
        }

        // 'a: 'b を要求する関数
        fn get_first_if_longer(&self) -> Option<&'a str>
        where
            'a: 'b, // 'aは'bより長生きする必要がある
        {
            if self.first.len() > self.second.len() {
                Some(self.first)
            } else {
                None
            }
        }
    }

    let long_lived = "長寿命の文字列";
    {
        let short_lived = String::from("短寿命");
        let container = Container::new(long_lived, &short_lived);

        if let Some(longer) = container.get_first_if_longer() {
            println!("長い方: {}", longer);
        }
    }
}

fn higher_order_lifetimes() {
    println!("\n--- 高階ライフタイム ---");

    // ライフタイムの抽象化
    demonstrate_lifetime_abstraction();

    // ライフタイム変換
    demonstrate_lifetime_conversion();
}

fn demonstrate_lifetime_abstraction() {
    println!("\n[ライフタイムの抽象化]");

    // 高階ライフタイム境界
    fn apply_to_all<F>(items: &[&str], f: F) -> Vec<String>
    where
        F: for<'a> Fn(&'a str) -> String,
    {
        items.iter().map(|&item| f(item)).collect()
    }

    let items = vec!["apple", "banana", "cherry"];
    let results = apply_to_all(&items, |s| s.to_uppercase());
    println!("変換結果: {:?}", results);

    // より複雑な高階境界
    fn complex_transform<F, G>(data: &str, f: F, g: G) -> String
    where
        F: for<'a> Fn(&'a str) -> &'a str,
        G: for<'a> Fn(&'a str) -> String,
    {
        let intermediate = f(data);
        g(intermediate)
    }

    let input = "hello world";
    let result = complex_transform(input, |s| s.trim(), |s| s.to_uppercase());
    println!("複雑な変換: {}", result);
}

fn demonstrate_lifetime_conversion() {
    println!("\n[ライフタイム変換]");

    // ライフタイムの縮小
    fn shorten_lifetime<'a, 'b>(long: &'a str) -> &'b str
    where
        'a: 'b,
    {
        long
    }

    let long_string = String::from("長い文字列");
    {
        let shortened = shorten_lifetime(&long_string);
        println!("短縮されたライフタイム: {}", shortened);
    }

    // ライフタイムの統合
    fn merge_lifetimes<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
        (x, y)
    }

    let s1 = "文字列1";
    let s2 = "文字列2";
    let (merged1, merged2) = merge_lifetimes(s1, s2);
    println!("統合されたライフタイム: {}, {}", merged1, merged2);
}

fn async_lifetime_concepts() {
    println!("\n--- 非同期コンテキストでのライフタイム ---");

    // 非同期関数のライフタイム概念
    demonstrate_async_lifetime_concepts();

    // Futureとライフタイム
    demonstrate_future_lifetimes();
}

fn demonstrate_async_lifetime_concepts() {
    println!("\n[非同期関数のライフタイム概念]");

    // 非同期関数のシミュレーション
    struct AsyncProcessor;

    impl AsyncProcessor {
        // 実際のasyncではないが、概念を示す
        fn process_data(&self, data: &str) -> String {
            // 実際の非同期処理では、以下のような制約がある：
            // - 参照は.awaitポイントを跨げない
            // - 'staticでない参照は特別な扱いが必要
            format!("処理済み: {}", data)
        }

        // 'static境界が必要な場合のシミュレーション
        fn process_owned(&self, data: String) -> String {
            format!("所有データ処理: {}", data)
        }
    }

    let processor = AsyncProcessor;

    // 参照を使った処理
    let data = String::from("async data");
    let result = processor.process_data(&data);
    println!("非同期風処理: {}", result);

    // 所有データを使った処理
    let owned_data = String::from("owned async data");
    let result = processor.process_owned(owned_data);
    println!("所有データ処理: {}", result);
}

fn demonstrate_future_lifetimes() {
    println!("\n[Futureとライフタイム]");

    // Futureのライフタイム制約をシミュレート
    struct FutureSimulator<'a> {
        data: &'a str,
    }

    impl<'a> FutureSimulator<'a> {
        fn new(data: &'a str) -> Self {
            FutureSimulator { data }
        }

        // 実際のFutureでは、pollメソッドでライフタイム制約が重要
        fn simulate_poll(&self) -> String {
            format!("Future完了: {}", self.data)
        }
    }

    let data = String::from("future data");
    let future_sim = FutureSimulator::new(&data);
    let result = future_sim.simulate_poll();
    println!("{}", result);

    // 'static境界が必要な場合
    fn requires_static_future<F>(f: F) -> String
    where
        F: FnOnce() -> String + 'static,
    {
        f()
    }

    let static_data = "static future data";
    let result = requires_static_future(move || format!("静的Future: {}", static_data));
    println!("{}", result);
}
