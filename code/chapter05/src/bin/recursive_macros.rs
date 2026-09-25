//! 再帰的マクロの実装
//! 
//! マクロの再帰的な呼び出しを使った高度なテクニックを示します。

// リストの長さを計算する再帰マクロ
macro_rules! count_items {
    () => { 0 };
    ($head:expr) => { 1 };
    ($head:expr, $($tail:expr),*) => {
        1 + count_items!($($tail),*)
    };
}

// 最大値を見つける再帰マクロ
macro_rules! max {
    ($x:expr) => { $x };
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
    ($x:expr, $($rest:expr),+) => {
        max!($x, max!($($rest),+))
    };
}

// 木構造を作成する再帰マクロ
macro_rules! tree {
    (leaf: $value:expr) => {
        TreeNode::Leaf($value)
    };
    (node: $value:expr, [$($children:tt)*]) => {
        TreeNode::Node {
            value: $value,
            children: tree!(@children [] $($children)*),
        }
    };
    // 子要素は `leaf: 2` のように複数トークンから成るため、tt 単位ではなく先頭から1要素ずつ取り出す
    (@children [$($done:expr),*]) => {
        vec![$($done),*]
    };
    (@children [$($done:expr),*] leaf: $value:expr $(, $($rest:tt)*)?) => {
        tree!(@children [$($done,)* tree!(leaf: $value)] $($($rest)*)?)
    };
    (@children [$($done:expr),*] node: $value:expr, [$($sub:tt)*] $(, $($rest:tt)*)?) => {
        tree!(@children [$($done,)* tree!(node: $value, [$($sub)*])] $($($rest)*)?)
    };
}

#[derive(Debug)]
enum TreeNode<T> {
    Leaf(T),
    Node {
        value: T,
        children: Vec<TreeNode<T>>,
    },
}

// パイプライン処理の再帰マクロ
macro_rules! pipeline {
    ($val:expr, $fn:expr) => {
        $fn($val)
    };
    ($val:expr, $fn:expr, $($rest:expr),+) => {
        pipeline!($fn($val), $($rest),+)
    };
}

// 条件付き繰り返しマクロ
macro_rules! repeat_n {
    ($n:expr, $body:expr) => {
        for _ in 0..$n {
            $body();
        }
    };
}

// タプルを展開する再帰マクロ
macro_rules! tuple_len {
    () => { 0 };
    ($($t:ty),+) => {
        <[()]>::len(&[$(tuple_len!(@single $t)),*])
    };
    (@single $t:ty) => { () };
}

// ビットフラグを生成する再帰マクロ
macro_rules! bitflags {
    (
        pub struct $name:ident: $type:ty {
            $(const $flag:ident = $value:expr;)*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name($type);

        impl $name {
            $(pub const $flag: $name = $name($value);)*

            pub fn contains(&self, other: $name) -> bool {
                self.0 & other.0 == other.0
            }

            pub fn insert(&mut self, other: $name) {
                self.0 |= other.0;
            }

            pub fn remove(&mut self, other: $name) {
                self.0 &= !other.0;
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                $name(self.0 | other.0)
            }
        }
    };
}

// パターンマッチの再帰展開
macro_rules! match_all {
    ($value:expr, { $($pattern:pat => $result:expr),* }) => {
        match $value {
            $($pattern => Some($result),)*
            _ => None,
        }
    };
}

// リスト処理の再帰マクロ
macro_rules! map_list {
    // 変換済みの要素を蓄積しながら先頭から1つずつ処理する
    (@acc $f:expr, [], [$($done:expr),*]) => {
        [$($done),*]
    };
    (@acc $f:expr, [$head:expr $(, $tail:expr)*], [$($done:expr),*]) => {
        map_list!(@acc $f, [$($tail),*], [$($done,)* $f($head)])
    };
    ($f:expr, [$($items:expr),* $(,)?]) => {
        map_list!(@acc $f, [$($items),*], [])
    };
}

// 実行時に計算する階乗関数
fn runtime_factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * runtime_factorial(n - 1),
    }
}

// 実行時に計算するフィボナッチ関数
fn runtime_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => runtime_fibonacci(n - 1) + runtime_fibonacci(n - 2),
    }
}

