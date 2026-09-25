// src/bin/problem4_variance.rs
// 復習問題4: 変性（Variance）の解答

fn main() {
    println!("=== 復習問題4: 変性（Variance）===\n");

    // 共変性の説明
    covariance_explanation();

    // 不変性の説明
    invariance_explanation();

    // 反変性の説明
    contravariance_explanation();

    // 実践的な例
    practical_variance_examples();

    // 変性の安全性
    variance_safety_examples();
}

// 共変性の説明
fn covariance_explanation() {
    println!("【共変性（Covariance）】");
    println!("&'a T が共変である理由\n");

    // ライフタイムの階層関係
    println!("ライフタイムの階層:");
    println!("'static > 'a > 'b > 'c （より長い > より短い）");

    // 共変性の例
    fn demonstrate_covariance() {
        let static_str: &'static str = "これは静的文字列です";

        // 'static から 'a への安全な変換（共変）
        {
            let _local_var = String::from("ローカル変数");
            let shorter_ref: &str = static_str; // 'static → 'a への変換

            println!("静的文字列: {}", static_str);
            println!("短いライフタイムでの参照: {}", shorter_ref);

            // これは安全：より長いライフタイムから短いライフタイムへの変換
        }

        // より具体的な例
        fn accept_string_ref(s: &str) {
            println!("受け取った文字列: {}", s);
        }

        // 'static な文字列を任意のライフタイムを期待する関数に渡せる
        accept_string_ref(static_str); // 'static → 'a への暗黙的変換

        let owned = String::from("所有された文字列");
        accept_string_ref(&owned); // 'owned → 'a への変換
    }

    demonstrate_covariance();

    // 共変性の型レベルでの表現
    println!("\n型レベルでの共変性:");

    struct CovariantBox<'a> {
        content: &'a str,
    }

    fn demonstrate_covariant_struct() {
        let static_content: &'static str = "静的コンテンツ";
        let static_box = CovariantBox {
            content: static_content,
        };

        // より短いライフタイムを期待する関数に渡せる
        fn process_box(box_ref: CovariantBox) {
            println!("ボックスの内容: {}", box_ref.content);
        }

        process_box(static_box); // 'static → 'a への共変変換
    }

    demonstrate_covariant_struct();

    println!("\n✓ 不変参照(&T)は共変");
    println!("✓ より長いライフタイムからより短いライフタイムへの変換は安全");
    println!("✓ 読み取り専用なので、データの整合性が保たれる");
}

// 不変性の説明
fn invariance_explanation() {
    println!("\n【不変性（Invariance）】");
    println!("&'a mut T が不変である理由\n");

    // 不変性が必要な理由を示す危険な例（概念的）
    println!("もし可変参照が共変だったら起こりうる問題:");

    /*
    // これは実際にはコンパイルできない危険なコード
    fn dangerous_if_covariant() {
        let mut long_lived = String::from("長生きする文字列");
        let long_ref: &'static mut String = &mut long_lived; // 仮想的

        {
            let mut short_lived = String::from("短命な文字列");
            // もし共変が許可されたら...
            let short_ref: &mut String = long_ref; // 'static → 'local
            *short_ref = short_lived; // 危険！
        }
        // short_livedは解放されているが、long_refが参照している！
    }
    */

    println!("1. 長いライフタイムの可変参照を短いライフタイムに変換");
    println!("2. 短いライフタイムの値を長いライフタイムの場所に書き込み");
    println!("3. 短いライフタイムの値が解放される");
    println!("4. 長いライフタイムの参照がダングリングポインタになる");

    // 実際の不変性の動作
    println!("\n実際の不変性の動作:");

    fn demonstrate_invariance() {
        let mut s1 = String::from("文字列1");
        let mut s2 = String::from("文字列2");

        // 同じライフタイムでのみ可変参照を扱える
        fn swap_strings(a: &mut String, b: &mut String) {
            std::mem::swap(a, b);
        }

        println!("交換前: s1='{}', s2='{}'", s1, s2);
        swap_strings(&mut s1, &mut s2);
        println!("交換後: s1='{}', s2='{}'", s1, s2);

        // 異なるライフタイムの混合は不可
        /*
        {
            let mut short_lived = String::from("短命");
            // これはコンパイルエラー（ライフタイムの不一致）
            // swap_strings(&mut s1, &mut short_lived);
        }
        */

        println!("異なるライフタイムの可変参照は混合できない");
    }

    demonstrate_invariance();

    // Cell/RefCellも不変
    use std::cell::Cell;

    fn demonstrate_cell_invariance() {
        println!("\nCell/RefCellの不変性:");

        let _cell: Cell<&'static str> = Cell::new("静的文字列");
        println!("Cellを作成しました");

        // Cellの型パラメータは不変
        fn process_cell(c: Cell<&str>) {
            println!("セルの値: {}", c.get());
        }

        // これはコンパイルエラーになる
        // process_cell(cell); // Cell<&'static str> は Cell<&str> に変換できない

        // 明示的な型注釈で回避
        let cell_any: Cell<&str> = Cell::new("任意のライフタイム");
        process_cell(cell_any);

        println!("Cell<T>は不変なので、ライフタイム変換は不可");
    }

    demonstrate_cell_invariance();

    println!("\n✓ 可変参照(&mut T)は不変");
    println!("✓ Cell<T>、RefCell<T>も不変");
    println!("✓ 書き込み可能な型では安全性のため変換を禁止");
}

