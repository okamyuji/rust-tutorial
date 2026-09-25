// src/bin/lifetime_subtyping.rs

// PhantomDataは使用されていないため削除

fn main() {
    println!("=== ライフタイムサブタイピング ===\n");

    basic_subtyping();
    subtyping_in_functions();
    subtyping_with_structs();
    contravariance_examples();
    practical_subtyping_patterns();
    subtyping_rules_summary();
}

fn basic_subtyping() {
    println!("--- ライフタイムサブタイピングの基本 ---");

    // 'static は任意のライフタイムのサブタイプ
    demonstrate_static_subtyping();

    // より長いライフタイムはより短いライフタイムのサブタイプ
    demonstrate_lifetime_hierarchy();

    // サブタイピングの推移性
    demonstrate_transitivity();
}

fn demonstrate_static_subtyping() {
    println!("\n['staticのサブタイピング]");

    // 'static → 'a の変換
    fn accept_any_lifetime<'a>(s: &'a str) -> &'a str {
        s
    }

    // 'static文字列を任意のライフタイムとして使用
    let static_str: &'static str = "永続的な文字列";
    let result = accept_any_lifetime(static_str);
    println!("結果: {}", result);

    // 構造体でのサブタイピング
    struct Container<'a> {
        data: &'a str,
    }

    // 'staticをより短いライフタイムの構造体に格納
    let static_data: &'static str = "static data";
    let container = Container { data: static_data };
    println!("コンテナ内のデータ: {}", container.data);
}

fn demonstrate_lifetime_hierarchy() {
    println!("\n[ライフタイムの階層]");

    // 外側のスコープ（より長いライフタイム 'a）
    let outer_string = String::from("外側の文字列");
    let outer_ref: &str = &outer_string;

    {
        // 内側のスコープ（より短いライフタイム 'b）
        let inner_string = String::from("内側の文字列");
        let inner_ref: &str = &inner_string;

        // 'a は 'b のサブタイプ（'a: 'b）
        fn process_refs<'a, 'b>(longer: &'a str, shorter: &'b str)
        where
            'a: 'b, // 'aは'bより長生きする
        {
            println!("長いライフタイム: {}", longer);
            println!("短いライフタイム: {}", shorter);

            // 'aを'bとして使用可能
            let _temp: &'b str = longer;
        }

        process_refs(outer_ref, inner_ref);
    }

    // outer_refはまだ有効
    println!("外側の参照はまだ有効: {}", outer_ref);
}

fn demonstrate_transitivity() {
    println!("\n[サブタイピングの推移性]");

    // 'static : 'a : 'b : 'c
    fn demonstrate_chain<'a, 'b, 'c>(s: &'a str)
    where
        'static: 'a,
        'a: 'b,
        'b: 'c,
    {
        let level_a: &'a str = s;
        let level_b: &'b str = level_a; // 'a → 'b
        let level_c: &'c str = level_b; // 'b → 'c

        println!("推移的な変換: {} → {} → {}", s, level_b, level_c);
    }

    let static_str: &'static str = "static";
    demonstrate_chain(static_str);
}

fn subtyping_in_functions() {
    println!("\n--- 関数におけるサブタイピング ---");

    // 引数のサブタイピング
    argument_subtyping();

    // 戻り値のサブタイピング
    return_value_subtyping();

    // 関数ポインタのサブタイピング
    function_pointer_subtyping();
}

fn argument_subtyping() {
    println!("\n[引数のサブタイピング]");

    // より制限的な関数に、より寛容な引数を渡す
    fn process_string<'a>(s: &'a str) -> usize {
        println!("処理中: {}", s);
        s.len()
    }

    // 'staticな文字列を渡す
    let static_str: &'static str = "static string";
    let len = process_string(static_str);
    println!("長さ: {}", len);

    // ローカルな文字列を渡す
    let local = String::from("local string");
    let len = process_string(&local);
    println!("長さ: {}", len);
}

fn return_value_subtyping() {
    println!("\n[戻り値のサブタイピング]");

    // より長いライフタイムを返す関数
    fn get_static_or_local<'a>(use_static: bool, local: &'a str) -> &'a str {
        if use_static {
            // 'static は 'a のサブタイプなので返せる
            "static string"
        } else {
            local
        }
    }

    let local = String::from("ローカル文字列");
    let result1 = get_static_or_local(true, &local);
    let result2 = get_static_or_local(false, &local);

    println!("静的を選択: {}", result1);
    println!("ローカルを選択: {}", result2);
}

