// src/bin/lifetime_elision.rs

fn main() {
    println!("=== ライフタイム省略規則 ===\n");

    demonstrate_elision_rules();
    input_lifetime_rule();
    output_lifetime_rule();
    method_lifetime_rule();
    when_elision_fails();
    elision_summary();
}

fn demonstrate_elision_rules() {
    println!("--- ライフタイム省略の基本 ---");

    // 省略可能な例
    let s = "hello";
    let first = first_word(s);
    println!("最初の単語: {}", first);

    // 複数の参照を受け取る場合
    let s1 = "hello";
    let s2 = "world";
    let result = string_operation(s1, s2);
    println!("操作結果: {}", result);
}

// 規則1と2が適用される：入力が1つなので出力に同じライフタイムが適用
fn first_word(s: &str) -> &str {
    // 完全な注釈: fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 規則1が適用：各入力に別々のライフタイムが割り当てられる
fn string_operation(s1: &str, s2: &str) -> String {
    // 完全な注釈: fn string_operation<'a, 'b>(s1: &'a str, s2: &'b str) -> String
    format!("{} + {}", s1, s2)
}

fn input_lifetime_rule() {
    println!("\n--- 規則1: 入力ライフタイム ---");

    let x = 5;
    let y = 10;

    // 各参照パラメータは独自のライフタイムを持つ
    print_values(&x, &y);

    let s1 = "first";
    let s2 = "second";
    let s3 = "third";
    print_three_refs(s1, s2, s3);
}

// 規則1の例：3つの入力、3つの異なるライフタイム
fn print_three_refs(s1: &str, s2: &str, s3: &str) {
    // 完全な注釈: fn print_three_refs<'a, 'b, 'c>(s1: &'a str, s2: &'b str, s3: &'c str)
    println!("3つの参照: {}, {}, {}", s1, s2, s3);
}

fn print_values(x: &i32, y: &i32) {
    // 完全な注釈: fn print_values<'a, 'b>(x: &'a i32, y: &'b i32)
    println!("値: x={}, y={}", x, y);
}

fn output_lifetime_rule() {
    println!("\n--- 規則2: 出力ライフタイム（単一入力） ---");

    let data = vec![1, 2, 3, 4, 5];
    let first = get_first(&data);
    println!("最初の要素: {:?}", first);

    let text = String::from("Hello, world!");
    let trimmed = trim_string(&text);
    println!("トリムされた文字列: '{}'", trimmed);
}

// 規則2の例：入力が1つなので、その入力のライフタイムが出力に適用される
fn get_first<T>(data: &[T]) -> Option<&T> {
    // 完全な注釈: fn get_first<'a, T>(data: &'a [T]) -> Option<&'a T>
    data.first()
}

fn trim_string(s: &str) -> &str {
    // 完全な注釈: fn trim_string<'a>(s: &'a str) -> &'a str
    s.trim()
}

fn method_lifetime_rule() {
    println!("\n--- 規則3: メソッドのライフタイム ---");

    let parser = Parser {
        content: "パーサーのコンテンツ",
    };

    let part = parser.get_part();
    println!("パーサーの一部: {}", part);

    let combined = parser.combine("追加データ");
    println!("結合結果: {}", combined);

    let context = Context::new("コンテキストデータ");
    let processed = context.process("入力");
    println!("処理結果: {}", processed);
}

struct Parser<'a> {
    content: &'a str,
}

impl<'a> Parser<'a> {
    // 規則3: &selfがあるので、selfのライフタイムが返り値に適用される
    fn get_part(&self) -> &str {
        // 完全な注釈: fn get_part(&'a self) -> &'a str
        // バイト位置で切るとマルチバイト文字の途中で panic するため、先頭5文字の境界で切る
        match self.content.char_indices().nth(5) {
            Some((end, _)) => &self.content[..end],
            None => self.content,
        }
    }

    // selfと他の参照の両方がある場合
    fn combine(&self, other: &str) -> String {
        // 完全な注釈: fn combine<'b>(&'a self, other: &'b str) -> String
        format!("{} + {}", self.content, other)
    }
}

struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    fn new(data: &'a str) -> Self {
        Context { data }
    }

    // 規則3が適用される
    fn process(&self, _input: &str) -> &str {
        // 完全な注釈: fn process<'b>(&'a self, input: &'b str) -> &'a str
        self.data
    }
}

fn when_elision_fails() {
    println!("\n--- ライフタイム省略が失敗する場合 ---");

    // 明示的な注釈が必要な例
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let result = longest_explicit(&s1, &s2);
    println!("より長い文字列（明示的）: {}", result);

    // 複雑な構造での例
    demonstrate_complex_lifetime();
}

// ライフタイム省略が適用できない例：複数の入力参照があり、どちらを返すか不明
fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 省略規則では解決できないため、明示的な注釈が必要
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demonstrate_complex_lifetime() {
    let string1 = String::from("複雑な");
    let string2 = String::from("ライフタイム");

    let wrapper = ComplexWrapper::new(&string1, &string2);
    let result = wrapper.get_longer();
    println!("複雑な構造での結果: {}", result);

    // 他のメソッドも使用
    wrapper.display_both();
    println!("Second: {}", wrapper.get_second());
}

// 複数のライフタイムが関わる構造体
struct ComplexWrapper<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> ComplexWrapper<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        ComplexWrapper { first, second }
    }

    // ライフタイム省略が適用できない例
    fn get_longer(&self) -> &'a str
    where
        'b: 'a, // 'bは'aより長生きする必要がある
    {
        if self.first.len() > self.second.len() {
            self.first
        } else {
            // self.secondを返すには制約が必要
            self.first // 簡単のため、常にfirstを返す
        }
    }

    fn get_second(&self) -> &'b str {
        self.second
    }

    fn display_both(&self) {
        println!("First: {}, Second: {}", self.first, self.second);
    }
}

// まとめ：省略規則の適用例
fn elision_summary() {
    // 規則1のみ：複数入力、所有型を返す
    fn process_two(_x: &str, _y: &str) -> String {
        String::new()
    }

    // 規則1+2：単一入力、参照を返す
    fn get_ref(s: &str) -> &str {
        s
    }

    // 規則1+3：メソッド
    struct S<'a> {
        data: &'a str,
    }

    impl<'a> S<'a> {
        fn method(&self) -> &str {
            self.data
        }
    }

    let s = S {
        data: "テストデータ",
    };
    println!("メソッド結果: {}", s.method());

    // 未使用関数を使用
    let processed = process_two("入力1", "入力2");
    println!("処理結果: {}", processed);

    let reference = get_ref("参照テスト");
    println!("参照結果: {}", reference);
}

#[cfg(test)]
mod tests {
    use super::Parser;

    // get_part の戻り値は省略規則3で &self に結び付くため、一時値の Parser からは String にして返す
    fn part(content: &str) -> String {
        Parser { content }.get_part().to_string()
    }

    #[test]
    fn get_part_takes_first_five_ascii_chars() {
        assert_eq!(part("abcdefgh"), "abcde");
    }

    #[test]
    fn get_part_cuts_on_char_boundary_for_multibyte_text() {
        assert_eq!(part("パーサーのコンテンツ"), "パーサーの");
    }

    #[test]
    fn get_part_returns_whole_text_at_or_below_five_chars() {
        assert_eq!(part("あいうえお"), "あいうえお");
        assert_eq!(part("abcd"), "abcd");
        assert_eq!(part(""), "");
    }
}