fn main() {
    println!("=== 再帰的マクロの実装 ===\n");

    // 再帰的な計算（実行時）
    println!("--- 再帰的計算（実行時） ---");
    println!("5! = {}", runtime_factorial(5));
    println!("フィボナッチ(10) = {}", runtime_fibonacci(10));

    // アイテム数のカウント
    println!("\n--- アイテム数カウント ---");
    let count = count_items!(1, 2, 3, 4, 5, 6, 7);
    println!("アイテム数: {}", count);

    // 最大値の検索
    println!("\n--- 最大値検索 ---");
    let max_val = max!(5, 2, 8, 1, 9, 3, 7);
    println!("最大値: {}", max_val);

    // 木構造の作成
    println!("\n--- 木構造 ---");
    let tree = tree!(
        node: 1, [
            leaf: 2,
            node: 3, [
                leaf: 4,
                leaf: 5
            ],
            leaf: 6
        ]
    );
    println!("木構造: {:#?}", tree);

    // パイプライン処理
    println!("\n--- パイプライン処理 ---");
    let result = pipeline!(
        5,
        |x| x * 2,
        |x| x + 3,
        |x: i32| x.to_string()
    );
    println!("パイプライン結果: {}", result);

    // 条件付き繰り返し
    println!("\n--- 条件付き繰り返し ---");
    let mut counter = 0;
    repeat_n!(5, || {
        counter += 1;
        println!("繰り返し {}", counter);
    });

    // タプルの長さ
    println!("\n--- タプルの長さ ---");
    type MyTuple = (i32, String, bool, f64);
    let tuple_length = tuple_len!(i32, String, bool, f64);
    println!("タプルの長さ: {}", tuple_length);

    // ビットフラグ
    println!("\n--- ビットフラグ ---");
    bitflags! {
        pub struct Permissions: u32 {
            const READ = 0b001;
            const WRITE = 0b010;
            const EXECUTE = 0b100;
        }
    }

    let mut perms = Permissions::READ;
    perms.insert(Permissions::WRITE);
    println!("権限: {:?}", perms);
    println!("書き込み権限あり: {}", perms.contains(Permissions::WRITE));
    println!("実行権限あり: {}", perms.contains(Permissions::EXECUTE));

    // パターンマッチの再帰展開
    println!("\n--- パターンマッチ展開 ---");
    let value = 3;
    let result = match_all!(value, {
        1 => "one",
        2 => "two", 
        3 => "three",
        4 => "four"
    });
    println!("マッチ結果: {:?}", result);

    // リスト処理
    println!("\n--- リスト処理 ---");
    let doubled = map_list!(|x: i32| x * 2, [1, 2, 3, 4, 5]);
    println!("2倍にしたリスト: {:?}", doubled);

    // コンパイル時定数（const関数を使用）
    println!("\n--- コンパイル時定数 ---");
    const fn const_factorial(n: u32) -> u32 {
        match n {
            0 => 1,
            _ => n * const_factorial(n - 1),
        }
    }
    const FACT_5: u32 = const_factorial(5);
    println!("5! (const) = {}", FACT_5);

    println!("\n再帰的マクロの力を理解しました！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_builds_nested_children_in_order() {
        let tree = tree!(node: 1, [leaf: 2, node: 3, [leaf: 4], leaf: 5]);

        let TreeNode::Node { value, children } = tree else { panic!("node expected") };
        assert_eq!(value, 1);
        assert_eq!(children.len(), 3);
        assert!(matches!(children[0], TreeNode::Leaf(2)));
        assert!(matches!(&children[1], TreeNode::Node { value: 3, children } if matches!(children.as_slice(), [TreeNode::Leaf(4)])));
        assert!(matches!(children[2], TreeNode::Leaf(5)));
    }

    #[test]
    fn tree_supports_leaf_only_and_empty_children() {
        assert!(matches!(tree!(leaf: 9), TreeNode::Leaf(9)));
        assert!(matches!(tree!(node: 1, []), TreeNode::Node { value: 1, children } if children.is_empty()));
    }

    #[test]
    fn map_list_applies_function_in_order() {
        assert_eq!(map_list!(|x: i32| x * 2, [1, 2, 3]), [2, 4, 6]);
        assert_eq!(map_list!(|x: i32| x + 1, [7]), [8]);
    }
}
