// src/bin/problem3_trait_bounds.rs
// 復習問題3: 複数トレイト境界を持つジェネリック関数

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

fn main() {
    println!("=== 復習問題3: 複数トレイト境界 ===\n");

    // 基本的なトレイト境界
    basic_trait_bounds();

    // 複雑なトレイト境界
    complex_trait_bounds();

    // where句の活用
    where_clause_examples();

    // 実用的な応用例
    practical_applications();
}

// 基本的なトレイト境界の例
fn basic_trait_bounds() {
    println!("【基本的なトレイト境界】");

    // 問題の要件：Display + Debug を受け取り、Clone + Default を返す
    fn process_and_convert<T, U>(input: T) -> U
    where
        T: Display + Debug + Clone,
        U: Clone + Default + From<String>,
    {
        println!("デバッグ表示: {:?}", input);
        println!("通常表示: {}", input);

        let cloned = input.clone();
        println!("クローン後のデバッグ表示: {:?}", cloned);

        let string_repr = format!("{}", input);
        U::from(string_repr)
    }

    // 使用例
    let number = 42;
    let result: String = process_and_convert(number);
    println!("変換結果: {}\n", result);

    let text = "Hello, World!";
    let result2: String = process_and_convert(text);
    println!("文字列変換結果: {}\n", result2);

    // より複雑な変換例
    fn advanced_conversion<T, U, V>(input1: T, input2: U, combiner: fn(&str, &str) -> String) -> V
    where
        T: Display + Debug + Clone,
        U: Display + Debug + Into<String>,
        V: From<String> + Clone + Default + Debug,
    {
        println!("入力1のデバッグ: {:?}", input1);
        println!("入力2のデバッグ: {:?}", input2);

        let str1 = format!("{}", input1);
        let str2 = input2.into();

        let combined = combiner(&str1, &str2);
        println!("結合結果: {}", combined);

        let result = V::from(combined);
        println!("最終結果: {:?}", result);
        result
    }

    let num = 123;
    let text = "test".to_string();
    let combiner = |a: &str, b: &str| format!("{}_{}", a, b);

    let final_result: String = advanced_conversion(num, text, combiner);
    println!("アドバンス変換: {}", final_result);
}

// 複雑なトレイト境界の例
fn complex_trait_bounds() {
    println!("\n【複雑なトレイト境界】");

    // 多重制約の例
    fn multi_constraint_function<T, K, V>(
        data: T,
        key_extractor: fn(&T) -> K,
        value_extractor: fn(&T) -> V,
    ) -> HashMap<K, V>
    where
        T: Debug + Clone,
        K: Hash + Eq + Debug + Clone,
        V: Debug + Clone,
    {
        println!("処理対象データ: {:?}", data);

        let key = key_extractor(&data);
        let value = value_extractor(&data);

        println!("抽出されたキー: {:?}", key);
        println!("抽出された値: {:?}", value);

        let mut map = HashMap::new();
        map.insert(key, value);
        map
    }

    #[derive(Debug, Clone)]
    struct Person {
        id: u32,
        name: String,
        age: u32,
    }

    let person = Person {
        id: 1,
        name: "Alice".to_string(),
        age: 30,
    };

    let name_map = multi_constraint_function(person.clone(), |p| p.id, |p| p.name.clone());

    let age_map = multi_constraint_function(person, |p| p.name.clone(), |p| p.age);

    println!("名前マップ: {:?}", name_map);
    println!("年齢マップ: {:?}", age_map);

    // 条件付きトレイト境界
    fn conditional_processing<T>(items: Vec<T>) -> (Vec<T>, Vec<String>)
    where
        T: Display + Debug + Clone + PartialOrd,
    {
        println!("アイテム数: {}", items.len());

        let mut sorted_items = items.clone();
        sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let string_representations: Vec<String> =
            items.iter().map(|item| format!("{}", item)).collect();

        println!("ソート済みアイテム: {:?}", sorted_items);

        (sorted_items, string_representations)
    }

    let numbers = vec![std::f64::consts::PI, 1.41, 2.71, 0.57];
    let (sorted, strings) = conditional_processing(numbers);
    println!("ソート結果: {:?}", sorted);
    println!("文字列表現: {:?}", strings);
}

