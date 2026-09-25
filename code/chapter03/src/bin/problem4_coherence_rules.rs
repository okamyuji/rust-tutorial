// src/bin/problem4_coherence_rules.rs
// 復習問題4: コヒーレンスルールとコンパイルエラー

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

fn main() {
    println!("=== 復習問題4: コヒーレンスルールとコンパイルエラー ===\n");

    // 問題の分析
    analyze_coherence_problem();

    // 解決策1: New Type パターン
    newtype_pattern_solution();

    // 解決策2: 拡張トレイト
    extension_trait_solution();

    // 解決策3: ジェネリック関数アプローチ
    generic_function_approach();

    // 解決策4: ラッパー構造体
    wrapper_struct_approach();

    // 高度なコヒーレンスの例
    advanced_coherence_examples();
}

// 問題の分析
fn analyze_coherence_problem() {
    println!("【問題の分析】");

    /*
    // このコードはコンパイルエラーになる
    impl Display for Option<String> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Some(s) => write!(f, "Some({})", s),
                None => write!(f, "None"),
            }
        }
    }
    */

    println!("エラーの理由:");
    println!("1. 孤児ルール（Orphan Rule）の違反");
    println!("   - Option<T> は標準ライブラリで定義された外部型");
    println!("   - Display も標準ライブラリで定義された外部トレイト");
    println!("   - 両方とも外部の型とトレイトには実装できない");

    println!("\n2. コヒーレンスの保証");
    println!("   - トレイト実装の一意性を保証");
    println!("   - 複数のクレートが同じ実装を提供することを防ぐ");
    println!("   - 実行時の曖昧性を回避");

    println!("\n3. 孤児ルールの条件");
    println!("   - トレイトまたは型の少なくとも一方が現在のクレートで定義されている必要がある");
    println!("   - これにより実装の衝突を防ぐ");
}

// 解決策1: New Type パターン
fn newtype_pattern_solution() {
    println!("\n【解決策1: New Type パターン】");

    // ラッパー型を作成
    #[derive(Debug, Clone, PartialEq)]
    struct DisplayOption(Option<String>);

    impl Display for DisplayOption {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match &self.0 {
                Some(s) => write!(f, "Some({})", s),
                None => write!(f, "None"),
            }
        }
    }

    // 便利なメソッドを追加
    impl DisplayOption {
        fn new(opt: Option<String>) -> Self {
            DisplayOption(opt)
        }

        fn some(value: String) -> Self {
            DisplayOption(Some(value))
        }

        fn none() -> Self {
            DisplayOption(None)
        }

        fn inner(&self) -> &Option<String> {
            &self.0
        }

        fn into_inner(self) -> Option<String> {
            self.0
        }

        fn is_some(&self) -> bool {
            self.0.is_some()
        }

        fn is_none(&self) -> bool {
            self.0.is_none()
        }

        fn as_ref(&self) -> Option<&String> {
            self.0.as_ref()
        }
    }

    // From/Into トレイトでの相互変換
    impl From<Option<String>> for DisplayOption {
        fn from(opt: Option<String>) -> Self {
            DisplayOption(opt)
        }
    }

    impl From<DisplayOption> for Option<String> {
        fn from(display_opt: DisplayOption) -> Self {
            display_opt.0
        }
    }

    impl From<String> for DisplayOption {
        fn from(s: String) -> Self {
            DisplayOption(Some(s))
        }
    }

    impl From<&str> for DisplayOption {
        fn from(s: &str) -> Self {
            DisplayOption(Some(s.to_string()))
        }
    }

    // 使用例
    println!("New Type パターンの使用例:");

    let some_option = DisplayOption::some("hello world".to_string());
    let none_option = DisplayOption::none();

    println!("  Some値: {}", some_option);
    println!("  None値: {}", none_option);

    // 変換例
    let regular_option: Option<String> = Some("test".to_string());
    let display_option = DisplayOption::from(regular_option);
    println!("  変換後: {}", display_option);

    // メソッドチェーン例
    let result = DisplayOption::from("initial")
        .inner()
        .as_ref()
        .map(|s| s.to_uppercase())
        .unwrap_or_default();
    println!("  メソッドチェーン結果: {}", result);

    // ジェネリック版
    #[derive(Debug, Clone, PartialEq)]
    struct DisplayOptionGeneric<T>(Option<T>);

    impl<T: Display> Display for DisplayOptionGeneric<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match &self.0 {
                Some(value) => write!(f, "Some({})", value),
                None => write!(f, "None"),
            }
        }
    }

    impl<T> DisplayOptionGeneric<T> {
        fn new(opt: Option<T>) -> Self {
            DisplayOptionGeneric(opt)
        }

        fn some(value: T) -> Self {
            DisplayOptionGeneric(Some(value))
        }

        fn none() -> Self {
            DisplayOptionGeneric(None)
        }
    }

    println!("\nジェネリック版の例:");
    let int_option = DisplayOptionGeneric::some(42);
    let float_option = DisplayOptionGeneric::some(std::f64::consts::PI);
    let none_int: DisplayOptionGeneric<i32> = DisplayOptionGeneric::none();

    println!("  整数Option: {}", int_option);
    println!("  浮動小数点Option: {}", float_option);
    println!("  None整数: {}", none_int);
}

