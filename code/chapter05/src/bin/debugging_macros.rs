//! マクロのデバッグテクニック
//! 
//! マクロ開発時のデバッグ方法とトラブルシューティングを示します。

// デバッグ用のヘルパーマクロ
macro_rules! debug_macro {
    ($($tokens:tt)*) => {
        {
            #[cfg(debug_assertions)]
            eprintln!("[DEBUG MACRO] Input: {}", stringify!($($tokens)*));
            
            $($tokens)*
        }
    };
}

// トークンの表示
macro_rules! show_tokens {
    ($($tokens:tt)*) => {
        concat!("Tokens: ", $(stringify!($tokens), " "),*)
    };
}

// 段階的なマクロ展開の追跡
macro_rules! trace_expansion {
    (@step 1, $x:expr) => {
        {
            eprintln!("Step 1: Processing expression");
            eprintln!("  Expression: {}", stringify!($x));
            trace_expansion!(@step 2, $x * 2)
        }
    };
    (@step 2, $x:expr) => {
        {
            eprintln!("Step 2: Doubling");
            eprintln!("  Expression: {}", stringify!($x));
            trace_expansion!(@step 3, $x + 10)
        }
    };
    (@step 3, $x:expr) => {
        {
            eprintln!("Step 3: Adding 10");
            eprintln!("  Final expression: {}", stringify!($x));
            $x
        }
    };
    ($x:expr) => {
        {
            eprintln!("=== Trace Expansion Start ===");
            let result = trace_expansion!(@step 1, $x);
            eprintln!("=== Trace Expansion End ===");
            result
        }
    };
}

// パターンマッチのデバッグ
macro_rules! debug_patterns {
    // ケース1: 単一の識別子
    ($id:ident) => {
        eprintln!("Pattern matched: single identifier '{}'", stringify!($id));
    };
    // ケース2: 型
    // expr を先に置くと Vec<i32> を式として解析し始めて失敗し、次の分岐へ戻らずエラーになる
    ($t:ty) => {
        eprintln!("Pattern matched: type '{}'", stringify!($t));
    };
    // ケース3: 式
    ($expr:expr) => {
        eprintln!("Pattern matched: expression '{}'", stringify!($expr));
    };
    // デフォルトケース
    ($($tokens:tt)*) => {
        eprintln!("Pattern matched: tokens '{}'", stringify!($($tokens)*));
    };
}

// コンパイルエラーの意図的な生成
macro_rules! compile_error_demo {
    (valid) => {
        println!("Valid pattern");
    };
    ($($other:tt)*) => {
        compile_error!(concat!("Invalid pattern: ", stringify!($($other)*)));
    };
}

// 型チェックのデバッグ
macro_rules! type_debug {
    ($expr:expr) => {
        {
            let value = $expr;
            eprintln!("Type of '{}': {}", 
                stringify!($expr), 
                std::any::type_name_of_val(&value)
            );
            value
        }
    };
}

// マクロ内での条件付きコンパイル
macro_rules! conditional_debug {
    ($($body:tt)*) => {
        {
            #[cfg(feature = "macro-debug")]
            eprintln!("Macro debug enabled");
            
            #[cfg(not(feature = "macro-debug"))]
            eprintln!("Macro debug disabled");
            
            $($body)*
        }
    };
}

// 再帰的展開の深さ制限チェック
macro_rules! check_recursion_limit {
    (@count ) => { 0 };
    (@count $x:tt $($xs:tt)*) => { 1 + check_recursion_limit!(@count $($xs)*) };
    
    ($($tokens:tt)*) => {
        {
            const DEPTH: usize = check_recursion_limit!(@count $($tokens)*);
            // compile_error! は if の中でも展開時に必ず発火するため、const 評価の assert で上限超過時だけ止める
            const _: () = assert!(DEPTH <= 10, "Recursion limit exceeded!");
            eprintln!("Recursion depth: {}", DEPTH);
        }
    };
}