fn function_pointer_subtyping() {
    println!("\n[関数ポインタのサブタイピング]");

    // 関数ポインタは反変
    type ShortLifetimeFn<'a> = fn(&'a str) -> usize;
    type LongLifetimeFn = fn(&'static str) -> usize;

    fn static_only(s: &'static str) -> usize {
        s.len()
    }

    fn any_lifetime(s: &str) -> usize {
        s.len()
    }

    // より一般的な関数を、より制限的な型として使用可能
    let fn_ptr: LongLifetimeFn = static_only;
    // let fn_ptr2: ShortLifetimeFn = static_only; // エラー：反変のため

    let _fn_ptr3: ShortLifetimeFn = any_lifetime; // OK

    let result = fn_ptr("static only");
    println!("関数ポインタの結果: {}", result);
}

fn subtyping_with_structs() {
    println!("\n--- 構造体でのサブタイピング ---");

    // 共変フィールド
    covariant_fields();

    // 不変フィールド
    invariant_fields();

    // 複雑な構造体
    complex_struct_subtyping();
}

fn covariant_fields() {
    println!("\n[共変フィールド]");

    // 不変参照は共変
    #[derive(Debug)]
    struct ReadOnly<'a> {
        data: &'a str,
    }

    // より長いライフタイムから短いライフタイムへ
    fn shorten_lifetime<'a, 'b>(ro: ReadOnly<'a>) -> ReadOnly<'b>
    where
        'a: 'b,
    {
        ReadOnly { data: ro.data }
    }

    let static_ro = ReadOnly { data: "static" };
    let shortened = shorten_lifetime(static_ro);
    println!("短縮されたライフタイム: {:?}", shortened);
}

fn invariant_fields() {
    println!("\n[不変フィールド]");

    use std::cell::Cell;

    // Cell<T>は不変
    struct Mutable<'a> {
        data: Cell<&'a str>,
    }

    // ライフタイムの変換は不可能
    // fn convert_lifetime<'a, 'b>(m: Mutable<'a>) -> Mutable<'b>
    // where 'a: 'b
    // {
    //     Mutable { data: m.data } // エラー：不変のため
    // }

    let data = String::from("mutable");
    let mutable = Mutable {
        data: Cell::new(&data),
    };

    println!("可変フィールドの現在値: {}", mutable.data.get());
    mutable.data.set("changed");
    println!("変更後: {}", mutable.data.get());
}

fn complex_struct_subtyping() {
    println!("\n[複雑な構造体のサブタイピング]");

    // 複数のライフタイムを持つ構造体
    struct Complex<'a, 'b> {
        shared: &'a str,      // 共変
        mutable: &'b mut i32, // 不変
    }

    // 部分的なサブタイピング
    fn process_complex<'a, 'b, 'c>(c: Complex<'a, 'b>) -> &'c str
    where
        'a: 'c, // sharedフィールドのライフタイムを変換
    {
        // mutableフィールドも確認
        *c.mutable = 42;
        c.shared
    }

    let shared_data = String::from("shared");
    let mut mutable_data = 42;

    let complex = Complex {
        shared: &shared_data,
        mutable: &mut mutable_data,
    };

    let result = process_complex(complex);
    println!("処理結果: {}", result);
}

fn contravariance_examples() {
    println!("\n--- 反変の例 ---");

    // 関数引数での反変
    function_argument_contravariance();

    // トレイトオブジェクトでの反変
    trait_object_contravariance();
}

fn function_argument_contravariance() {
    println!("\n[関数引数の反変]");

    // より一般的な関数は、より特殊な関数として使える
    trait Handler<'a> {
        fn handle(&self, data: &'a str);
    }

    struct GeneralHandler;
    struct SpecificHandler;

    impl<'a> Handler<'a> for GeneralHandler {
        fn handle(&self, data: &'a str) {
            println!("一般的なハンドラ: {}", data);
        }
    }

    impl Handler<'static> for SpecificHandler {
        fn handle(&self, data: &'static str) {
            println!("特定のハンドラ（staticのみ）: {}", data);
        }
    }

    // 使用例
    let general = GeneralHandler;
    let specific = SpecificHandler;

    general.handle("any lifetime");
    specific.handle("only static");
}

fn trait_object_contravariance() {
    println!("\n[トレイトオブジェクトの反変]");

    // トレイトオブジェクトも反変
    trait Processor {
        fn process(&self, data: &str) -> String;
    }

    struct UppercaseProcessor;

    impl Processor for UppercaseProcessor {
        fn process(&self, data: &str) -> String {
            data.to_uppercase()
        }
    }

    // Box<dyn Processor>として使用
    let processor: Box<dyn Processor> = Box::new(UppercaseProcessor);
    let result = processor.process("hello");
    println!("処理結果: {}", result);
}

fn practical_subtyping_patterns() {
    println!("\n--- 実践的なサブタイピングパターン ---");

    // APIデザインでの活用
    api_design_patterns();

    // エラー処理での活用
    error_handling_patterns();

    // ビルダーパターンでの活用
    builder_pattern_with_lifetimes();
}

