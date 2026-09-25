//! マクロの衛生性（Hygiene）
//!
//! Rustのマクロがどのように変数のスコープと名前の衝突を防ぐかを示します。

// 基本的な衛生性の例
macro_rules! hygienic_let {
    ($e:expr) => {
        {
            #[allow(unused_variables)]
            let x = 42;  // マクロ内のx（呼び出し元の x とは別物なので使われない）
            $e          // 呼び出し元の式を評価
        }
    };
}

// 非衛生的な識別子の導入（$nameを使用）
macro_rules! create_variable {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

// スコープの分離を示すマクロ
macro_rules! isolated_scope {
    () => {
        let hidden = "マクロ内だけの値";
        println!("マクロ内: {}", hidden);
    };
}

// ラベルの衛生性
macro_rules! loop_with_label {
    ($body:block) => {
        'macro_loop: loop {
            $body
            break 'macro_loop;
        }
    };
}

// 型の衛生性
macro_rules! type_alias {
    () => {
        type MyType = i32;
        fn use_type() -> MyType {
            42
        }
    };
}

// パスの衛生性
macro_rules! use_std {
    () => {
        use std::collections::HashMap as Map;
        let _map: Map<i32, i32> = Map::new();
    };
}

// 衛生的なジェネリクス
macro_rules! generic_function {
    ($name:ident) => {
        fn $name<T: std::fmt::Display>(value: T) {
            println!("値: {}", value);
        }
    };
}

// マクロ内でのマクロ定義
macro_rules! define_inner_macro {
    () => {
        macro_rules! inner {
            () => {
                println!("内部マクロが呼ばれました");
            };
        }
        inner!();
    };
}

// 識別子の意図的な汚染（unhygienic）
macro_rules! unhygienic {
    ($name:ident) => {
        let $name = 100;
    };
}

// クロージャとマクロの相互作用
macro_rules! create_closure {
    ($capture:expr) => {{
        let x = 10; // マクロ内のx
        move || $capture + x // $captureは呼び出し元のスコープ
    }};
}

// ライフタイムの衛生性
macro_rules! lifetime_macro {
    ($t:ty) => {
        fn macro_function(x: &$t) -> &$t {
            x
        }
    };
}

// モジュールスコープとマクロ
macro_rules! module_macro {
    () => {
        mod macro_module {
            pub fn module_fn() -> &'static str {
                "マクロで生成されたモジュール"
            }
        }
    };
}

// $crateによるパスの解決
macro_rules! use_crate_path {
    () => {
        // $crateはマクロが定義されたクレートを指す
        fn crate_function() {
            println!("クレートパスを使用");
        }
    };
}

fn main() {
    println!("=== マクロの衛生性 ===\n");

    // 基本的な衛生性
    println!("--- 基本的な衛生性 ---");
    let x = 10;
    let result = hygienic_let!(x * 2); // 呼び出し元のx（10）が使われる
    println!("結果: {} (マクロ内のxではなく、外側のxが使われる)", result);

    // 非衛生的な識別子
    println!("\n--- 非衛生的な識別子 ---");
    create_variable!(my_var, 123);
    println!("作成された変数: {}", my_var);

    // スコープの分離
    println!("\n--- スコープの分離 ---");
    isolated_scope!();
    // println!("{}", hidden);  // エラー：hiddenは見えない

    // ラベルの衛生性
    println!("\n--- ラベルの衛生性 ---");
    'outer_block: {
        loop_with_label!({
            println!("マクロ内のループ");
            // break 'macro_loop;  // エラー：'macro_loopは見えない
        });
        println!("外側のブロック");
        break 'outer_block;
    }

    // 型の衛生性
    println!("\n--- 型の衛生性 ---");
    type_alias!();
    // let value: MyType = 42;  // エラー：MyTypeは見えない

    // パスの衛生性
    println!("\n--- パスの衛生性 ---");
    use_std!();
    // let map: Map<i32, i32> = Map::new();  // エラー：Mapは見えない

    // ジェネリック関数の生成
    println!("\n--- ジェネリック関数 ---");
    generic_function!(print_value);
    print_value(42);
    print_value("Hello");

    // マクロ内マクロ
    println!("\n--- マクロ内マクロ ---");
    define_inner_macro!();
    // inner!();  // エラー：innerは見えない

    // 意図的な汚染
    println!("\n--- 意図的な汚染 ---");
    unhygienic!(contaminated);
    println!("汚染された変数: {}", contaminated);

    // クロージャとの相互作用
    println!("\n--- クロージャとマクロ ---");
    let x = 5;
    let closure = create_closure!(x);
    println!("クロージャの結果: {}", closure());

    // ライフタイムの衛生性
    println!("\n--- ライフタイム ---");
    lifetime_macro!(str);
    let s = "Hello";
    let result = macro_function(s);
    println!("ライフタイム付き関数: {}", result);

    // モジュールの生成
    println!("\n--- モジュール生成 ---");
    module_macro!();
    println!("{}", macro_module::module_fn());

    // マクロのベストプラクティス
    println!("\n--- ベストプラクティス ---");
    println!("1. 予期しない名前の衝突を避けるため、衛生性に依存する");
    println!("2. 意図的に識別子を導入する場合は、明確にドキュメント化する");
    println!("3. $crateを使用して、マクロ内でのパス解決を確実にする");
    println!("4. マクロの展開結果を理解するため、cargo expandを活用する");

    println!("\nマクロの衛生性を理解しました！");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
