// src/bin/problem5_conditional_select.rs
// 復習問題5: 条件に基づく文字列選択の実装課題

fn main() {
    println!("=== 復習問題5: 条件に基づく文字列選択 ===\n");

    // 基本的な実装
    basic_string_selection();

    // より複雑な条件付き選択
    advanced_conditional_selection();

    // ライフタイムの交差の実演
    lifetime_intersection_demo();

    // 実用的な応用例
    practical_applications();
}

// 基本的な文字列選択の実装
fn basic_string_selection() {
    println!("【基本的な文字列選択】");

    // 条件に基づいて文字列を選択する関数
    fn select_string<'a>(first: &'a str, second: &'a str, prefer_longer: bool) -> &'a str {
        if prefer_longer {
            if first.len() >= second.len() {
                first
            } else {
                second
            }
        } else {
            if first.len() <= second.len() {
                first
            } else {
                second
            }
        }
    }

    let string1 = String::from("Hello, world!");
    let string2 = String::from("Rust");

    let longer = select_string(&string1, &string2, true);
    println!("より長い文字列: '{}'", longer);

    let shorter = select_string(&string1, &string2, false);
    println!("より短い文字列: '{}'", shorter);

    // 複数の比較条件
    fn select_by_criteria<'a>(
        first: &'a str,
        second: &'a str,
        criteria: fn(&str, &str) -> bool,
    ) -> &'a str {
        if criteria(first, second) {
            first
        } else {
            second
        }
    }

    // 様々な比較関数
    let alphabetically_first = |a: &str, b: &str| a < b;
    let contains_rust = |a: &str, b: &str| a.contains("Rust") && !b.contains("Rust");
    let more_vowels = |a: &str, b: &str| {
        let count_vowels = |s: &str| s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count();
        count_vowels(a) > count_vowels(b)
    };

    let rust_string = "Rust programming";
    let python_string = "Python programming";

    println!("\n様々な選択基準:");
    println!(
        "アルファベット順で最初: '{}'",
        select_by_criteria(&rust_string, &python_string, alphabetically_first)
    );
    println!(
        "Rustを含む: '{}'",
        select_by_criteria(&rust_string, &python_string, contains_rust)
    );
    println!(
        "母音が多い: '{}'",
        select_by_criteria(&rust_string, &python_string, more_vowels)
    );
}

// より複雑な条件付き選択
fn advanced_conditional_selection() {
    println!("\n【高度な条件付き選択】");

    // 複数の文字列から選択
    fn conditional_select<'a>(strings: &[&'a str], condition: fn(&str) -> bool) -> Option<&'a str> {
        strings.iter().find(|&&s| condition(s)).copied()
    }

    // 複数の条件を組み合わせた選択
    fn multi_criteria_select<'a>(
        strings: &[&'a str],
        criteria: &[fn(&str) -> bool],
    ) -> Vec<&'a str> {
        strings
            .iter()
            .filter(|&&s| criteria.iter().all(|criterion| criterion(s)))
            .copied()
            .collect()
    }

    let programming_languages = vec![
        "Rust",
        "Python",
        "JavaScript",
        "Go",
        "TypeScript",
        "C++",
        "Java",
    ];

    // 様々な条件
    let starts_with_vowel = |s: &str| "aeiouAEIOU".contains(s.chars().next().unwrap_or(' '));
    let has_uppercase = |s: &str| s.chars().any(|c| c.is_uppercase());
    let length_over_4 = |s: &str| s.len() > 4;
    let contains_script = |s: &str| s.to_lowercase().contains("script");

    println!("プログラミング言語リスト: {:?}", programming_languages);

    if let Some(found) = conditional_select(&programming_languages, starts_with_vowel) {
        println!("母音で始まる最初の言語: {}", found);
    }

    if let Some(found) = conditional_select(&programming_languages, contains_script) {
        println!("'script'を含む最初の言語: {}", found);
    }

    let complex_criteria = vec![has_uppercase, length_over_4];
    let matching_languages = multi_criteria_select(&programming_languages, &complex_criteria);
    println!("大文字を含み4文字超の言語: {:?}", matching_languages);

    // 優先度付き選択
    fn priority_select<'a>(strings: &[&'a str], priorities: &[fn(&str) -> i32]) -> Option<&'a str> {
        strings
            .iter()
            .max_by_key(|&&s| priorities.iter().map(|priority| priority(s)).sum::<i32>())
            .copied()
    }

    // 優先度関数
    let length_priority = |s: &str| s.len() as i32;
    let rust_bonus = |s: &str| if s.contains("Rust") { 10 } else { 0 };
    let vowel_penalty = |s: &str| -(s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() as i32);

    let priorities = vec![length_priority, rust_bonus, vowel_penalty];

    if let Some(winner) = priority_select(&programming_languages, &priorities) {
        println!("優先度最高の言語: {}", winner);
    }
}