// 反変性の説明
fn contravariance_explanation() {
    println!("\n【反変性（Contravariance）】");
    println!("関数の引数位置での反変性\n");

    // 関数型での反変性
    println!("関数型における反変性:");

    fn demonstrate_contravariance() {
        // より具体的な関数をより抽象的な関数として使用
        fn process_static_str(s: &'static str) {
            println!("静的文字列を処理: {}", s);
        }

        fn process_any_str(s: &str) {
            println!("任意の文字列を処理: {}", s);
        }

        // 高階関数
        fn apply_processor<F>(processor: F, input: &str)
        where
            F: Fn(&str),
        {
            processor(input);
        }

        let test_input = "テスト文字列";

        // より長いライフタイムを受け取る関数を、
        // より短いライフタイムを期待する場所で使用可能
        apply_processor(process_any_str, test_input);

        // 静的文字列のみを受け取る関数の使用例
        process_static_str("これは静的文字列です");

        // 静的文字列のみを受け取る関数は使用できない
        // apply_processor(process_static_str, test_input); // エラー

        println!("関数の引数では、より長いライフタイムを受け取る関数が");
        println!("より短いライフタイムを期待する場所で使用可能（反変）");
    }

    demonstrate_contravariance();

    // トレイトオブジェクトでの反変性
    println!("\nトレイトオブジェクトでの反変性:");

    trait Processor {
        fn process(&self, input: &str);
    }

    struct StaticProcessor;
    impl Processor for StaticProcessor {
        fn process(&self, input: &str) {
            println!("静的プロセッサ: {}", input);
        }
    }

    fn use_processor(processor: &dyn Processor, data: &str) {
        processor.process(data);
    }

    let processor = StaticProcessor;
    let data = String::from("データ");
    use_processor(&processor, &data);

    println!("トレイトオブジェクトでも同様の反変性が適用される");

    println!("\n✓ 関数の引数位置は反変");
    println!(
        "✓ より長いライフタイムを受け取る関数が、より短いライフタイムを期待する場所で使用可能"
    );
    println!("✓ これにより関数の再利用性が向上");
}

