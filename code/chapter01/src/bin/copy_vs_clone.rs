// src/bin/copy_vs_clone.rs

use std::mem;

// Copyトレイトを実装できる型（すべてのフィールドがCopy）
#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

// Copyトレイトを実装できない型（StringはCopyではない）
#[derive(Debug, Clone)]
struct NamedPoint {
    name: String,
    position: Point2D,
}

fn main() {
    println!("=== Copy vs Clone ===\n");

    // Copyトレイトの動作
    println!("--- Copyトレイト ---");
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = p1; // 暗黙的にコピー
    println!("p1: {:?}", p1); // p1はまだ使える
    println!("p2: {:?}", p2);
    println!("p1とp2は別のメモリ: p1={:p}, p2={:p}", &p1, &p2);
    println!("p1の座標: ({}, {})", p1.x, p1.y);

    // Cloneトレイトの動作
    println!("\n--- Cloneトレイト ---");
    let np1 = NamedPoint {
        name: String::from("Origin"),
        position: Point2D { x: 0.0, y: 0.0 },
    };
    let np2 = np1.clone(); // 明示的にクローン
    println!("np1: {:?}", np1); // クローン後も使える
    println!("np2: {:?}", np2);
    println!("np1のポイント名: {}", np1.name);
    println!("np1の位置: ({}, {})", np1.position.x, np1.position.y);

    // メモリ使用量の比較
    println!("\n--- メモリ使用量 ---");
    println!("Point2Dのサイズ: {}バイト", mem::size_of::<Point2D>());
    println!("NamedPointのサイズ: {}バイト", mem::size_of::<NamedPoint>());

    // Copyの制約
    demonstrate_copy_constraints();
}

fn demonstrate_copy_constraints() {
    println!("\n--- Copyトレイトの制約 ---");

    // Copyを実装する標準型
    let primitives = (42i32, std::f64::consts::PI, true, 'R');
    let copy = primitives; // すべてCopyなのでタプルもCopy
    println!("元のタプル: {:?}", primitives);
    println!("コピー: {:?}", copy);

    // 配列とCopy
    let arr1 = [1, 2, 3, 4, 5]; // i32はCopyなので配列もCopy
    let arr2 = arr1;
    println!("\n配列もCopy可能: {:?}", arr1);
    println!("コピーされた配列: {:?}", arr2);

    // Option<T>とCopy
    let opt1: Option<i32> = Some(42);
    let opt2 = opt1; // i32はCopyなのでOption<i32>もCopy
    println!("\nOption<i32>もCopy可能: {:?}, {:?}", opt1, opt2);
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
