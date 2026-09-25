//! 高度なマクロパターンマッチング
//! 
//! より複雑なパターンマッチングテクニックを示します。

use std::collections::HashMap;

// インクリメンタルなマッチング（TT Muncher）
macro_rules! count_tts {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count_tts!($($tail)*) };
}

// プッシュダウン累積
macro_rules! reverse {
    ([] $($reversed:tt)*) => {
        [$($reversed),*]
    };
    ([$head:tt $($tail:tt)*] $($reversed:tt)*) => {
        reverse!([$($tail)*] $head $($reversed)*)
    };
}

// コールバックパターン
macro_rules! with_callback {
    ($callback:ident, $($args:tt)*) => {
        $callback!($($args)*)
    };
}

macro_rules! my_callback {
    ($val:expr) => {
        println!("コールバックが呼ばれました: {}", $val);
    };
}

// 内部ルールパターン
macro_rules! impl_methods {
    (
        struct $name:ident {
            $($field:ident: $type:ty),* $(,)?
        }
        
        impl {
            $($methods:tt)*
        }
    ) => {
        struct $name {
            $($field: $type),*
        }
        
        impl $name {
            $($methods)*
        }
    };
}

// 可変長引数の処理
macro_rules! tuple_to_vec {
    (@accum () -> ($($accum:expr),*)) => {
        vec![$($accum),*]
    };
    (@accum ($head:expr $(, $tail:expr)*) -> ($($accum:expr),*)) => {
        tuple_to_vec!(@accum ($($tail),*) -> ($($accum,)* $head))
    };
    ($($x:expr),*) => {
        tuple_to_vec!(@accum ($($x),*) -> ())
    };
}

// キー・バリューペアのパース
macro_rules! parse_kvs {
    // エントリポイント
    (@parse $map:ident, ) => {};
    
    // キー・バリューペアのパース
    // 値の型が混在しても1つの HashMap に入るよう、文字列に揃えて格納する
    (@parse $map:ident, $key:literal : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($key, $value.to_string());
        parse_kvs!(@parse $map, $($($rest)*)?);
    };
    
    // メインマクロ
    ($($tokens:tt)*) => {{
        let mut map = HashMap::new();
        parse_kvs!(@parse map, $($tokens)*);
        map
    }};
}

// 条件付きコンパイル（実務では Rust 1.95 で安定化した std::cfg_select! が使える）
macro_rules! cfg_match {
    (
        $( #[cfg($meta:meta)] $arm:expr, )*
        _ => $default:expr $(,)?
    ) => {{
        // 式への属性は unstable なので、属性を付けられる let 文で分岐させる
        $(
            #[cfg($meta)]
            let selected = $arm;
        )*
        #[cfg(not(any($($meta),*)))]
        let selected = $default;
        selected
    }};
}

// デリゲートパターン
macro_rules! delegate {
    (
        $self:ident.$field:ident {
            $(fn $method:ident(&self $(, $arg:ident: $arg_ty:ty)*) -> $ret:ty;)*
        }
    ) => {
        $(
            fn $method(&self $(, $arg: $arg_ty)*) -> $ret {
                self.$field.$method($($arg),*)
            }
        )*
    };
}

// 型レベル計算
macro_rules! type_level_max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        type_level_max!(type_level_max!($a, $b), $($rest),+)
    };
}

// ジェネリクスを含むマクロ
macro_rules! generic_impl {
    (
        impl<$($generic:ident),*> $trait:ident for $type:ty {
            $($body:tt)*
        }
    ) => {
        impl<$($generic),*> $trait for $type {
            $($body)*
        }
    };
}

fn main() {
    println!("=== 高度なマクロパターンマッチング ===\n");

    // トークン数のカウント
    println!("--- トークンカウント ---");
    println!("トークン数: {}", count_tts!(a b c d e));
    
    // リバース
    println!("\n--- リバース ---");
    let reversed = reverse!([1 2 3 4 5]);
    println!("リバース結果: {:?}", reversed);

    // コールバック
    println!("\n--- コールバックパターン ---");
    with_callback!(my_callback, "Hello from callback!");

    // 構造体とメソッドの実装
    println!("\n--- 構造体とメソッド ---");
    impl_methods! {
        struct Point {
            x: f64,
            y: f64,
        }
        
        impl {
            fn new(x: f64, y: f64) -> Self {
                Self { x, y }
            }
            
            fn distance(&self) -> f64 {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        }
    }

    let point = Point::new(3.0, 4.0);
    println!("点の距離: {}", point.distance());

    // タプルからベクタ
    println!("\n--- タプルからベクタ ---");
    let vec = tuple_to_vec!(1, 2, 3, 4, 5);
    println!("生成されたベクタ: {:?}", vec);

    // キー・バリューパース
    println!("\n--- キー・バリューパース ---");
    let config = parse_kvs! {
        "host": "localhost",
        "port": 8080,
        "debug": true,
    };
    println!("設定: {:?}", config);

    // 条件付きコンパイル
    println!("\n--- 条件付きコンパイル ---");
    let platform = cfg_match! {
        #[cfg(target_os = "windows")] "Windows",
        #[cfg(target_os = "macos")] "macOS",
        #[cfg(target_os = "linux")] "Linux",
        _ => "その他のOS"
    };
    println!("プラットフォーム: {}", platform);

    // デリゲートパターンの使用例
    println!("\n--- デリゲートパターン ---");
    struct Wrapper {
        inner: Vec<i32>,
    }

    impl Wrapper {
        fn new() -> Self {
            Self { inner: vec![1, 2, 3, 4, 5] }
        }

        delegate! {
            self.inner {
                fn len(&self) -> usize;
                fn is_empty(&self) -> bool;
            }
        }
    }

    let wrapper = Wrapper::new();
    println!("ラッパーの長さ: {}", wrapper.len());
    println!("空かどうか: {}", wrapper.is_empty());

    // 型レベル計算
    println!("\n--- 型レベル計算 ---");
    let max = type_level_max!(3, 7, 2, 9, 5);
    println!("最大値: {}", max);

    // ジェネリクスの実装
    println!("\n--- ジェネリクス実装 ---");
    trait MyTrait {
        fn describe(&self) -> String;
    }

    generic_impl! {
        impl<T> MyTrait for Vec<T> {
            fn describe(&self) -> String {
                format!("ベクタ（長さ: {}）", self.len())
            }
        }
    }

    let vec: Vec<i32> = vec![1, 2, 3];
    println!("説明: {}", vec.describe());

    println!("\n高度なマクロパターンをマスターしました！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_produces_array_in_reverse_order() {
        assert_eq!(reverse!([1 2 3 4 5]), [5, 4, 3, 2, 1]);
        assert_eq!(reverse!([7]), [7]);
    }

    #[test]
    fn parse_kvs_stores_mixed_values_as_strings() {
        let map = parse_kvs! {
            "host": "localhost",
            "port": 8080,
            "debug": true,
        };

        assert_eq!(map.len(), 3);
        assert_eq!(map["host"], "localhost");
        assert_eq!(map["port"], "8080");
        assert_eq!(map["debug"], "true");
    }

    #[test]
    fn cfg_match_selects_arm_for_current_target_or_default() {
        let os = cfg_match! {
            #[cfg(target_os = "macos")] "macOS",
            #[cfg(target_os = "linux")] "Linux",
            _ => "other"
        };
        let expected = if cfg!(target_os = "macos") {
            "macOS"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else {
            "other"
        };
        assert_eq!(os, expected);

        let fallback = cfg_match! {
            #[cfg(target_os = "no-such-os")] 1,
            _ => 2
        };
        assert_eq!(fallback, 2);
    }
}
