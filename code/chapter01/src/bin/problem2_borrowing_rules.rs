// src/bin/problem2_borrowing_rules.rs
// 復習問題2: 借用チェッカーの解答

// 正しい方法1: 所有権を返す
fn no_dangle1() -> String {
    String::from("hello") // 所有権を移動
}

fn main() {
    println!("=== 復習問題2: 借用チェッカー ===\n");

    // 元のコード（コンパイルエラー）
    println!("【問題のあるコード】");
    println!("let mut data = vec![1, 2, 3];");
    println!("let r1 = &data;      // 不変参照");
    println!("let r2 = &mut data;  // 可変参照（エラー）");
    println!("println!(\"{{:?}} {{:?}}\", r1, r2);");

    println!("\n【問題点の説明】");
    println!("・不変参照（r1）と可変参照（r2）を同時に作成しようとしている");
    println!("・借用の基本ルールに違反：「1つの可変参照」または「任意の数の不変参照」");
    println!("・これによりデータ競合を防いでいる");

    // 修正方法1: 借用のタイミングをずらす
    println!("\n=== 修正方法1: 借用のタイミングをずらす ===");
    {
        let mut data = vec![1, 2, 3];
        let r1 = &data;
        println!("不変参照を使用: {:?}", r1); // r1を先に使用
                                              // r1の寿命がここで終わる（Non-Lexical Lifetimes）

        let r2 = &mut data; // r1の寿命が終わった後に可変借用
        r2.push(4);
        println!("可変参照で変更後: {:?}", r2);

        println!("解説: NLLにより、参照の最後の使用時点で寿命が終わる");
    }

    // 修正方法2: 可変参照のみ使用
    println!("\n=== 修正方法2: 可変参照のみ使用 ===");
    {
        let mut data = vec![1, 2, 3];
        let r2 = &mut data;
        println!("可変参照で読み取り: {:?}", r2);
        r2.push(4);
        println!("可変参照で変更後: {:?}", r2);

        println!("解説: 可変参照は読み取りと変更の両方が可能");
    }

    // 修正方法3: スコープを分ける
    println!("\n=== 修正方法3: スコープを分ける ===");
    {
        let mut data = vec![1, 2, 3];

        // 不変参照のスコープ
        {
            let r1 = &data;
            println!("不変参照: {:?}", r1);
        } // r1はここでスコープアウト

        // 可変参照のスコープ
        {
            let r2 = &mut data;
            r2.push(4);
            println!("可変参照: {:?}", r2);
        } // r2はここでスコープアウト

        println!("最終結果: {:?}", data);
        println!("解説: 明示的なスコープで借用の寿命を制御");
    }

    // 借用ルールの詳細な例
    println!("\n=== 借用ルールの詳細例 ===");

    // 複数の不変参照は OK
    {
        let data = vec![1, 2, 3];
        let r1 = &data;
        let r2 = &data;
        let r3 = &data;
        println!("複数の不変参照: {:?}, {:?}, {:?}", r1, r2, r3);
        println!("✓ 複数の不変参照は同時に存在可能");
    }

    // 関数呼び出しでの借用
    println!("\n=== 関数呼び出しでの借用 ===");
    {
        fn read_data(data: &Vec<i32>) {
            println!("読み取り専用: {:?}", data);
        }

        fn modify_data(data: &mut Vec<i32>) {
            data.push(999);
            println!("変更後: {:?}", data);
        }

        let mut data = vec![1, 2, 3];

        // 複数回の不変借用
        read_data(&data);
        read_data(&data);

        // 可変借用
        modify_data(&mut data);

        // 再び不変借用
        read_data(&data);

        println!("解説: 関数呼び出し時も同じ借用ルールが適用される");
    }

    // ダングリング参照の防止
    println!("\n=== ダングリング参照の防止 ===");
    {
        // この関数はコンパイルエラーになる（例として説明のみ）
        /*
        fn dangle() -> &String {
            let s = String::from("hello");
            &s  // s はここでドロップされるため、参照が無効になる
        }
        */

        // 正しい方法2: ライフタイムパラメータを使用
        fn no_dangle2(s: &String) -> &String {
            s // 借用した参照をそのまま返す
        }

        let result1 = no_dangle1();
        println!("所有権移動: {}", result1);

        let original = String::from("world");
        let result2 = no_dangle2(&original);
        println!("借用返却: {}", result2);

        println!("解説: 借用チェッカーがダングリング参照を防止");
    }

    // Non-Lexical Lifetimes（NLL）の例
    println!("\n=== Non-Lexical Lifetimes（NLL）の例 ===");
    {
        let mut vec = vec![1, 2, 3];
        let r1 = &vec[0];
        println!("最初の要素: {}", r1); // r1の最後の使用
                                        // NLLにより、r1の寿命はここで終わる

        vec.push(4); // ここで可変借用が可能
        println!("追加後: {:?}", vec);

        println!("解説: NLLにより、参照の実際の使用期間でのみ借用が有効");
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}

#[cfg(test)]
mod helper_tests {
    #[test]
    fn no_dangle1_returns_owned_string() {
        assert_eq!(super::no_dangle1(), "hello");
    }
}
