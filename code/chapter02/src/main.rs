// src/main.rs
fn main() {
    println!("=== Rust 第2章：ライフタイムの深層理解 ===");
    println!("\n各サンプルプログラムを実行するには以下のコマンドを使用してください：");
    println!("cargo run --bin lifetime_annotations");
    println!("cargo run --bin lifetime_elision");
    println!("cargo run --bin hrtb_examples");
    println!("cargo run --bin variance_examples");
    println!("cargo run --bin static_lifetime");
    println!("cargo run --bin practical_lifetimes");
    println!("cargo run --bin complex_lifetimes");
    println!("cargo run --bin lifetime_generics");
    println!("cargo run --bin lifetime_subtyping");
    
    demonstrate_basic_lifetime();
    demonstrate_lifetime_bounds();
}

fn demonstrate_basic_lifetime() {
    println!("\n=== ライフタイムの基本概念 ===");
    
    // ライフタイムはコンパイル時の概念
    let string1 = String::from("長い文字列です");
    let result;
    
    {
        let string2 = String::from("短い");
        // longest関数は両方の参照が有効な間だけ結果を返せる
        result = longest(string1.as_str(), string2.as_str());
        println!("より長い文字列: {}", result);
    } // string2はここでドロップされる
    
    // println!("結果: {}", result); // エラー: string2のライフタイムが終了
}

// ライフタイム注釈が必要な関数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 戻り値のライフタイムは、xとyの短い方に制限される
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demonstrate_lifetime_bounds() {
    println!("\n=== ライフタイム境界 ===");
    
    // 構造体のライフタイム
    let novel = String::from("むかしむかし、あるところに...");
    let first_sentence = novel.split('、').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("抜粋: {}", excerpt.part);
    
    // ジェネリックとライフタイム
    let number_list = vec![1, 2, 3, 4, 5];
    let largest = find_largest(&number_list);
    println!("最大値: {}", largest);
}

// ライフタイムを持つ構造体
struct Excerpt<'a> {
    part: &'a str,
}

// ライフタイムとジェネリックの組み合わせ
fn find_largest<'a, T>(list: &'a [T]) -> &'a T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