// 解決策2: 拡張トレイト
fn extension_trait_solution() {
    println!("\n【解決策2: 拡張トレイト】");

    // 拡張トレイトを定義
    trait OptionDisplay {
        fn display_pretty(&self) -> String;
        fn display_with_prefix(&self, prefix: &str) -> String;
        fn display_or_default(&self, default: &str) -> String;
    }

    // 特定の実装はコヒーレンス規則で競合するため削除
    // より汎用的な実装のみを使用

    // より汎用的な実装
    impl<T: Display> OptionDisplay for Option<T> {
        fn display_pretty(&self) -> String {
            match self {
                Some(value) => format!("Some({})", value),
                None => "None".to_string(),
            }
        }

        fn display_with_prefix(&self, prefix: &str) -> String {
            match self {
                Some(value) => format!("{}: {}", prefix, value),
                None => format!("{}: None", prefix),
            }
        }

        fn display_or_default(&self, default: &str) -> String {
            match self {
                Some(value) => format!("{}", value),
                None => default.to_string(),
            }
        }
    }

    // 使用例
    println!("拡張トレイトの使用例:");

    let some_string = Some("hello".to_string());
    let none_string: Option<String> = None;
    let some_number = Some(42);
    let none_number: Option<i32> = None;

    println!("  文字列Some: {}", some_string.display_pretty());
    println!("  文字列None: {}", none_string.display_pretty());
    println!("  数値Some: {}", some_number.display_pretty());
    println!("  数値None: {}", none_number.display_pretty());

    println!("\nプレフィックス付き表示:");
    println!("  {}", some_string.display_with_prefix("Value"));
    println!("  {}", none_string.display_with_prefix("Value"));

    println!("\nデフォルト値付き表示:");
    println!("  {}", some_string.display_or_default("デフォルト"));
    println!("  {}", none_string.display_or_default("デフォルト"));

    // メソッドチェーンでの使用
    trait OptionChain {
        type Item;
        fn chain_display(self) -> ChainedDisplay<Self::Item>
        where
            Self: Sized;
    }

    impl<T> OptionChain for Option<T> {
        type Item = T;

        fn chain_display(self) -> ChainedDisplay<T> {
            ChainedDisplay { option: self }
        }
    }

    struct ChainedDisplay<T> {
        option: Option<T>,
    }

    impl<T: Display> ChainedDisplay<T> {
        fn with_brackets(self) -> String {
            match self.option {
                Some(value) => format!("[{}]", value),
                None => "[None]".to_string(),
            }
        }

        fn with_quotes(self) -> String {
            match self.option {
                Some(value) => format!("\"{}\"", value),
                None => "\"None\"".to_string(),
            }
        }
    }

    println!("\nメソッドチェーンの例:");
    let chained1 = some_number.chain_display().with_brackets();
    let chained2 = none_number.chain_display().with_quotes();
    println!("  括弧付き: {}", chained1);
    println!("  引用符付き: {}", chained2);
}