// フラグメント指定子の確認
macro_rules! fragment_types {
    ($e:expr) => { eprintln!("Expression: {}", stringify!($e)); };
    ($i:ident) => { eprintln!("Identifier: {}", stringify!($i)); };
    ($t:ty) => { eprintln!("Type: {}", stringify!($t)); };
    ($p:pat) => { eprintln!("Pattern: {}", stringify!($p)); };
    ($s:stmt) => { eprintln!("Statement: {}", stringify!($s)); };
    ($b:block) => { eprintln!("Block: {}", stringify!($b)); };
    ($m:meta) => { eprintln!("Meta: {}", stringify!($m)); };
    ($tt:tt) => { eprintln!("Token tree: {}", stringify!($tt)); };
}

// マクロ展開の可視化
macro_rules! visualize_expansion {
    (
        input: $input:expr,
        process: $process:ident,
        output: $output:ident
    ) => {
        {
            eprintln!("┌─ Macro Expansion ─┐");
            eprintln!("│ Input:  {:?}      │", $input);
            eprintln!("│ Process: {}       │", stringify!($process));
            eprintln!("│ Output: {}        │", stringify!($output));
            eprintln!("└───────────────────┘");
            
            let $output = $process($input);
            $output
        }
    };
}

// エラー境界のテスト
macro_rules! test_error_handling {
    (ok: $expr:expr) => {
        match $expr {
            Ok(val) => eprintln!("Success: {:?}", val),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    };
    (panic: $expr:expr) => {
        std::panic::catch_unwind(|| {
            $expr
        }).unwrap_or_else(|_| eprintln!("Panic caught!"));
    };
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("=== マクロのデバッグテクニック ===\n");

    // 基本的なデバッグ
    println!("--- 基本的なデバッグ ---");
    let result = debug_macro!(2 + 2);
    println!("結果: {}", result);

    // トークンの表示
    println!("\n--- トークンの表示 ---");
    let tokens = show_tokens!(hello world 123 + - *);
    println!("{}", tokens);

    // 展開の追跡
    println!("\n--- 展開の追跡 ---");
    let traced = trace_expansion!(5);
    println!("最終結果: {}", traced);

    // パターンマッチのデバッグ
    println!("\n--- パターンマッチ ---");
    debug_patterns!(identifier);
    debug_patterns!(2 + 2);
    debug_patterns!(Vec<i32>);
    debug_patterns!(random tokens here);

    // コンパイルエラーのデモ（コメントアウト）
    println!("\n--- コンパイルエラー ---");
    compile_error_demo!(valid);
    // compile_error_demo!(invalid);  // これはコンパイルエラーを生成

    // 型デバッグ
    println!("\n--- 型デバッグ ---");
    let _: i32 = type_debug!(42);
    let _: String = type_debug!(String::from("Hello"));
    let _: Vec<i32> = type_debug!(vec![1, 2, 3]);

    // 条件付きデバッグ
    println!("\n--- 条件付きデバッグ ---");
    conditional_debug!({
        println!("本体の実行");
    });

    // 再帰深さチェック
    println!("\n--- 再帰深さチェック ---");
    check_recursion_limit!(a b c d e);
    // check_recursion_limit!(a b c d e f g h i j k l);  // エラー

    // マクロ展開の可視化
    println!("\n--- 展開の可視化 ---");
    let result = visualize_expansion! {
        input: 21,
        process: double,
        output: doubled_value
    };
    println!("Doubled value: {}", result);

    // エラーハンドリングのテスト
    println!("\n--- エラーハンドリング ---");
    test_error_handling!(ok: Ok::<i32, &str>(42));
    test_error_handling!(ok: Err::<i32, &str>("Error occurred"));

    // デバッグのベストプラクティス
    println!("\n--- デバッグのベストプラクティス ---");
    println!("1. cargo expand を使用してマクロ展開を確認");
    println!("2. trace_macros!(true); を使用して展開を追跡");
    println!("3. stringify! を使用してトークンを文字列化");
    println!("4. 段階的な開発とテスト");
    println!("5. compile_error! で明確なエラーメッセージ");
    
    // trace_macros! は nightly 専用のため stable では使えない。
    // 展開結果は `cargo +nightly rustc --bin debugging_macros -- -Z trace-macros` や cargo-expand で確かめる

    println!("\nマクロのデバッグ方法をマスターしました！");
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_patterns_accepts_types_and_recursion_check_passes_under_limit() {
        debug_patterns!(Vec<i32>);
        debug_patterns!(2 + 2);
        check_recursion_limit!(a b c d e f g h i j);
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
