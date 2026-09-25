// src/bin/problem3_hrtb.rs
// 復習問題3: HRTB（高階トレイト境界）の解答

fn main() {
    println!("=== 復習問題3: HRTB（高階トレイト境界）===\n");

    // 基本的なHRTBの例
    basic_hrtb_example();

    // 実践的なHRTBの使用例
    practical_hrtb_examples();

    // HRTBが必要な理由
    why_hrtb_needed();

    // 複雑なHRTBパターン
    advanced_hrtb_patterns();
}

// 基本的なHRTBの例
fn basic_hrtb_example() {
    println!("【基本的なHRTB】");

    // for<'a>が必要な関数
    fn apply_to_string<F>(f: F) -> String
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s = String::from("hello world");
        f(&s).to_string()
    }

    // 関数ポインタを使用（HRTBの問題を回避）
    fn extract_first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }

    let result = apply_to_string(extract_first_word);
    println!("結果: {}", result);

    // より複雑な例
    fn get_first_char(s: &str) -> &str {
        if s.is_empty() {
            s
        } else {
            &s[0..1]
        }
    }

    let result2 = apply_to_string(get_first_char);
    println!("最初の文字: {}", result2);

    println!("\n解説:");
    println!("・for<'a>は「任意のライフタイム'aに対して」という意味");
    println!("・クロージャが呼び出されるたびに異なるライフタイムで動作可能");
    println!("・通常のライフタイム境界では'aが固定されてしまう");
}

// 実践的なHRTBの使用例
fn practical_hrtb_examples() {
    println!("\n【実践的なHRTB使用例】");

    // 文字列処理関数のコンテナ
    struct StringProcessor<F>
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        processor: F,
        name: String,
    }

    impl<F> StringProcessor<F>
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        fn new(name: String, processor: F) -> Self {
            StringProcessor { processor, name }
        }

        fn process(&self, input: &str) -> String {
            let result = (self.processor)(input);
            format!("{}: {}", self.name, result)
        }

        fn process_multiple(&self, inputs: &[&str]) -> Vec<String> {
            inputs.iter().map(|&input| self.process(input)).collect()
        }
    }

    // 様々な処理関数
    let trim_processor = StringProcessor::new("Trimmer".to_string(), |s: &str| s.trim());

    let first_word_processor = StringProcessor::new("FirstWord".to_string(), |s: &str| {
        s.split_whitespace().next().unwrap_or("")
    });

    let inputs = vec![
        "  hello world  ",
        "rust programming",
        "   functional   programming   ",
    ];

    println!("Trim処理:");
    for result in trim_processor.process_multiple(&inputs) {
        println!("  {}", result);
    }

    println!("\nFirstWord処理:");
    for result in first_word_processor.process_multiple(&inputs) {
        println!("  {}", result);
    }

    // 関数ポインタとの組み合わせ
    fn extract_extension(filename: &str) -> &str {
        filename.split('.').last().unwrap_or("")
    }

    let extension_processor = StringProcessor::new("Extension".to_string(), extract_extension);

    let filenames = vec!["document.pdf", "image.png", "script.rs"];
    println!("\nExtension処理:");
    for result in extension_processor.process_multiple(&filenames) {
        println!("  {}", result);
    }
}

// HRTBが必要な理由
fn why_hrtb_needed() {
    println!("\n【HRTBが必要な理由】");

    // 通常のライフタイム境界の問題
    println!("通常のライフタイム境界の問題:");

    // これはコンパイルエラーになる例（概念的説明）
    /*
    fn broken_apply<'a, F>(f: F) -> String
    where
        F: Fn(&'a str) -> &'a str  // 'aが固定される
    {
        let s = String::from("test");  // 'aより短いライフタイム
        f(&s).to_string()  // エラー: ライフタイムが合わない
    }
    */

    println!("・通常の境界 F: Fn(&'a str) -> &'a str では'aが固定される");
    println!("・関数内で作成した値のライフタイムと合わない");
    println!("・HRTBにより任意のライフタイムで動作可能になる");

    // HRTBの解決策
    fn working_apply<F>(f: F) -> String
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s = String::from("test");
        f(&s).to_string()
    }

    fn identity(s: &str) -> &str {
        s
    }
    let result = working_apply(identity);
    println!("HRTBによる解決: {}", result);

    // より複雑な例：ネストした関数呼び出し
    fn nested_processing<F1, F2>(f1: F1, f2: F2) -> String
    where
        F1: for<'a> Fn(&'a str) -> &'a str,
        F2: for<'a> Fn(&'a str) -> &'a str,
    {
        let input = String::from("nested processing example");
        let intermediate = f1(&input);
        let final_result = f2(intermediate);
        final_result.to_string()
    }

    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    fn to_uppercase_first(s: &str) -> &str {
        if s.is_empty() {
            s
        } else {
            &s[0..1]
        }
    }

    let nested_result = nested_processing(first_word, to_uppercase_first);
    println!("ネストした処理: {}", nested_result);
}