// ライフタイムの交差の実演
fn lifetime_intersection_demo() {
    println!("\n【ライフタイムの交差】");

    fn select_string<'a>(first: &'a str, second: &'a str, prefer_longer: bool) -> &'a str {
        if prefer_longer {
            if first.len() >= second.len() {
                first
            } else {
                second
            }
        } else {
            if first.len() <= second.len() {
                first
            } else {
                second
            }
        }
    }

    // ライフタイムの交差を示す例
    let long_lived = String::from("これは長い期間生きる文字列です");

    println!("長期間生存する文字列: '{}'", long_lived);

    {
        let short_lived = String::from("短命");
        println!("短期間の文字列: '{}'", short_lived);

        // 結果のライフタイムは両方の入力の交差（より短い方）
        let result = select_string(&long_lived, &short_lived, true);
        println!("スコープ内での選択結果: '{}'", result);

        // この時点では両方の文字列が生きているのでOK
    }
    // short_livedのスコープが終了

    println!("short_livedのスコープ終了後、resultは使用不可");
    // println!("スコープ外での結果: {}", result); // コンパイルエラー

    // 所有権を取る版（ライフタイムの制約を回避）
    fn select_string_owned(first: &str, second: &str, prefer_longer: bool) -> String {
        let selected = if prefer_longer {
            if first.len() >= second.len() {
                first
            } else {
                second
            }
        } else {
            if first.len() <= second.len() {
                first
            } else {
                second
            }
        };
        selected.to_string() // 所有権のある値を返す
    }

    let owned_result = {
        let short_lived = String::from("短命");
        select_string_owned(&long_lived, &short_lived, true)
    };

    println!(
        "所有権版の結果（スコープ外でも使用可能）: '{}'",
        owned_result
    );

    // より複雑なライフタイム関係
    fn demonstrate_complex_lifetimes() {
        struct StringHolder<'a> {
            content: &'a str,
        }

        impl<'a> StringHolder<'a> {
            fn select_content(&self, other: &'a str) -> &'a str {
                if self.content.len() > other.len() {
                    self.content
                } else {
                    other
                }
            }
        }

        let base_string = String::from("ベース文字列");
        let holder = StringHolder {
            content: &base_string,
        };

        {
            let compare_string = String::from("比較用文字列");
            let selected = holder.select_content(&compare_string);
            println!("構造体での選択: '{}'", selected);
        }

        println!("構造体を使った複雑なライフタイム関係も同様に制約される");
    }

    demonstrate_complex_lifetimes();
}

