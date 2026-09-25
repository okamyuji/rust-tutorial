// src/bin/coherence_rules.rs

use std::fmt::{self, Display};
use std::ops::Add;

fn main() {
    println!("=== コヒーレンスルールと孤児ルール ===\n");

    orphan_rule_demo();
    newtype_pattern();
    trait_coherence();
    blanket_implementations();
}

// 孤児ルール（Orphan Rule）のデモ
fn orphan_rule_demo() {
    println!("--- 孤児ルール ---");

    // 自分で定義した型
    struct MyType {
        value: String,
    }

    // ✅ OK: 自分の型に標準トレイトを実装
    impl Display for MyType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyType: {}", self.value)
        }
    }

    let my_value = MyType {
        value: "Hello from MyType".to_string(),
    };
    println!("{}", my_value);

    // 自分で定義したトレイト
    trait MyTrait {
        fn my_method(&self) -> String;
    }

    // ✅ OK: 自分のトレイトを標準型に実装
    impl MyTrait for String {
        fn my_method(&self) -> String {
            format!("MyTrait for String: {}", self)
        }
    }

    let s = String::from("test");
    println!("{}", s.my_method());

    // ✅ OK: 自分の型に自分のトレイトを実装
    impl MyTrait for MyType {
        fn my_method(&self) -> String {
            format!("MyTrait for MyType: {}", self.value)
        }
    }

    println!("{}", my_value.my_method());

    // ❌ できないこと：外部型に外部トレイトを実装
    // impl Display for Vec<String> { ... } // エラー！

    println!("\n孤児ルールの理由:");
    println!("- 実装の衝突を防ぐ");
    println!("- クレート間の独立性を保つ");
    println!("- 後方互換性を維持する");
}

// ニュータイプパターン（孤児ルールの回避策）
fn newtype_pattern() {
    println!("\n--- ニュータイプパターン ---");

    // Vec<String>にDisplayを実装したい場合
    struct DisplayableVec(Vec<String>);

    impl Display for DisplayableVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    let vec = DisplayableVec(vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ]);
    println!("DisplayableVec: {}", vec);

    // タプル構造体でのラップ
    #[derive(Debug, Clone)]
    struct Meters(f64);

    #[derive(Debug, Clone)]
    struct Feet(f64);

    // 独自の演算を定義
    impl Add for Meters {
        type Output = Meters;

        fn add(self, other: Meters) -> Meters {
            Meters(self.0 + other.0)
        }
    }

    impl From<Feet> for Meters {
        fn from(feet: Feet) -> Self {
            Meters(feet.0 * 0.3048)
        }
    }

    let m1 = Meters(5.0);
    let m2 = Meters(3.0);
    let m3 = m1.clone() + m2.clone();
    println!("\n単位付き計算: {:?} + {:?} = {:?}", m1, m2, m3);

    let feet = Feet(10.0);
    let meters: Meters = feet.into();
    println!("変換: 10 feet = {:?}", meters);

    // Derefを使った透過的なラップ
    use std::ops::Deref;

    struct MyString(String);

    impl Deref for MyString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for MyString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyString: {}", self.0)
        }
    }

    let my_string = MyString("Deref example".to_string());
    println!("\n{}", my_string);
    println!("長さ: {}", my_string.len()); // Derefにより自動的にStringのメソッドが使える
}