// 解決策3: ジェネリック関数アプローチ
fn generic_function_approach() {
    println!("\n【解決策3: ジェネリック関数アプローチ】");

    // 汎用的なOption表示関数
    fn display_option<T: Display>(opt: &Option<T>) -> String {
        match opt {
            Some(value) => format!("Some({})", value),
            None => "None".to_string(),
        }
    }

    // より高度な表示オプション付き
    fn display_option_formatted<T: Display>(
        opt: &Option<T>,
        some_format: &str,
        none_format: &str,
    ) -> String {
        match opt {
            Some(value) => some_format.replace("{}", &format!("{}", value)),
            None => none_format.to_string(),
        }
    }

    // カスタムフォーマッタ付き
    fn display_option_with_formatter<T, F>(opt: &Option<T>, formatter: F) -> String
    where
        F: Fn(&T) -> String,
    {
        match opt {
            Some(value) => format!("Some({})", formatter(value)),
            None => "None".to_string(),
        }
    }

    // 複数のOptionを比較表示
    fn compare_options<T: Display>(opts: &[Option<T>], separator: &str) -> String {
        opts.iter()
            .map(|opt| display_option(opt))
            .collect::<Vec<_>>()
            .join(separator)
    }

    // 使用例
    println!("ジェネリック関数の使用例:");

    let string_opt = Some("hello".to_string());
    let number_opt = Some(42);
    let none_opt: Option<String> = None;

    println!("  基本表示: {}", display_option(&string_opt));
    println!("  数値表示: {}", display_option(&number_opt));
    println!("  None表示: {}", display_option(&none_opt));

    println!("\nフォーマット付き表示:");
    println!(
        "  {}",
        display_option_formatted(&string_opt, "Value: {}", "No value")
    );
    println!(
        "  {}",
        display_option_formatted(&none_opt, "Value: {}", "No value")
    );

    println!("\nカスタムフォーマッタ:");
    let custom_formatter = |s: &String| s.to_uppercase();
    println!(
        "  {}",
        display_option_with_formatter(&string_opt, custom_formatter)
    );

    let number_formatter = |n: &i32| format!("#{:04}", n);
    println!(
        "  {}",
        display_option_with_formatter(&number_opt, number_formatter)
    );

    println!("\n複数Option比較:");
    let options = vec![Some("first".to_string()), None, Some("third".to_string())];
    println!("  {}", compare_options(&options, " | "));

    // より実用的な例：設定値の表示
    #[derive(Debug)]
    struct Config {
        database_url: Option<String>,
        api_key: Option<String>,
        timeout: Option<u32>,
        debug_mode: Option<bool>,
    }

    impl Config {
        fn display_summary(&self) -> String {
            let mut summary = vec![];

            summary.push(format!(
                "Database URL: {}",
                display_option_formatted(&self.database_url, "{}", "Not configured")
            ));
            summary.push(format!(
                "API Key: {}",
                display_option_formatted(&self.api_key, "{}...", "Not set")
            ));
            summary.push(format!(
                "Timeout: {}",
                display_option_formatted(&self.timeout, "{}ms", "Default")
            ));
            summary.push(format!(
                "Debug Mode: {}",
                display_option_formatted(&self.debug_mode, "{}", "Off")
            ));

            summary.join("\n")
        }
    }

    let config = Config {
        database_url: Some("postgresql://localhost/mydb".to_string()),
        api_key: None,
        timeout: Some(5000),
        debug_mode: Some(true),
    };

    println!("\n設定表示の例:");
    println!("{}", config.display_summary());
}

