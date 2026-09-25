// src/bin/problem1_lifetime_annotations.rs
// 復習問題1: ライフタイム注釈の解答

fn main() {
    println!("=== 復習問題1: ライフタイム注釈 ===\n");

    // 基本的なライフタイム注釈
    basic_lifetime_example();

    // より複雑な例
    complex_lifetime_example();

    // ライフタイムとスコープ
    scope_demonstration();

    // 複数のライフタイムパラメータ
    multiple_lifetimes_example();
}

// 基本的なライフタイム注釈の例
fn basic_lifetime_example() {
    // 元の問題のあるコード（コンパイルエラー）
    /*
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() { x } else { y }
    }
    */

    // 修正されたコード
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("【基本的なライフタイム注釈】");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最長の文字列: '{}'", result);

    // より詳細な例
    let string3 = String::from("Hello, world!");
    let string4 = String::from("Rust programming language");

    let longer = longest(&string3, &string4);
    println!("より長い文字列: '{}'", longer);

    println!("\n解説:");
    println!("・<'a>でライフタイムパラメータを宣言");
    println!("・両方の入力パラメータに同じライフタイム'aを付与");
    println!("・戻り値も同じライフタイム'aを持つ");
    println!("・戻り値のライフタイムは両方の入力の共通部分となる");
}

// より複雑なライフタイムの例
fn complex_lifetime_example() {
    // 構造体とライフタイム
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        // ライフタイム省略により、selfのライフタイムが戻り値に使用される
        fn level(&self) -> i32 {
            3
        }

        // 明示的なライフタイム注釈
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("お知らせ: {}", announcement);
            self.part
        }
    }

    println!("\n【構造体とライフタイム】");

    let novel = String::from("吾輩は猫である。名前はまだ無い。");
    let first_sentence = novel.split('。').next().expect("文が見つかりませんでした");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("抜粋: {:?}", excerpt);
    println!("レベル: {}", excerpt.level());

    let returned_part = excerpt.announce_and_return_part("重要な文章です");
    println!("返却された部分: '{}'", returned_part);
}

// ライフタイムとスコープの関係
fn scope_demonstration() {
    println!("\n【ライフタイムとスコープ】");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("内側スコープでの結果: {}", result);

        // この時点では両方の文字列が生きているのでOK
    }

    // string2のスコープが終了したため、resultは使用不可
    // println!("外側スコープでの結果: {}", result); // コンパイルエラー

    println!("string2のスコープが終了したため、resultは使用不可");

    // 正しいパターン：より長いライフタイムを持つ値を返す
    let _string3 = String::from("short");
    let _result2;

    {
        let string4 = String::from("even shorter");
        _result2 = longest(string1.as_str(), string4.as_str());
        println!("result2を使用: {}", _result2);
        // string1の方が長いので、string1への参照が返される可能性が高い
    }

    // しかし、ライフタイムはより短い方に制限されるため、依然として使用不可
    // println!("結果2: {}", result2); // コンパイルエラー

    println!("ライフタイムは常により短い方に制限される");
}

// 複数のライフタイムパラメータ
fn multiple_lifetimes_example() {
    println!("\n【複数のライフタイムパラメータ】");

    // 異なるライフタイムを持つ関数
    fn first_word_or_default<'a, 'b>(text: &'a str, default: &'b str) -> &'a str
    where
        'b: 'a, // 'bは'aより長いかまたは等しい
    {
        let first_word = text.split_whitespace().next();
        match first_word {
            Some(word) => word,
            None => default, // 'b: 'aの制約により、ここで'bを'aとして使用可能
        }
    }

    let text = String::from("Hello world from Rust");
    let default_word = "default"; // 'static ライフタイム

    let result = first_word_or_default(&text, default_word);
    println!("最初の単語またはデフォルト: '{}'", result);

    // 戻り値と入力の関係を明示的にする関数
    fn combine_strings<'a, 'b>(first: &'a str, second: &'b str) -> (String, &'a str, &'b str) {
        let combined = format!("{} {}", first, second);
        (combined, first, second)
    }

    let string1 = String::from("First");
    let string2 = String::from("Second");

    let (combined, ref1, ref2) = combine_strings(&string1, &string2);
    println!("結合: {}", combined);
    println!("参照1: {}, 参照2: {}", ref1, ref2);

    // ライフタイム境界の例
    fn process_strings<'a>(strings: &[&'a str]) -> Vec<&'a str>
    where
        'a: 'static, // 'aは'staticと同じかそれより長い（つまり'static）
    {
        strings.iter().filter(|s| s.len() > 3).copied().collect()
    }

    let static_strings = vec!["Hello", "world", "Rust", "is", "awesome"];
    let filtered = process_strings(&static_strings);
    println!("フィルタされた文字列: {:?}", filtered);

    println!("\n解説:");
    println!("・複数のライフタイムパラメータで柔軟な制約を表現");
    println!("・'b: 'aは「'bは'aより長いかまたは等しい」という意味");
    println!("・戻り値の各部分に適切なライフタイムを指定可能");
}
