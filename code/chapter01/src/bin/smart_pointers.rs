// src/bin/smart_pointers.rs

use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::cell::RefCell;
use std::thread;

fn main() {
    println!("=== スマートポインタの使い分け ===\n");
    
    box_use_cases();
    rc_use_cases();
    arc_use_cases();
    weak_references();
}

fn box_use_cases() {
    println!("--- Box<T>: 単一所有権のヒープ配置 ---");
    
    // ユースケース1: 再帰的データ構造
    #[derive(Debug)]
    enum BinaryTree {
        Node {
            value: i32,
            left: Box<BinaryTree>,
            right: Box<BinaryTree>,
        },
        Empty,
    }
    
    let tree = BinaryTree::Node {
        value: 10,
        left: Box::new(BinaryTree::Node {
            value: 5,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }),
        right: Box::new(BinaryTree::Node {
            value: 15,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }),
    };
    
    println!("二分木: {:?}", tree);
    
    // 木構造の使用例
    if let BinaryTree::Node { value, left, right } = &tree {
        println!("ルートノードの値: {}", value);
        
        // 左の子ノードの確認
        if let BinaryTree::Node { value: left_value, .. } = left.as_ref() {
            println!("左の子ノードの値: {}", left_value);
        }
        
        // 右の子ノードの確認
        if let BinaryTree::Node { value: right_value, .. } = right.as_ref() {
            println!("右の子ノードの値: {}", right_value);
        }
    }
    
    // ユースケース2: トレイトオブジェクト
    trait Draw {
        fn draw(&self);
    }
    
    struct Circle { radius: f64 }
    struct Rectangle { width: f64, height: f64 }
    
    impl Draw for Circle {
        fn draw(&self) {
            println!("円を描画（半径: {}）", self.radius);
        }
    }
    
    impl Draw for Rectangle {
        fn draw(&self) {
            println!("長方形を描画（{}x{}）", self.width, self.height);
        }
    }
    
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for shape in &shapes {
        shape.draw();
    }
}

fn rc_use_cases() {
    println!("\n--- Rc<T>: 単一スレッドでの共有所有権 ---");
    
    // ユースケース: 複数の所有者を持つデータ
    #[derive(Debug)]
    struct SharedData {
        value: String,
    }
    
    let data = Rc::new(SharedData {
        value: String::from("共有されるデータ"),
    });
    
    println!("共有データの値: {}", data.value);
    
    println!("初期参照カウント: {}", Rc::strong_count(&data));
    
    let _data2 = Rc::clone(&data);
    let _data3 = Rc::clone(&data);
    
    println!("3つの参照後: {}", Rc::strong_count(&data));
    
    {
        let _data4 = Rc::clone(&data);
        println!("スコープ内での参照: {}", Rc::strong_count(&data));
    }
    
    println!("スコープ後: {}", Rc::strong_count(&data));
}

fn arc_use_cases() {
    println!("\n--- Arc<T>: マルチスレッドでの共有所有権 ---");
    
    #[derive(Debug)]
    struct Config {
        server_name: String,
        port: u16,
    }
    
    let config = Arc::new(Config {
        server_name: String::from("localhost"),
        port: 8080,
    });
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            println!("スレッド{}: サーバー{}:{}", 
                     i, config.server_name, config.port);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最終参照カウント: {}", Arc::strong_count(&config));
}

fn weak_references() {
    println!("\n--- Weak<T>: 循環参照の防止 ---");
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leafの値: {}", leaf.value);
    println!("leafの子数: {}", leaf.children.borrow().len());
    
    println!("leafの強参照: {}, 弱参照: {}", 
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("branchの強参照: {}, 弱参照: {}", 
                 Rc::strong_count(&branch), Rc::weak_count(&branch));
        
        // 親への弱参照を通じてアクセス
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leafの親の値: {}", parent.value);
        }
    }
    
    // branchがドロップされた後
    println!("leafの親（ドロップ後）: {:?}", 
             leaf.parent.borrow().upgrade());
}