// where句の効果的な活用
fn where_clause_examples() {
    println!("\n【where句の効果的な活用】");

    // 複雑な制約を読みやすく
    fn complex_where_clause<T, U, V, W>(
        source: T,
        transformer: U,
        validator: V,
    ) -> Result<W, String>
    where
        T: Debug + Clone + Display,
        U: Fn(T) -> Result<W, String>,
        V: Fn(&W) -> bool,
        W: Debug + Clone + Default + PartialEq,
    {
        println!("ソースデータ: {} ({:?})", source, source);

        match transformer(source) {
            Ok(result) => {
                println!("変換結果: {:?}", result);

                if validator(&result) {
                    println!("バリデーション成功");
                    Ok(result)
                } else {
                    println!("バリデーション失敗");
                    Err("バリデーションエラー".to_string())
                }
            }
            Err(e) => {
                println!("変換エラー: {}", e);
                Err(e)
            }
        }
    }

    // トランスフォーマー関数
    let string_to_number = |s: String| -> Result<i32, String> {
        s.parse::<i32>().map_err(|_| "数値解析エラー".to_string())
    };

    // バリデーター関数
    let positive_validator = |n: &i32| -> bool { *n > 0 };

    // 使用例
    println!("有効な数値の例:");
    match complex_where_clause("42".to_string(), string_to_number, positive_validator) {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失敗: {}", e),
    }

    println!("\n無効な数値の例:");
    match complex_where_clause("-10".to_string(), string_to_number, positive_validator) {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失敗: {}", e),
    }

    println!("\n解析エラーの例:");
    match complex_where_clause("abc".to_string(), string_to_number, positive_validator) {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失敗: {}", e),
    }

    // 高階トレイト境界との組み合わせ
    fn higher_order_bounds<F, G, T, U>(data: Vec<T>, processor: F, combiner: G) -> U
    where
        F: for<'a> Fn(&'a T) -> String,
        G: Fn(Vec<String>) -> U,
        T: Debug,
        U: Debug,
    {
        println!("処理対象: {:?}", data);

        let processed: Vec<String> = data.iter().map(processor).collect();

        println!("処理済み文字列: {:?}", processed);

        let result = combiner(processed);
        println!("最終結果: {:?}", result);
        result
    }

    let numbers = vec![1, 2, 3, 4, 5];

    let processor = |n: &i32| format!("num_{}", n);
    let combiner = |strings: Vec<String>| strings.join(", ");

    let combined: String = higher_order_bounds(numbers, processor, combiner);
    println!("高階関数結果: {}", combined);
}

