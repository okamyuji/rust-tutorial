// src/bin/problem2_lifetime_elision.rs
// 復習問題2: ライフタイム省略の解答

fn main() {
    println!("=== 復習問題2: ライフタイム省略 ===\n");
    
    // ライフタイム省略の基本例
    basic_elision_example();
    
    // 3つの省略規則の詳細
    elision_rules_explanation();
    
    // 省略が適用されない場合
    no_elision_cases();
    
    // メソッドでの省略
    method_elision_example();
}

// 基本的なライフタイム省略の例
fn basic_elision_example() {
    // 元の関数（ライフタイム省略が適用される）
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    println!("【基本的なライフタイム省略】");
    
    let sentence = String::from("Hello world from Rust programming");
    let word = first_word(&sentence);
    println!("最初の単語: '{}'", word);
    
    // コンパイラによる自動変換（概念的な表現）
    fn first_word_explicit<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let word_explicit = first_word_explicit(&sentence);
    println!("明示的ライフタイム版: '{}'", word_explicit);
    
    println!("\n解説:");
    println!("・入力パラメータが1つの参照のみ");
    println!("・省略規則により、出力のライフタイムは入力と同じになる");
    println!("・fn first_word(s: &str) -> &str は fn first_word<'a>(s: &'a str) -> &'a str と等価");
}

// 3つの省略規則の詳細説明
fn elision_rules_explanation() {
    println!("\n【ライフタイム省略の3つの規則】");
    
    // 規則1: 各参照パラメータは独自のライフタイムを取得
    fn rule1_example(x: &str, y: &str) -> String {
        // コンパイラによる変換: fn rule1_example<'a, 'b>(x: &'a str, y: &'b str) -> String
        format!("{} {}", x, y)
    }
    
    println!("規則1の例:");
    let result1 = rule1_example("Hello", "World");
    println!("  結果: {}", result1);
    println!("  変換: fn(x: &str, y: &str) -> fn<'a, 'b>(x: &'a str, y: &'b str)");
    
    // 規則2: 入力ライフタイムが1つだけの場合、それが出力に適用される
    fn rule2_example(s: &str) -> &str {
        // コンパイラによる変換: fn rule2_example<'a>(s: &'a str) -> &'a str
        &s[0..1]
    }
    
    println!("\n規則2の例:");
    let input = String::from("Rust");
    let result2 = rule2_example(&input);
    println!("  入力: '{}', 結果: '{}'", input, result2);
    println!("  変換: fn(s: &str) -> &str => fn<'a>(s: &'a str) -> &'a str");
    
    // 規則3: メソッドで&selfがある場合、selfのライフタイムが出力に適用される
    struct TextProcessor {
        prefix: String,
    }
    
    impl TextProcessor {
        fn process(&self, input: &str) -> String {
            // コンパイラによる変換: fn process<'a, 'b>(&'a self, input: &'b str) -> String
            format!("{}: {}", self.prefix, input)
        }
        
        fn get_prefix(&self) -> &str {
            // コンパイラによる変換: fn get_prefix<'a>(&'a self) -> &'a str
            &self.prefix
        }
    }
    
    println!("\n規則3の例:");
    let processor = TextProcessor {
        prefix: "処理".to_string(),
    };
    
    let processed = processor.process("データ");
    println!("  処理結果: {}", processed);
    
    let prefix = processor.get_prefix();
    println!("  プレフィックス: '{}'", prefix);
    println!("  変換: fn get_prefix(&self) -> &str => fn<'a>(&'a self) -> &'a str");
}