// 実用的な応用例
fn practical_applications() {
    println!("\n【実用的な応用例】");

    // 設定ファイルの読み込みと選択
    println!("1. 設定ファイルの値選択:");

    struct ConfigSelector<'a> {
        default_config: &'a str,
        user_config: Option<&'a str>,
        env_config: Option<&'a str>,
    }

    impl<'a> ConfigSelector<'a> {
        fn get_effective_config(&self) -> &'a str {
            // 優先度: 環境変数 > ユーザー設定 > デフォルト
            if let Some(env) = self.env_config {
                env
            } else if let Some(user) = self.user_config {
                user
            } else {
                self.default_config
            }
        }

        fn get_config_with_fallback(&self, key: &str) -> &'a str {
            // 特定のキーに基づく選択ロジック
            match key {
                "theme" if self.user_config.is_some() => self.user_config.unwrap(),
                "api_url" if self.env_config.is_some() => self.env_config.unwrap(),
                _ => self.default_config,
            }
        }
    }

    let default = "default_theme";
    let user = Some("dark_theme");
    let env = Some("production_theme");

    let selector = ConfigSelector {
        default_config: default,
        user_config: user,
        env_config: env,
    };

    println!("  有効な設定: {}", selector.get_effective_config());
    println!(
        "  テーマ設定: {}",
        selector.get_config_with_fallback("theme")
    );
    println!(
        "  API URL設定: {}",
        selector.get_config_with_fallback("api_url")
    );

    // テキスト処理での応用
    println!("\n2. テキスト処理での文字列選択:");

    fn extract_best_summary<'a>(
        full_text: &'a str,
        first_sentence: &'a str,
        first_paragraph: &'a str,
        max_length: usize,
    ) -> &'a str {
        let candidates = [first_sentence, first_paragraph, full_text];

        // 最大長以下で最も長いものを選択
        candidates
            .iter()
            .filter(|&&text| text.len() <= max_length)
            .max_by_key(|&&text| text.len())
            .unwrap_or(&first_sentence) // フォールバック
    }

    let full_text =
        "これは完全なテキストです。非常に長い内容を含んでいます。詳細な説明が続きます。";
    let first_sentence = "これは完全なテキストです。";
    let first_paragraph = "これは完全なテキストです。非常に長い内容を含んでいます。";

    let summary50 = extract_best_summary(full_text, first_sentence, first_paragraph, 50);
    let summary30 = extract_best_summary(full_text, first_sentence, first_paragraph, 30);
    let summary15 = extract_best_summary(full_text, first_sentence, first_paragraph, 15);

    println!("  50文字制限: '{}'", summary50);
    println!("  30文字制限: '{}'", summary30);
    println!("  15文字制限: '{}'", summary15);

    // パフォーマンス重視の選択
    println!("\n3. パフォーマンス重視の選択:");

    fn select_fastest_algorithm<'a>(
        algorithms: &[(&'a str, fn() -> i32)],
        test_data_size: usize,
    ) -> &'a str {
        // データサイズに基づいてアルゴリズムを選択
        match test_data_size {
            0..=100 => algorithms[0].0,    // 小さいデータ: シンプルなアルゴリズム
            101..=1000 => algorithms[1].0, // 中程度: バランス型
            _ => algorithms[2].0,          // 大きいデータ: 高度なアルゴリズム
        }
    }

    fn bubble_sort() -> i32 {
        42
    }
    fn quick_sort() -> i32 {
        42
    }
    fn merge_sort() -> i32 {
        42
    }

    let algorithms = [
        ("bubble_sort", bubble_sort as fn() -> i32),
        ("quick_sort", quick_sort as fn() -> i32),
        ("merge_sort", merge_sort as fn() -> i32),
    ];

    println!(
        "  小データ(50): {}",
        select_fastest_algorithm(&algorithms, 50)
    );
    println!(
        "  中データ(500): {}",
        select_fastest_algorithm(&algorithms, 500)
    );
    println!(
        "  大データ(5000): {}",
        select_fastest_algorithm(&algorithms, 5000)
    );

    println!("\n【重要なポイント】");
    println!("✓ 戻り値のライフタイムは入力のすべてのライフタイムの交差");
    println!("✓ より短いライフタイムを持つ値がスコープを抜けると戻り値も使用不可");
    println!("✓ ライフタイムの制約を回避したい場合は所有権を取る型（String等）を使用");
    println!("✓ 実用的な場面では設定選択、テキスト処理、アルゴリズム選択等で活用");
}
