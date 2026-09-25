//! 宣言的マクロ（macro_rules!）の基礎
//! 
//! このプログラムは、Rustの宣言的マクロの基本的な使い方を示します。

// 基本的なマクロ：引数なし
macro_rules! create_function {
    () => {
        fn generated_function() {
            println!("マクロによって生成された関数です！");
        }
    };
}

// パターンマッチングを使ったマクロ
macro_rules! test_pattern {
    // 単一の式
    ($e:expr) => {
        println!("式: {}", $e);
    };
    // 2つの式
    ($e1:expr, $e2:expr) => {
        println!("式1: {}, 式2: {}", $e1, $e2);
    };
}

// 反復を使ったマクロ
macro_rules! vec_strs {
    (
        // 0個以上の式を受け取る
        $($element:expr),* $(,)?
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

// 異なるフラグメント指定子を使ったマクロ
macro_rules! create_struct {
    (
        $struct_name:ident {
            $($field_name:ident : $field_type:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(pub $field_name: $field_type),*
        }
    };
}

// トークンツリーを使った柔軟なマクロ
macro_rules! flexible_macro {
    ($($tokens:tt)*) => {
        println!("受け取ったトークン: {}", stringify!($($tokens)*));
    };
}

// ブロックを受け取るマクロ
macro_rules! with_timing {
    ($name:expr, $block:block) => {
        {
            use std::time::Instant;
            let start = Instant::now();
            let result = $block;
            let duration = start.elapsed();
            println!("{} の実行時間: {:?}", $name, duration);
            result
        }
    };
}

// 型の名前を表示するマクロ
macro_rules! print_type_of {
    ($val:expr) => {
        {
            let val = &$val;
            println!("{} の型: {}", stringify!($val), std::any::type_name_of_val(val));
        }
    };
}

fn main() {
    println!("=== 宣言的マクロの基礎 ===\n");

    // 関数生成マクロの使用
    create_function!();
    generated_function();

    println!("\n--- パターンマッチング ---");
    test_pattern!(42);
    test_pattern!("Hello", "World");

    println!("\n--- 反復パターン ---");
    let strings = vec_strs![1, 2, "three", 4.5];
    println!("生成されたベクター: {:?}", strings);

    println!("\n--- 構造体生成 ---");
    create_struct! {
        Person {
            name: String,
            age: u32,
            email: String,
        }
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    println!("生成された構造体: {:?}", person);

    println!("\n--- トークンツリー ---");
    flexible_macro!(これは 任意の トークン 1 + 2 * 3);

    println!("\n--- ブロック実行とタイミング ---");
    let result = with_timing!("重い計算", {
        // 合計は約 5×10^11 になり、既定の i32 では溢れる
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    });
    println!("計算結果: {}", result);

    println!("\n--- 型情報の表示 ---");
    let number = 42;
    let text = "Hello";
    let vec_data = vec![1, 2, 3];
    
    print_type_of!(number);
    print_type_of!(text);
    print_type_of!(vec_data);

    // マクロの入れ子
    println!("\n--- マクロの入れ子 ---");
    macro_rules! outer {
        ($($inner:tt)*) => {
            println!("外側のマクロ");
            inner!($($inner)*);
        };
    }

    macro_rules! inner {
        ($msg:expr) => {
            println!("内側のマクロ: {}", $msg);
        };
    }

    outer!("入れ子のメッセージ");

    // マクロのスコープとエクスポート
    println!("\n--- マクロのスコープ ---");
    {
        macro_rules! local_macro {
            () => {
                println!("ローカルスコープのマクロ");
            };
        }
        local_macro!();
    }
    // local_macro!(); // エラー：スコープ外

    println!("\n宣言的マクロの基本的な使い方を理解しました！");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
