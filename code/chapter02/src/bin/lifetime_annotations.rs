// src/bin/lifetime_annotations.rs

use std::fmt::Display;

fn main() {
    println!("=== ライフタイム注釈の完全ガイド ===\n");

    basic_lifetime_annotations();
    multiple_lifetimes();
    lifetime_in_structs();
    lifetime_with_methods();
    lifetime_bounds();
}

fn basic_lifetime_annotations() {
    println!("--- 基本的なライフタイム注釈 ---");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最長の文字列: {}", result);

    // ライフタイムの制約を理解する
    let string3 = String::from("長寿命の文字列");
    let result;
    {
        let string4 = String::from("短寿命");
        result = longest(string3.as_str(), string4.as_str());
        println!("スコープ内での結果: {}", result);
        // resultはstring4のライフタイムに制約される
    }
    // println!("スコープ外: {}", result); // エラー！
}

// 単一のライフタイムパラメータ
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 返される参照のライフタイムは、xとyの短い方に制約される
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn multiple_lifetimes() {
    println!("\n--- 複数のライフタイム ---");

    let string1 = String::from("hello");
    let string2 = String::from("world");

    let result = longest_with_announcement(&string1, &string2, "比較中...");
    println!("結果: {}", result);

    // 異なるライフタイムの例
    let x = 5;
    let y = 10;
    let r = different_lifetimes(&x, &y);
    println!("最初の参照: {}", r);
}

// 複数の異なるライフタイムパラメータ
fn longest_with_announcement<'a, 'b>(x: &'a str, y: &'a str, announcement: &'b str) -> &'a str {
    println!("お知らせ: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 独立したライフタイム
fn different_lifetimes<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    // 戻り値は'aのライフタイムのみに依存
    x
}

fn lifetime_in_structs() {
    println!("\n--- 構造体のライフタイム ---");

    // ImportantExcerptの使用例
    let novel = String::from("むかしむかし、あるところに。その後、彼らは...");
    let first_sentence = novel.split('。').next().expect("文が見つかりません");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
        metadata: "第1章",
    };

    println!("重要な抜粋: {}", excerpt.part);
    println!("メタデータ: {}", excerpt.metadata);

    // 複数のライフタイムを持つ構造体
    let context = "グローバルコンテキスト";
    let data = String::from("ローカルデータ");
    let complex = ComplexStruct {
        context,
        data: &data,
        number: 42,
    };

    println!(
        "複雑な構造体: context={}, data={}, number={}",
        complex.context, complex.data, complex.number
    );
}

// ライフタイムパラメータを持つ構造体
struct ImportantExcerpt<'a> {
    part: &'a str,
    metadata: &'a str,
}

// 複数のライフタイムパラメータを持つ構造体
struct ComplexStruct<'a, 'b> {
    context: &'a str,
    data: &'b str,
    number: i32,
}

fn lifetime_with_methods() {
    println!("\n--- メソッドとライフタイム ---");

    let text = String::from("最初の部分。次の部分。");
    let first = text.split('。').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first,
        metadata: "メタ情報",
    };

    println!("レベル: {}", excerpt.level());
    println!(
        "お知らせ付き: {}",
        excerpt.announce_and_return_part("新しいお知らせ")
    );

    // ライフタイムの異なる引数
    let other = "他の文字列";
    let compared = excerpt.compare_with(other);
    println!("比較結果（長い方）: {}", compared);
}

// メソッドの実装
impl<'a> ImportantExcerpt<'a> {
    // selfのライフタイムが自動的に返り値に適用される
    fn level(&self) -> i32 {
        3
    }

    // selfのライフタイムが返り値に伝播
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注目！{}", announcement);
        self.part
    }

    // 複数のライフタイムが関わる場合
    fn compare_with<'b>(&self, other: &'b str) -> &'a str
    where
        'a: 'b, // 'aは'bより長生きする必要がある
    {
        if self.part.len() > other.len() {
            self.part
        } else {
            // otherは返せない（ライフタイムが異なる）
            self.part
        }
    }
}

fn lifetime_bounds() {
    println!("\n--- ライフタイム境界 ---");

    let string = String::from("ライフタイム境界のテスト");
    let announcement: Announcement<'_, i32> = Announcement::new(&string);
    announcement.announce();

    // ジェネリックとライフタイムの組み合わせ
    let numbers = vec![1, 2, 3, 4, 5];
    let result = find_and_announce(&numbers, "検索中...");
    println!("最初の要素: {}", result);
}

// ライフタイム境界を持つ構造体
struct Announcement<'a, T: Display> {
    message: &'a str,
    value: T,
}

impl<'a> Announcement<'a, i32> {
    fn new(message: &'a str) -> Self {
        Announcement {
            message,
            value: 42, // i32として固定
        }
    }

    fn announce(&self) {
        println!("重要なお知らせ: {} (ID: {})", self.message, self.value);
    }
}

// ジェネリック、トレイト境界、ライフタイムの組み合わせ
fn find_and_announce<'a, T: Display>(list: &'a [T], announcement: &str) -> &'a T {
    println!("{}", announcement);
    &list[0]
}