// 複雑なHRTBパターン
fn advanced_hrtb_patterns() {
    println!("\n【高度なHRTBパターン】");

    // 複数の引数を持つHRTB
    fn combine_strings<F>(f: F) -> String
    where
        F: for<'a, 'b> Fn(&'a str, &'b str) -> String,
    {
        let s1 = String::from("Hello");
        let s2 = String::from("World");
        f(&s1, &s2)
    }

    let combiner = |a: &str, b: &str| format!("{} {}", a, b);
    let combined = combine_strings(combiner);
    println!("文字列結合: {}", combined);

    // HRTBとトレイトオブジェクト
    trait StringTransformer {
        fn transform<'a>(&self, input: &'a str) -> &'a str;
    }

    struct TrimTransformer;
    impl StringTransformer for TrimTransformer {
        fn transform<'a>(&self, input: &'a str) -> &'a str {
            input.trim()
        }
    }

    struct FirstWordTransformer;
    impl StringTransformer for FirstWordTransformer {
        fn transform<'a>(&self, input: &'a str) -> &'a str {
            input.split_whitespace().next().unwrap_or("")
        }
    }

    fn apply_transformer(transformer: &dyn StringTransformer, input: &str) -> String {
        transformer.transform(input).to_string()
    }

    let trim_transformer = TrimTransformer;
    let first_word_transformer = FirstWordTransformer;

    let test_input = "  hello world  ";

    println!(
        "Trim変換: {}",
        apply_transformer(&trim_transformer, test_input)
    );
    println!(
        "FirstWord変換: {}",
        apply_transformer(&first_word_transformer, test_input)
    );

    // HRTBとイテレータ
    fn process_iterator<I, F>(iter: I, processor: F) -> Vec<String>
    where
        I: Iterator,
        I::Item: AsRef<str>,
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        iter.map(|item| processor(item.as_ref()).to_string())
            .collect()
    }

    let words = vec!["  hello  ", "  world  ", "  rust  "];
    let trimmed = process_iterator(words.iter(), |s| s.trim());
    println!("イテレータ処理: {:?}", trimmed);

    // 実際のユースケース：設定バリデーター
    struct ConfigValidator<F>
    where
        F: for<'a> Fn(&'a str) -> Result<&'a str, String>,
    {
        validator: F,
        name: String,
    }

    impl<F> ConfigValidator<F>
    where
        F: for<'a> Fn(&'a str) -> Result<&'a str, String>,
    {
        fn new(name: String, validator: F) -> Self {
            ConfigValidator { validator, name }
        }

        fn validate(&self, input: &str) -> Result<String, String> {
            match (self.validator)(input) {
                Ok(valid) => Ok(format!("{}: {}", self.name, valid)),
                Err(e) => Err(format!("{}: {}", self.name, e)),
            }
        }
    }

    // URL バリデーター
    let url_validator = ConfigValidator::new("URL".to_string(), |s: &str| {
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(s)
        } else {
            Err("無効なURL形式".to_string())
        }
    });

    // ポート番号バリデーター
    let port_validator = ConfigValidator::new("Port".to_string(), |s: &str| {
        if s.parse::<u16>().is_ok() {
            Ok(s)
        } else {
            Err("無効なポート番号".to_string())
        }
    });

    println!("\nバリデーション例:");

    let test_url = "https://example.com";
    match url_validator.validate(test_url) {
        Ok(result) => println!("✓ {}", result),
        Err(error) => println!("✗ {}", error),
    }

    let test_port = "8080";
    match port_validator.validate(test_port) {
        Ok(result) => println!("✓ {}", result),
        Err(error) => println!("✗ {}", error),
    }

    let invalid_url = "not-a-url";
    match url_validator.validate(invalid_url) {
        Ok(result) => println!("✓ {}", result),
        Err(error) => println!("✗ {}", error),
    }

    println!("\n解説:");
    println!("・HRTBは関数型プログラミングパターンで重要");
    println!("・任意のライフタイムで動作する汎用的な関数を作成可能");
    println!("・トレイトオブジェクトとの組み合わせで柔軟性が向上");
    println!("・イテレータパターンでも頻繁に使用される");
    println!("・設定バリデーション等の実用的な場面でも活用できる");
}
