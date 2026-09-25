// src/bin/associated_types.rs

use std::ops::Add;

fn main() {
    println!("=== 関連型（Associated Types）vs ジェネリックパラメータ ===\n");
    
    associated_types_basics();
    generic_vs_associated();
    complex_associated_types();
    practical_examples();
}

// 関連型の基本
fn associated_types_basics() {
    println!("--- 関連型の基本 ---");
    
    // Iteratorトレイトの簡略版
    trait MyIterator {
        type Item;  // 関連型
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    // カウンターイテレータ
    struct Counter {
        count: u32,
        max: u32,
    }
    
    impl MyIterator for Counter {
        type Item = u32;  // 関連型を具体的に指定
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    let mut counter = Counter { count: 0, max: 5 };
    println!("カウンター:");
    while let Some(n) = counter.next() {
        print!("{} ", n);
    }
    println!();
    
    // 文字列イテレータ
    struct Words {
        text: Vec<String>,
        index: usize,
    }
    
    impl MyIterator for Words {
        type Item = String;  // 異なる関連型
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.text.len() {
                let word = self.text[self.index].clone();
                self.index += 1;
                Some(word)
            } else {
                None
            }
        }
    }
    
    let mut words = Words {
        text: vec!["Hello".to_string(), "Rust".to_string(), "World".to_string()],
        index: 0,
    };
    
    println!("単語:");
    while let Some(word) = words.next() {
        print!("{} ", word);
    }
    println!();
}

// ジェネリックパラメータと関連型の比較
fn generic_vs_associated() {
    println!("\n--- ジェネリックパラメータ vs 関連型 ---");
    
    // ジェネリックパラメータを使用（複数の実装が可能）
    trait Container<T> {
        fn get(&self) -> &T;
        fn set(&mut self, value: T);
    }
    
    struct MyBox<T> {
        value: T,
    }
    
    // 同じ型に対して複数の実装が可能
    impl Container<i32> for MyBox<i32> {
        fn get(&self) -> &i32 {
            &self.value
        }
        
        fn set(&mut self, value: i32) {
            self.value = value;
        }
    }
    
    impl Container<String> for MyBox<String> {
        fn get(&self) -> &String {
            &self.value
        }
        
        fn set(&mut self, value: String) {
            self.value = value;
        }
    }
    
    // 関連型を使用（型ごとに一つの実装）
    trait Storage {
        type Item;  // 関連型
        
        fn store(&mut self, item: Self::Item);
        fn retrieve(&self) -> Option<&Self::Item>;
    }
    
    struct Stack<T> {
        items: Vec<T>,
    }
    
    impl<T> Storage for Stack<T> {
        type Item = T;  // Tに対して一つの実装のみ
        
        fn store(&mut self, item: Self::Item) {
            self.items.push(item);
        }
        
        fn retrieve(&self) -> Option<&Self::Item> {
            self.items.last()
        }
    }
    
    let int_box = MyBox { value: 42 };
    println!("ジェネリック Container<i32>: {}", int_box.get());
    
    let mut stack = Stack { items: Vec::new() };
    stack.store(100);
    stack.store(200);
    println!("関連型 Storage: {:?}", stack.retrieve());
}

// 複雑な関連型の使用
fn complex_associated_types() {
    println!("\n--- 複雑な関連型パターン ---");
    
    // 複数の関連型を持つトレイト
    trait Converter {
        type Input;
        type Output;
        type Error;
        
        fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    }
    
    // 文字列から数値への変換器
    struct StringToNumber;
    
    impl Converter for StringToNumber {
        type Input = String;
        type Output = i32;
        type Error = std::num::ParseIntError;
        
        fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            input.parse()
        }
    }
    
    // バイト配列から文字列への変換器
    struct BytesToString;
    
    impl Converter for BytesToString {
        type Input = Vec<u8>;
        type Output = String;
        type Error = std::string::FromUtf8Error;
        
        fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            String::from_utf8(input)
        }
    }
    
    let str_to_num = StringToNumber;
    match str_to_num.convert("42".to_string()) {
        Ok(n) => println!("文字列→数値変換: {}", n),
        Err(e) => println!("変換エラー: {}", e),
    }
    
    let bytes_to_str = BytesToString;
    match bytes_to_str.convert(vec![72, 101, 108, 108, 111]) {
        Ok(s) => println!("バイト→文字列変換: {}", s),
        Err(e) => println!("変換エラー: {}", e),
    }
    
    // 関連型の制約
    trait Graph {
        type Node;
        type Edge;
        
        fn nodes(&self) -> Vec<&Self::Node>;
        fn edges(&self) -> Vec<&Self::Edge>;
        fn neighbors(&self, node: &Self::Node) -> Vec<&Self::Node>;
    }
    
    // where句で関連型に制約を追加
    fn print_graph<G>(graph: &G) 
    where
        G: Graph,
        G::Node: std::fmt::Debug,
        G::Edge: std::fmt::Debug,
    {
        println!("グラフのノード: {:?}", graph.nodes());
        println!("グラフのエッジ: {:?}", graph.edges());
    }
}

