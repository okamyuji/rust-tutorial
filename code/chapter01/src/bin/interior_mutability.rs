// src/bin/interior_mutability.rs

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    println!("=== 内部可変性パターン ===\n");
    
    cell_example();
    refcell_example();
    practical_example();
}

fn cell_example() {
    println!("--- Cell<T>: Copyな型の内部可変性 ---");
    
    struct Counter {
        count: Cell<u32>,
        name: String,
    }
    
    impl Counter {
        fn new(name: String) -> Self {
            Counter {
                count: Cell::new(0),
                name,
            }
        }
        
        // &selfでも内部の値を変更可能
        fn increment(&self) {
            self.count.set(self.count.get() + 1);
        }
        
        fn get(&self) -> u32 {
            self.count.get()
        }
    }
    
    let counter = Counter::new(String::from("ページビュー"));
    
    // 不変参照でもカウントアップ可能
    for _ in 0..5 {
        counter.increment();
    }
    
    println!("{}: {}", counter.name, counter.get());
}

fn refcell_example() {
    println!("\n--- RefCell<T>: 実行時借用チェック ---");
    
    #[derive(Debug)]
    struct LogBuffer {
        messages: RefCell<Vec<String>>,
    }
    
    impl LogBuffer {
        fn new() -> Self {
            LogBuffer {
                messages: RefCell::new(Vec::new()),
            }
        }
        
        // &selfでもメッセージを追加可能
        fn log(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }
        
        fn print_all(&self) {
            for (i, msg) in self.messages.borrow().iter().enumerate() {
                println!("  [{}] {}", i, msg);
            }
        }
        
        // 借用ルール違反の例（安全のためコメントアウト）
        fn demonstrate_borrow_rules(&self) {
            let borrow1 = self.messages.borrow();
            println!("メッセージ数: {}", borrow1.len());
            drop(borrow1); // 借用を明示的に終了
            
            // これならOK
            let mut borrow2 = self.messages.borrow_mut();
            borrow2.push(String::from("借用ルールのデモ"));
        }
    }
    
    let logger = LogBuffer::new();
    logger.log("システム起動");
    logger.log("初期化完了");
    logger.log("処理開始");
    
    println!("ログ内容:");
    logger.print_all();
    
    logger.demonstrate_borrow_rules();
    println!("\n借用ルールデモ後:");
    logger.print_all();
}

fn practical_example() {
    println!("\n--- 実践例: グラフ構造 ---");
    
    // 循環参照を持つグラフノード
    #[derive(Debug)]
    struct Node {
        value: i32,
        edges: RefCell<Vec<Rc<Node>>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                edges: RefCell::new(Vec::new()),
            })
        }
        
        fn add_edge(self: &Rc<Self>, other: &Rc<Node>) {
            self.edges.borrow_mut().push(Rc::clone(other));
        }
    }
    
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    
    // グラフの構築（不変のRc参照でもエッジを追加可能）
    node1.add_edge(&node2);
    node1.add_edge(&node3);
    node2.add_edge(&node3);
    
    println!("ノード1の接続数: {}", node1.edges.borrow().len());
    println!("ノード1の参照カウント: {}", Rc::strong_count(&node1));
    println!("ノード1の値: {}", node1.value);
}
