//! 第5章：マクロとメタプログラミング
//!
//! このプログラムは、Rustのマクロシステムの概要を示します。
//! 各サンプルプログラムを個別に実行して、詳細を学習してください。

use std::collections::HashMap;

// 簡単な宣言的マクロの例
macro_rules! say_hello {
    () => {
        println!("Hello, Macro World!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// より複雑なマクロの例：HashMapを簡単に作成
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

// デバッグ用マクロ
macro_rules! debug_print {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            eprintln!(
                "[{}:{}] {} = {:?}",
                file!(),
                line!(),
                stringify!($val),
                $val
            );
        }
    };
}

fn main() {
    println!("第5章：マクロとメタプログラミング");
    println!("==================================\n");

    // 基本的なマクロの使用
    say_hello!();
    say_hello!("Rust");

    // HashMapマクロの使用
    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 87,
        "Charlie" => 95
    };

    println!("\nスコア一覧:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // デバッグマクロの使用
    let x = 42;
    debug_print!(x);

    // const関数の例
    const FIBONACCI_10: u32 = const_fibonacci(10);
    println!("\nフィボナッチ数列の10番目: {}", FIBONACCI_10);

    println!("\n各サンプルプログラムを実行してみてください:");
    println!("- cargo run --bin declarative_macros");
    println!("- cargo run --bin macro_patterns");
    println!("- cargo run --bin recursive_macros");
    println!("- cargo run --bin const_functions");
    println!("- cargo run --bin macro_hygiene");
    println!("- cargo run --bin debugging_macros");
    println!("- cargo run --bin practical_macros");

    println!("\n復習問題:");
    println!("- cargo run --bin problem1_custom_assert");
    println!("- cargo run --bin problem2_builder_macro");
    println!("- cargo run --bin problem3_type_safe_units");
}

// コンパイル時に計算されるフィボナッチ関数
const fn const_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => const_fibonacci(n - 1) + const_fibonacci(n - 2),
    }
}