// 実践的な例
fn practical_examples() {
    println!("\n--- 実践的な関連型の使用例 ---");
    
    // 非同期処理の例（簡略化）
    trait AsyncTask {
        type Output;
        type Error;
        
        fn execute(&self) -> Result<Self::Output, Self::Error>;
    }
    
    struct HttpRequest {
        url: String,
    }
    
    #[derive(Debug)]
    struct HttpResponse {
        status: u16,
        body: String,
    }
    
    #[derive(Debug)]
    struct HttpError {
        message: String,
    }
    
    impl AsyncTask for HttpRequest {
        type Output = HttpResponse;
        type Error = HttpError;
        
        fn execute(&self) -> Result<Self::Output, Self::Error> {
            // 実際のHTTPリクエストのシミュレーション
            if self.url.starts_with("https://") {
                Ok(HttpResponse {
                    status: 200,
                    body: format!("Response from {}", self.url),
                })
            } else {
                Err(HttpError {
                    message: "URLはhttps://で始まる必要があります".to_string(),
                })
            }
        }
    }
    
    let request = HttpRequest {
        url: "https://example.com".to_string(),
    };
    
    match request.execute() {
        Ok(response) => println!("HTTPレスポンス: {:?}", response),
        Err(error) => println!("HTTPエラー: {:?}", error),
    }
    
    // 演算子オーバーロードでの関連型
    #[derive(Debug, Clone)]
    struct Vector2D {
        x: f64,
        y: f64,
    }
    
    impl Add for Vector2D {
        type Output = Vector2D;  // 関連型で出力型を指定
        
        fn add(self, other: Vector2D) -> Self::Output {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    let v3 = v1.clone() + v2.clone();
    println!("\nベクトル加算: {:?} + {:?} = {:?}", v1, v2, v3);
    
    // ビルダーパターンでの関連型
    trait Builder {
        type Output;
        
        fn build(self) -> Self::Output;
    }
    
    struct CarBuilder {
        make: Option<String>,
        model: Option<String>,
        year: Option<u16>,
    }
    
    struct Car {
        make: String,
        model: String,
        year: u16,
    }
    
    impl Builder for CarBuilder {
        type Output = Result<Car, String>;
        
        fn build(self) -> Self::Output {
            match (self.make, self.model, self.year) {
                (Some(make), Some(model), Some(year)) => {
                    Ok(Car { make, model, year })
                }
                _ => Err("必須フィールドが不足しています".to_string())
            }
        }
    }
    
    impl CarBuilder {
        fn new() -> Self {
            CarBuilder {
                make: None,
                model: None,
                year: None,
            }
        }
        
        fn make(mut self, make: &str) -> Self {
            self.make = Some(make.to_string());
            self
        }
        
        fn model(mut self, model: &str) -> Self {
            self.model = Some(model.to_string());
            self
        }
        
        fn year(mut self, year: u16) -> Self {
            self.year = Some(year);
            self
        }
    }
    
    let car_result = CarBuilder::new()
        .make("Toyota")
        .model("Corolla")
        .year(2023)
        .build();
    
    match car_result {
        Ok(car) => println!("\n車の構築成功: {} {} ({}年)", 
                           car.make, car.model, car.year),
        Err(e) => println!("車の構築失敗: {}", e),
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
