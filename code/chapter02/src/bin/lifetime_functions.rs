// src/bin/lifetime_functions.rs

fn main() {
    println!("=== 関数とライフタイム ===\n");
    
    demonstrate_lifetime_parameters();
    demonstrate_lifetime_bounds();
    demonstrate_lifetime_subtyping();
    demonstrate_callback_lifetimes();
}

// 基本的なライフタイムパラメータ
fn demonstrate_lifetime_parameters() {
    println!("--- ライフタイムパラメータの基本 ---");
    
    // 最も長いライフタイムを返す関数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("最も長い文字列: {}", result);
    }
    
    // 異なるライフタイムを持つ参照
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("最初の単語: {}", word);
}

// ライフタイム境界
fn demonstrate_lifetime_bounds() {
    println!("\n--- ライフタイム境界 ---");
    
    // ジェネリックなライフタイム境界
    fn print_ref<'a, T>(t: &'a T) 
    where
        T: std::fmt::Display
    {
        println!("参照値: {}", t);
    }
    
    let number = 42;
    print_ref(&number);
    
    // 複数のライフタイム境界
    fn compare_and_display<'a, 'b, T>(x: &'a T, y: &'b T)
    where
        T: std::fmt::Display + PartialOrd,
        'b: 'a,  // 'bは'aよりも長生きする必要がある
    {
        if x < y {
            println!("{} < {}", x, y);
        } else {
            println!("{} >= {}", x, y);
        }
    }
    
    let n1 = 10;
    let n2 = 20;
    compare_and_display(&n1, &n2);
}

// ライフタイムサブタイピング
fn demonstrate_lifetime_subtyping() {
    println!("\n--- ライフタイムサブタイピング ---");
    
    // 共変性を示す例
    fn accept_str<'a>(s: &'a str) -> &'a str {
        s
    }
    
    // より長いライフタイムを持つ参照を渡せる
    let static_str: &'static str = "I live forever!";
    let result = accept_str(static_str);
    println!("静的文字列: {}", result);
    
    // ライフタイムの階層
    fn outer<'a>(x: &'a str) -> impl Fn() -> &'a str {
        move || x
    }
    
    let s = String::from("closure captured");
    let closure = outer(&s);
    println!("クロージャの結果: {}", closure());
}

// コールバックとライフタイム
fn demonstrate_callback_lifetimes() {
    println!("\n--- コールバックとライフタイム ---");
    
    // ライフタイムを持つコールバック
    struct Request<'a> {
        data: &'a str,
    }
    
    impl<'a> Request<'a> {
        fn process<F, R>(&self, callback: F) -> R
        where
            F: FnOnce(&'a str) -> R,
        {
            callback(self.data)
        }
    }
    
    let data = String::from("request data");
    let request = Request { data: &data };
    
    let result = request.process(|s| {
        format!("処理結果: {}", s.to_uppercase())
    });
    println!("{}", result);
    
    // 高階関数とライフタイム
    fn apply_to_3<'a, F>(f: F) -> &'a str
    where
        F: Fn(i32) -> &'a str,
    {
        f(3)
    }
    
    let closure = |num: i32| -> &'static str {
        match num {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "other",
        }
    };
    
    println!("数値3は: {}", apply_to_3(closure));
}