// 省略が適用されない場合
fn no_elision_cases() {
    println!("\n【省略が適用されない場合】");
    
    // ケース1: 複数の入力参照があり、戻り値も参照
    // この関数はライフタイム注釈が必要
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    println!("ケース1: 複数の入力参照");
    let str1 = "Hello";
    let str2 = "World!";
    let result = longest(str1, str2);
    println!("  最長: '{}'", result);
    println!("  理由: どの入力のライフタイムを使うべきかコンパイラが判断できない");
    
    // ケース2: 入力と無関係な参照を返す
    fn get_static() -> &'static str {
        "これは静的文字列です"
    }
    
    println!("\nケース2: 静的参照を返す");
    let static_str = get_static();
    println!("  静的文字列: '{}'", static_str);
    println!("  理由: 入力に依存しない'staticライフタイムが必要");
    
    // ケース3: 構造体のフィールドから異なるライフタイムの参照を返す
    struct Container<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    impl<'a, 'b> Container<'a, 'b> {
        // どちらのフィールドを返すかによってライフタイムが異なる
        fn get_first(&self) -> &'a str {
            self.first
        }
        
        fn get_second(&self) -> &'b str {
            self.second
        }
        
        // 条件によって異なるフィールドを返す場合、共通のライフタイムが必要
        fn get_conditional(&self, use_first: bool) -> &str 
        where 
            'a: 'b, // 'aは'bより長いかまたは等しい
        {
            if use_first {
                self.first
            } else {
                self.second
            }
        }
    }
    
    println!("\nケース3: 複数のライフタイムを持つ構造体");
    let first_str = String::from("First string");
    let second_str = String::from("Second");
    
    let container = Container {
        first: &first_str,
        second: &second_str,
    };
    
    println!("  最初のフィールド: '{}'", container.get_first());
    println!("  2番目のフィールド: '{}'", container.get_second());
    println!("  条件付き取得: '{}'", container.get_conditional(true));
}

// メソッドでのライフタイム省略
fn method_elision_example() {
    println!("\n【メソッドでのライフタイム省略】");
    
    struct StringAnalyzer {
        text: String,
    }
    
    impl StringAnalyzer {
        fn new(text: String) -> Self {
            StringAnalyzer { text }
        }
        
        // 省略適用: &self -> &str
        fn get_text(&self) -> &str {
            &self.text
        }
        
        // 省略適用: &self, &str -> &str (selfのライフタイムが使用される)
        fn find_substring(&self, pattern: &str) -> Option<&str> {
            if self.text.contains(pattern) {
                Some(&self.text)
            } else {
                None
            }
        }
        
        // 省略適用: &self -> Vec<&str>
        fn get_words(&self) -> Vec<&str> {
            self.text.split_whitespace().collect()
        }
        
        // 明示的なライフタイム注釈が必要な場合
        fn compare_with<'a>(&'a self, other: &'a str) -> &'a str {
            if self.text.len() > other.len() {
                &self.text
            } else {
                other
            }
        }
    }
    
    let analyzer = StringAnalyzer::new("Hello Rust programming world".to_string());
    
    println!("元のテキスト: '{}'", analyzer.get_text());
    
    if let Some(found) = analyzer.find_substring("Rust") {
        println!("パターン発見: '{}'", found);
    }
    
    let words = analyzer.get_words();
    println!("単語リスト: {:?}", words);
    
    let other_text = "Short";
    let longer = analyzer.compare_with(other_text);
    println!("より長いテキスト: '{}'", longer);
    
    println!("\n省略規則の適用:");
    println!("・get_text: &self -> &str (規則3適用)");
    println!("・find_substring: &self, &str -> Option<&str> (規則3適用、selfのライフタイム)");
    println!("・get_words: &self -> Vec<&str> (規則3適用)");
    println!("・compare_with: 明示的注釈が必要（戻り値が入力のどちらかを参照）");
    
    // ライフタイム省略の限界を示す例
    println!("\n【省略の限界】");
    
    // この関数はコンパイルエラーになる（コメントアウト）
    /*
    fn problematic_function(x: &str, y: &str) -> &str {
        // どちらの入力のライフタイムを使うべきかわからない
        if x.len() > y.len() { x } else { y }
    }
    */
    
    println!("省略が適用できない場合:");
    println!("・複数の入力参照があり、どれを返すか不明");
    println!("・入力と無関係な参照を返す");
    println!("・複雑なライフタイム関係がある");
    println!("→ このような場合は明示的なライフタイム注釈が必要");
}