fn api_design_patterns() {
    println!("\n[APIデザインパターン]");

    // 柔軟なAPIの設計
    struct Cache<'a> {
        data: Vec<&'a str>,
    }

    impl<'a> Cache<'a> {
        fn new() -> Self {
            Cache { data: Vec::new() }
        }

        // より長いライフタイムを受け入れる
        fn insert<'b>(&mut self, item: &'b str)
        where
            'b: 'a, // 'bは'aより長生きする必要がある
        {
            self.data.push(item);
        }

        // 格納されたライフタイムで返す
        fn get(&self, index: usize) -> Option<&'a str> {
            self.data.get(index).copied()
        }
    }

    let mut cache = Cache::new();

    // 'static文字列を挿入
    cache.insert("static item");

    // ローカル文字列を挿入
    let local = String::from("local item");
    cache.insert(&local);

    // getメソッドを使用
    if let Some(item) = cache.get(0) {
        println!("最初のアイテム: {}", item);
    }

    println!("キャッシュサイズ: {}", cache.data.len());
}

fn error_handling_patterns() {
    println!("\n[エラー処理パターン]");

    // エラー型でのライフタイム
    enum Error<'a> {
        NotFound(&'a str),
        InvalidInput(&'a str),
    }

    fn validate<'a>(input: &'a str) -> Result<(), Error<'a>> {
        if input.is_empty() {
            Err(Error::InvalidInput("入力が空です"))
        } else if !input.chars().all(|c| c.is_alphanumeric()) {
            Err(Error::InvalidInput(input))
        } else {
            Ok(())
        }
    }

    fn search<'a>(database: &[&'a str], query: &'a str) -> Result<&'a str, Error<'a>> {
        for item in database {
            if item.contains(query) {
                return Ok(item);
            }
        }
        Err(Error::NotFound(query))
    }

    match validate("test123") {
        Ok(()) => println!("検証成功"),
        Err(Error::NotFound(msg)) => println!("見つかりません: {}", msg),
        Err(Error::InvalidInput(msg)) => println!("無効な入力: {}", msg),
    }

    // search関数の使用例
    let database = vec!["apple", "banana", "cherry"];
    match search(&database, "ban") {
        Ok(found) => println!("検索結果: {}", found),
        Err(Error::NotFound(query)) => println!("「{}」が見つかりません", query),
        Err(Error::InvalidInput(msg)) => println!("無効な入力: {}", msg),
    }
}

fn builder_pattern_with_lifetimes() {
    println!("\n[ビルダーパターン]");

    // ライフタイム付きビルダー
    struct ConfigBuilder<'a> {
        name: Option<&'a str>,
        value: Option<&'a str>,
    }

    struct Config<'a> {
        name: &'a str,
        value: &'a str,
    }

    impl<'a> ConfigBuilder<'a> {
        fn new() -> Self {
            ConfigBuilder {
                name: None,
                value: None,
            }
        }

        // より長いライフタイムを受け入れる
        fn name<'b>(mut self, name: &'b str) -> Self
        where
            'b: 'a,
        {
            self.name = Some(name);
            self
        }

        fn value<'b>(mut self, value: &'b str) -> Self
        where
            'b: 'a,
        {
            self.value = Some(value);
            self
        }

        fn build(self) -> Result<Config<'a>, &'static str> {
            match (self.name, self.value) {
                (Some(name), Some(value)) => Ok(Config { name, value }),
                _ => Err("不完全な設定"),
            }
        }
    }

    // 使用例
    let config = ConfigBuilder::new()
        .name("アプリ名")
        .value("設定値")
        .build()
        .unwrap();

    println!("設定: {} = {}", config.name, config.value);

    // 異なるライフタイムの組み合わせ
    let static_name = "static_name";
    let local_value = String::from("local_value");

    let config2 = ConfigBuilder::new()
        .name(static_name) // 'static
        .value(&local_value) // ローカル
        .build()
        .unwrap();

    println!("混合設定: {} = {}", config2.name, config2.value);
}

// サブタイピング規則のまとめ
fn subtyping_rules_summary() {
    println!("\n=== サブタイピング規則のまとめ ===");

    // 1. 基本規則
    // - 'static : 'a （'staticは任意のライフタイムのサブタイプ）
    // - 'a : 'b ならば、&'a T は &'b T として使える（共変）

    // 2. 変性規則
    // - &'a T: 共変（'a に関して）
    // - &'a mut T: 不変（'a に関して）
    // - fn(&'a T): 反変（'a に関して）

    // 3. 複合型の変性
    // - Option<&'a T>: 共変
    // - Vec<&'a T>: 共変
    // - Cell<&'a T>: 不変

    // 4. 実用的な意味
    // - 共変：より長いライフタイムから短いライフタイムへ変換可能
    // - 反変：より短いライフタイムから長いライフタイムへ変換可能
    // - 不変：ライフタイムの変換不可
}