// 実践的な変性の例
fn practical_variance_examples() {
    println!("\n【実践的な変性の例】");

    // イテレータでの共変性
    println!("イテレータでの共変性:");

    fn demonstrate_iterator_covariance() {
        let static_strs: Vec<&'static str> = vec!["hello", "world", "rust"];

        // 'static なイテレータを任意のライフタイムを期待する関数に渡せる
        fn process_str_iter<'a, I>(iter: I)
        where
            I: Iterator<Item = &'a str>,
        {
            for item in iter {
                println!("  項目: {}", item);
            }
        }

        process_str_iter(static_strs.iter().copied());

        println!("Iterator<Item = &'static str> → Iterator<Item = &'a str> への変換");
    }

    demonstrate_iterator_covariance();

    // コレクションでの共変性
    println!("\nコレクションでの共変性:");

    fn demonstrate_collection_covariance() {
        let static_vec: Vec<&'static str> = vec!["項目1", "項目2", "項目3"];

        fn print_string_slice(slice: &[&str]) {
            println!("  スライス: {:?}", slice);
        }

        // Vec<&'static str> → &[&str] への変換
        print_string_slice(&static_vec);

        println!("Vec<&'static str> → &[&str] への共変変換");
    }

    demonstrate_collection_covariance();

    // Option/Resultでの共変性
    println!("\nOption/Resultでの共変性:");

    fn demonstrate_option_covariance() {
        let static_option: Option<&'static str> = Some("オプション値");

        fn process_option(opt: Option<&str>) -> String {
            match opt {
                Some(s) => format!("値: {}", s),
                None => "値なし".to_string(),
            }
        }

        let result = process_option(static_option);
        println!("  {}", result);

        println!("Option<&'static str> → Option<&str> への共変変換");
    }

    demonstrate_option_covariance();
}

// 変性の安全性の例
fn variance_safety_examples() {
    println!("\n【変性の安全性】");

    // 安全な共変変換
    println!("安全な共変変換:");

    fn safe_covariance_example() {
        let long_lived = String::from("長い期間生きる文字列");

        {
            let short_lived = String::from("短い期間の文字列");

            // より長いライフタイムの参照をより短いスコープで使用（安全）
            fn use_in_short_scope(s: &str) {
                println!("短いスコープで使用: {}", s);
            }

            use_in_short_scope(&long_lived); // 安全
            use_in_short_scope(&short_lived); // 安全

            // 短いスコープ終了
        }

        // long_livedはまだ有効
        println!("long_livedは依然として有効: {}", long_lived);
    }

    safe_covariance_example();

    // 不変性による安全性
    println!("\n不変性による安全性:");

    fn invariance_safety_example() {
        let mut data1 = vec![1, 2, 3];
        let mut data2 = vec![4, 5, 6];

        // 同じライフタイムの可変参照のみ扱える
        fn safe_swap(a: &mut Vec<i32>, b: &mut Vec<i32>) {
            std::mem::swap(a, b);
        }

        println!("交換前: data1={:?}, data2={:?}", data1, data2);
        safe_swap(&mut data1, &mut data2);
        println!("交換後: data1={:?}, data2={:?}", data1, data2);

        // 異なるライフタイムの混合は型システムが防ぐ
        {
            let _temp = [7, 8, 9];
            // safe_swap(&mut data1, &mut temp); // コンパイルエラー
            println!("異なるライフタイムの可変参照は混合不可（型システムが防ぐ）");
        }
    }

    invariance_safety_example();

    // PhantomDataでの変性制御
    use std::marker::PhantomData;

    println!("\nPhantomDataでの変性制御:");

    struct CovariantStruct<'a> {
        _marker: PhantomData<&'a ()>,
        data: String,
    }

    struct InvariantStruct<'a> {
        _marker: PhantomData<&'a mut ()>,
        data: String,
    }

    fn phantom_data_example() {
        let static_cov = CovariantStruct {
            _marker: PhantomData,
            data: "共変構造体".to_string(),
        };

        let static_inv = InvariantStruct {
            _marker: PhantomData,
            data: "不変構造体".to_string(),
        };

        // dataフィールドを使用
        println!("共変構造体のデータ: {}", static_cov.data);
        println!("不変構造体のデータ: {}", static_inv.data);

        fn use_covariant(_: CovariantStruct) {
            println!("共変構造体を使用");
        }

        fn use_invariant(_: InvariantStruct<'static>) {
            println!("不変構造体を使用");
        }

        use_covariant(static_cov); // 共変なので変換可能
        use_invariant(static_inv); // 不変なので正確な型が必要

        println!("PhantomDataで変性を明示的に制御可能");
    }

    phantom_data_example();

    println!("\n【まとめ】");
    println!("✓ 共変(&T): 安全な縮小変換（読み取り専用）");
    println!("✓ 不変(&mut T, Cell<T>): 安全性のため変換禁止（書き込み可能）");
    println!("✓ 反変(関数引数): 安全な拡大変換（より汎用的な関数を特殊な場所で使用）");
    println!("✓ Rustの型システムが変性を通じてメモリ安全性を保証");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