// トレイトのコヒーレンス
fn trait_coherence() {
    println!("\n--- トレイトのコヒーレンス ---");

    // トレイト定義
    trait Processor {
        fn process(&self, input: &str) -> String;
    }

    // 具体的な型への実装
    struct UpperCaseProcessor;

    impl Processor for UpperCaseProcessor {
        fn process(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    // ジェネリック実装（より一般的）
    struct IdentityProcessor<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Processor for IdentityProcessor<T> {
        fn process(&self, input: &str) -> String {
            input.to_string()
        }
    }

    let upper = UpperCaseProcessor;
    let identity = IdentityProcessor::<()> {
        _phantom: std::marker::PhantomData,
    };

    println!("UpperCase: {}", upper.process("hello"));
    println!("Identity: {}", identity.process("hello"));

    // トレイトの拡張
    trait AdvancedProcessor: Processor {
        fn process_with_metadata(&self, input: &str) -> (String, usize) {
            let result = self.process(input);
            let len = result.len();
            (result, len)
        }
    }

    impl AdvancedProcessor for UpperCaseProcessor {}

    let (result, len) = upper.process_with_metadata("test");
    println!("\nAdvanced処理: '{}' (長さ: {})", result, len);

    // 衝突の回避
    mod module_a {
        pub trait Converter {
            fn convert(&self) -> String;
        }

        impl Converter for i32 {
            fn convert(&self) -> String {
                format!("A: {}", self)
            }
        }
    }

    mod module_b {
        pub trait Converter {
            fn convert(&self) -> String;
        }

        impl Converter for i32 {
            fn convert(&self) -> String {
                format!("B: {}", self)
            }
        }
    }

    // 明示的な指定で衝突を解決
    use module_a::Converter as ConverterA;
    use module_b::Converter as ConverterB;

    let num = 42;
    println!("\n名前空間による解決:");
    println!("  {}", ConverterA::convert(&num));
    println!("  {}", ConverterB::convert(&num));
}

// ブランケット実装
fn blanket_implementations() {
    println!("\n--- ブランケット実装 ---");

    // すべてのDisplay型に対する実装
    trait Quotable {
        fn quote(&self) -> String;
    }

    impl<T: Display> Quotable for T {
        fn quote(&self) -> String {
            format!("「{}」", self)
        }
    }

    println!("整数の引用: {}", 42.quote());
    println!("文字列の引用: {}", "Hello".quote());

    // 条件付きブランケット実装
    trait Describable {
        fn describe(&self) -> String;
    }

    // Debugトレイトを実装する型すべてに自動実装
    impl<T: std::fmt::Debug> Describable for T {
        fn describe(&self) -> String {
            format!("Debug表現: {:?}", self)
        }
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    println!("\n{}", point.describe());

    // より複雑なブランケット実装
    trait Container {
        type Item;
        fn items(&self) -> Vec<&Self::Item>;
    }

    trait Summarizable {
        fn summary(&self) -> String;
    }

    // Containerを実装し、その要素がDisplayの場合に自動実装
    impl<T> Summarizable for T
    where
        T: Container,
        T::Item: Display,
    {
        fn summary(&self) -> String {
            let items: Vec<String> = self
                .items()
                .iter()
                .map(|item| format!("{}", item))
                .collect();
            format!(
                "Container with {} items: [{}]",
                items.len(),
                items.join(", ")
            )
        }
    }

    struct MyContainer {
        data: Vec<String>,
    }

    impl Container for MyContainer {
        type Item = String;

        fn items(&self) -> Vec<&Self::Item> {
            self.data.iter().collect()
        }
    }

    let container = MyContainer {
        data: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };

    println!("\n{}", container.summary());

    // 再帰的な制限
    println!("\n--- ブランケット実装の制限 ---");

    trait MyFrom<T> {
        fn my_from(value: T) -> Self;
    }

    // 基本実装
    impl MyFrom<i32> for String {
        fn my_from(value: i32) -> Self {
            value.to_string()
        }
    }

    impl MyFrom<bool> for String {
        fn my_from(value: bool) -> Self {
            if value { "true" } else { "false" }.to_string()
        }
    }

    // 以下は無限再帰になるため実装できない
    // impl<T, U> MyFrom<T> for U
    // where
    //     U: MyFrom<String>,
    //     T: ToString,
    // {
    //     fn my_from(value: T) -> Self {
    //         U::my_from(value.to_string())
    //     }
    // }

    println!("i32 -> String: {}", String::my_from(42));
    println!("bool -> String: {}", String::my_from(true));

    // 実装の優先順位
    trait Priority {
        fn priority(&self) -> &str;
    }

    // 一般的な実装
    impl<T> Priority for T {
        fn priority(&self) -> &str {
            "低優先度（デフォルト）"
        }
    }

    // 特定の実装はコヒーレンス規則で競合するため削除
    // より汎用的な実装のみを使用

    println!("\n優先順位のデモ:");
    println!("  i32: {}", 42.priority());
    println!("  String: {}", "test".to_string().priority());
}