// 解決策4: ラッパー構造体アプローチ
fn wrapper_struct_approach() {
    println!("\n【解決策4: ラッパー構造体アプローチ】");

    // 透明なラッパー構造体
    #[derive(Debug, Clone, PartialEq)]
    #[repr(transparent)]
    struct OptionWrapper<T>(pub Option<T>);

    impl<T: Display> Display for OptionWrapper<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match &self.0 {
                Some(value) => write!(f, "Some({})", value),
                None => write!(f, "None"),
            }
        }
    }

    // Deref トレイトで透明性を提供
    impl<T> std::ops::Deref for OptionWrapper<T> {
        type Target = Option<T>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> std::ops::DerefMut for OptionWrapper<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // 便利なコンストラクタ
    impl<T> OptionWrapper<T> {
        fn some(value: T) -> Self {
            OptionWrapper(Some(value))
        }

        fn none() -> Self {
            OptionWrapper(None)
        }

        fn wrap(option: Option<T>) -> Self {
            OptionWrapper(option)
        }

        fn unwrap_inner(self) -> Option<T> {
            self.0
        }
    }

    // From/Into実装
    impl<T> From<Option<T>> for OptionWrapper<T> {
        fn from(option: Option<T>) -> Self {
            OptionWrapper(option)
        }
    }

    impl<T> From<OptionWrapper<T>> for Option<T> {
        fn from(wrapper: OptionWrapper<T>) -> Self {
            wrapper.0
        }
    }

    impl<T> From<T> for OptionWrapper<T> {
        fn from(value: T) -> Self {
            OptionWrapper(Some(value))
        }
    }

    // Iterator実装
    impl<T> IntoIterator for OptionWrapper<T> {
        type Item = T;
        type IntoIter = std::option::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    // 使用例
    println!("ラッパー構造体の使用例:");

    let wrapped_some = OptionWrapper::some("hello".to_string());
    let wrapped_none: OptionWrapper<String> = OptionWrapper::none();

    println!("  Some値: {}", wrapped_some);
    println!("  None値: {}", wrapped_none);

    // Derefによる透明なアクセス
    println!("\n透明なアクセス:");
    if wrapped_some.is_some() {
        println!("  値が存在します");
    }

    if let Some(value) = wrapped_some.as_ref() {
        println!("  値: {}", value);
    }

    // イテレータとしての使用
    println!("\nイテレータとしての使用:");
    for value in wrapped_some.clone() {
        println!("  イテレータ値: {}", value);
    }

    // マクロでの便利な生成
    macro_rules! opt_wrap {
        ($value:expr) => {
            OptionWrapper::some($value)
        };
        () => {
            OptionWrapper::none()
        };
    }

    let macro_some = opt_wrap!("macro value".to_string());
    let macro_none: OptionWrapper<String> = opt_wrap!();

    println!("\nマクロ生成:");
    println!("  マクロSome: {}", macro_some);
    println!("  マクロNone: {}", macro_none);
}

// 高度なコヒーレンスの例
fn advanced_coherence_examples() {
    println!("\n【高度なコヒーレンス例】");

    // ブランケット実装とコヒーレンス
    trait MyDisplay {
        fn my_display(&self) -> String;
    }

    // すべてのDisplayを実装する型に対してMyDisplayを実装
    impl<T: Display> MyDisplay for T {
        fn my_display(&self) -> String {
            format!("MyDisplay: {}", self)
        }
    }

    // カスタム型での特殊化的な実装
    #[derive(Debug)]
    struct SpecialType {
        value: String,
    }

    impl Display for SpecialType {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "Special: {}", self.value)
        }
    }

    // MyDisplayは自動的に実装される（ブランケット実装により）

    println!("ブランケット実装の例:");
    let special = SpecialType {
        value: "test".to_string(),
    };
    println!("  {}", special.my_display());

    // 条件付きブランケット実装
    trait ConditionalDisplay {
        fn conditional_display(&self) -> String;
    }

    impl<T> ConditionalDisplay for T
    where
        T: Display + Debug,
    {
        fn conditional_display(&self) -> String {
            format!("Display: {} | Debug: {:?}", self, self)
        }
    }

    println!("\n条件付きブランケット実装:");
    println!("  {}", special.conditional_display());

    // マーカートレイトとの組み合わせ
    trait Marker {}

    trait MarkerBasedDisplay {
        fn marker_display(&self) -> String;
    }

    // マーカーを持つ型のみに実装
    impl<T> MarkerBasedDisplay for T
    where
        T: Display + Marker,
    {
        fn marker_display(&self) -> String {
            format!("Marked: {}", self)
        }
    }

    #[derive(Debug)]
    struct MarkedType {
        data: i32,
    }

    impl Display for MarkedType {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "MarkedType({})", self.data)
        }
    }

    impl Marker for MarkedType {}

    let marked = MarkedType { data: 42 };
    println!("\nマーカーベース実装:");
    println!("  {}", marked.marker_display());

    // 孤児ルールを考慮した設計パターン
    println!("\n【設計パターンのベストプラクティス】");
    println!("✓ New Type パターン: 最も安全で型安全");
    println!("✓ 拡張トレイト: 既存型への機能追加に最適");
    println!("✓ ジェネリック関数: 一時的な解決に有効");
    println!("✓ ラッパー構造体: 透明性が必要な場合");
    println!("✓ ブランケット実装: 汎用的な機能提供");
    println!("✓ マーカートレイト: 型の分類と特殊化");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
