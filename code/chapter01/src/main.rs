// src/main.rs
fn main() {
    println!("=== Rust 第1章：所有権システムとメモリ管理 ===");
    println!("\n各サンプルプログラムを実行するには以下のコマンドを使用してください：");
    println!("cargo run --bin memory_layout");
    println!("cargo run --bin move_semantics");
    println!("cargo run --bin copy_vs_clone");
    println!("cargo run --bin borrowing_basics");
    println!("cargo run --bin advanced_borrowing");
    println!("cargo run --bin interior_mutability");
    println!("cargo run --bin mutex_example");
    println!("cargo run --bin smart_pointers");
    println!("cargo run --bin pointer_performance");

    demonstrate_ownership_rules();
    demonstrate_scope_and_drop();
}

fn demonstrate_ownership_rules() {
    println!("\n=== 所有権の規則デモ ===");

    // 規則1: 各値には所有者（owner）と呼ばれる変数が1つだけ存在する
    let s1 = String::from("hello");
    println!("s1の所有者: s1変数");

    // 規則2: 所有者は同時に1つしか存在できない
    let s2 = s1; // 所有権の移動（ムーブ）
                 // println!("{}", s1); // コンパイルエラー: value borrowed here after move
    println!("所有権がs1からs2に移動: {}", s2);

    // なぜムーブが必要か？
    // String型はヒープにデータを持つ。もしs1とs2が同じヒープデータを
    // 指していたら、両方がスコープを抜けるときに二重解放が発生する
}

fn demonstrate_scope_and_drop() {
    println!("\n=== スコープとドロップ ===");

    // 規則3: 所有者がスコープを抜けると値は破棄される
    {
        let s = String::from("scoped string");
        println!("スコープ内: {}", s);

        // カスタムドロップの実装例
        struct Verbose {
            name: String,
        }

        impl Drop for Verbose {
            fn drop(&mut self) {
                println!("{}がドロップされました", self.name);
            }
        }

        let _v = Verbose {
            name: String::from("verbose object"),
        };
    } // ここでsとvが自動的にドロップされる

    println!("スコープ外: メモリは解放済み");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