// 実用的な応用例
fn practical_applications() {
    println!("\n【実用的な応用例】");

    // データ変換パイプライン
    trait Pipeline<Input, Output> {
        type Error;

        fn process(&self, input: Input) -> Result<Output, Self::Error>;
    }

    // 具体的なパイプライン実装
    struct ValidationPipeline<T, V, C> {
        validator: V,
        converter: C,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, V, C> ValidationPipeline<T, V, C> {
        fn new(validator: V, converter: C) -> Self {
            ValidationPipeline {
                validator,
                converter,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U, V, C> Pipeline<T, U> for ValidationPipeline<T, V, C>
    where
        T: Debug + Clone,
        U: Debug,
        V: Fn(&T) -> bool,
        C: Fn(T) -> Result<U, String>,
    {
        type Error = String;

        fn process(&self, input: T) -> Result<U, Self::Error> {
            println!("入力をバリデーション: {:?}", input);

            if !(self.validator)(&input) {
                return Err("バリデーション失敗".to_string());
            }

            println!("バリデーション成功、変換開始");
            (self.converter)(input)
        }
    }

    // パイプライン使用例
    let email_validator = |email: &String| -> bool { email.contains('@') && email.contains('.') };

    let email_processor = |email: String| -> Result<String, String> {
        if email.len() > 50 {
            Err("メールアドレスが長すぎます".to_string())
        } else {
            Ok(email.to_lowercase())
        }
    };

    let email_pipeline = ValidationPipeline::new(email_validator, email_processor);

    let emails = vec![
        "user@example.com".to_string(),
        "invalid-email".to_string(),
        "UPPERCASE@DOMAIN.COM".to_string(),
    ];

    println!("メール処理パイプライン:");
    for email in emails {
        match email_pipeline.process(email.clone()) {
            Ok(processed) => println!("  {} -> {}", email, processed),
            Err(e) => println!("  {} -> エラー: {}", email, e),
        }
    }

    // ジェネリックな集約関数
    fn aggregate_data<T, K, V, F, G>(
        items: Vec<T>,
        key_extractor: F,
        value_reducer: G,
    ) -> HashMap<K, V>
    where
        T: Debug + Clone,
        K: Hash + Eq + Debug + Clone,
        V: Debug + Clone + Default,
        F: Fn(&T) -> K,
        G: Fn(V, &T) -> V,
    {
        println!("集約対象アイテム: {:?}", items);

        let mut result = HashMap::new();

        for item in &items {
            let key = key_extractor(item);
            let current_value = result.get(&key).cloned().unwrap_or_default();
            let new_value = value_reducer(current_value, item);
            result.insert(key, new_value);
        }

        println!("集約結果: {:?}", result);
        result
    }

    #[derive(Debug, Clone)]
    struct Sale {
        product: String,
        amount: f64,
        quantity: i32,
    }

    let sales = vec![
        Sale {
            product: "Apple".to_string(),
            amount: 1.0,
            quantity: 10,
        },
        Sale {
            product: "Banana".to_string(),
            amount: 0.5,
            quantity: 20,
        },
        Sale {
            product: "Apple".to_string(),
            amount: 1.0,
            quantity: 15,
        },
        Sale {
            product: "Orange".to_string(),
            amount: 0.8,
            quantity: 12,
        },
        Sale {
            product: "Banana".to_string(),
            amount: 0.5,
            quantity: 8,
        },
    ];

    // 商品別数量集計
    println!("\n商品別数量集計:");
    let _quantity_by_product = aggregate_data(
        sales.clone(),
        |sale| sale.product.clone(),
        |acc: i32, sale| acc + sale.quantity,
    );

    // 商品別売上集計
    println!("\n商品別売上集計:");
    let revenue_by_product = aggregate_data(
        sales,
        |sale| sale.product.clone(),
        |acc: f64, sale| acc + (sale.amount * sale.quantity as f64),
    );

    for (product, revenue) in revenue_by_product {
        println!("  {}: ${:.2}", product, revenue);
    }

    // 制約付きソート関数
    fn constrained_sort<T, F>(mut items: Vec<T>, key_extractor: F) -> Vec<T>
    where
        T: Debug + Clone,
        F: Fn(&T) -> String,
    {
        println!("ソート前: {:?}", items);

        items.sort_by(|a, b| {
            let key_a = key_extractor(a);
            let key_b = key_extractor(b);
            key_a.cmp(&key_b)
        });

        println!("ソート後: {:?}", items);
        items
    }

    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        grade: i32,
    }

    let students = vec![
        Student {
            name: "Charlie".to_string(),
            grade: 85,
        },
        Student {
            name: "Alice".to_string(),
            grade: 92,
        },
        Student {
            name: "Bob".to_string(),
            grade: 78,
        },
    ];

    println!("\n名前順ソート:");
    let _sorted_by_name = constrained_sort(students.clone(), |s| s.name.clone());

    println!("\n成績順ソート:");
    let _sorted_by_grade = constrained_sort(students, |s| format!("{:03}", s.grade));

    println!("\n【設計パターンのポイント】");
    println!("✓ 複数の制約を組み合わせた柔軟な関数設計");
    println!("✓ where句による読みやすい制約の記述");
    println!("✓ 高階関数と組み合わせた再利用可能な設計");
    println!("✓ エラーハンドリングの組み込み");
    println!("✓ 実用的なドメインロジックへの適用");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
