// src/bin/pointer_performance.rs

use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    println!("=== スマートポインタのパフォーマンス比較 ===\n");

    const ITERATIONS: usize = 1_000_000;

    // Box<T>のパフォーマンス
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _b = Box::new(42);
    }
    println!("Box作成時間: {:?}", start.elapsed());

    // Rc<T>のパフォーマンス
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _r = Rc::new(42);
    }
    println!("Rc作成時間: {:?}", start.elapsed());

    // Arc<T>のパフォーマンス
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _a = Arc::new(42);
    }
    println!("Arc作成時間: {:?}", start.elapsed());

    // クローンのパフォーマンス比較
    println!("\n--- クローンのパフォーマンス ---");

    let rc = Rc::new(vec![0; 1000]);
    let arc = Arc::new(vec![0; 1000]);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _clone = Rc::clone(&rc);
    }
    println!("Rcクローン時間: {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _clone = Arc::clone(&arc);
    }
    println!("Arcクローン時間: {:?}", start.elapsed());

    // メモリ使用量の比較
    println!("\n--- メモリ使用量 ---");
    println!(
        "Box<i32>のサイズ: {}バイト",
        std::mem::size_of::<Box<i32>>()
    );
    println!("Rc<i32>のサイズ: {}バイト", std::mem::size_of::<Rc<i32>>());
    println!(
        "Arc<i32>のサイズ: {}バイト",
        std::mem::size_of::<Arc<i32>>()
    );
}
