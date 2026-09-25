// src/bin/trait_bounds.rs

use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

fn main() {
    println!("=== トレイト境界の高度なパターン ===\n");
    
    basic_trait_bounds();
    where_clause_patterns();
    multiple_trait_bounds();
    higher_ranked_bounds();
    conditional_implementations();
}

// 基本的なトレイト境界
fn basic_trait_bounds() {
    println!("--- 基本的なトレイト境界 ---");
    
    // シンプルなトレイト境界
    fn print_item<T: Display>(item: T) {
        println!("アイテム: {}", item);
    }
    
    // 複数の境界
    fn print_debug_and_display<T: Debug + Display>(item: T) {
        println!("Debug: {:?}", item);
        println!("Display: {}", item);
    }
    
    print_item("Hello, Rust!");
    print_item(42);
    
    print_debug_and_display("テスト文字列");
    
    // ライフタイムとトレイト境界
    fn longest_with_display<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: Display,
    {
        println!("比較: {} vs {}", x, y);
        if x.to_string().len() > y.to_string().len() {
            x
        } else {
            y
        }
    }
    
    let result = longest_with_display(&"short", &"much longer string");
    println!("より長い方: {}", result);
}

// where句の効果的な使用
fn where_clause_patterns() {
    println!("\n--- where句のパターン ---");
    
    // 複雑な境界を読みやすくする
    fn complex_function<T, U, V>(t: T, u: U) -> V
    where
        T: Clone + Debug,
        U: Clone + Debug,
        V: From<T> + From<U> + Default,
    {
        println!("T: {:?}", t);
        println!("U: {:?}", u);
        
        // 実装例
        if std::mem::size_of::<T>() > std::mem::size_of::<U>() {
            V::from(t)
        } else {
            V::from(u)
        }
    }
    
    // 関連型に対する境界
    trait Container {
        type Item;
        
        fn items(&self) -> &[Self::Item];
    }
    
    fn print_items<C>(container: &C)
    where
        C: Container,
        C::Item: Display,
    {
        for item in container.items() {
            println!("  - {}", item);
        }
    }
    
    struct StringContainer {
        data: Vec<String>,
    }
    
    impl Container for StringContainer {
        type Item = String;
        
        fn items(&self) -> &[Self::Item] {
            &self.data
        }
    }
    
    let container = StringContainer {
        data: vec!["First".to_string(), "Second".to_string()],
    };
    
    println!("コンテナの内容:");
    print_items(&container);
    
    // 条件付きメソッド実装
    struct Pair<T> {
        first: T,
        second: T,
    }
    
    impl<T> Pair<T> {
        fn new(first: T, second: T) -> Self {
            Pair { first, second }
        }
    }
    
    impl<T: Display + PartialOrd> Pair<T> {
        fn display_larger(&self) {
            if self.first > self.second {
                println!("より大きい値: {}", self.first);
            } else {
                println!("より大きい値: {}", self.second);
            }
        }
    }
    
    let pair = Pair::new(10, 20);
    pair.display_larger();
}

// 複数のトレイト境界の組み合わせ
fn multiple_trait_bounds() {
    println!("\n--- 複数のトレイト境界 ---");
    
    // 算術演算可能な型
    fn calculate<T>(a: T, b: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        a * b + a + b  // (a * b) + a + b
    }
    
    println!("整数計算: 3, 4 → {}", calculate(3, 4));
    println!("浮動小数点計算: 2.5, 3.5 → {}", calculate(2.5f64, 3.5f64));
    
    // トレイトエイリアス（実験的機能の代替）
    trait Numeric: Add<Output = Self> + Mul<Output = Self> + Copy + Display {}
    
    // ブランケット実装
    impl<T> Numeric for T 
    where 
        T: Add<Output = T> + Mul<Output = T> + Copy + Display 
    {}
    
    fn calculate_numeric<T: Numeric>(a: T, b: T) -> T {
        let result = a * b + a;
        println!("計算: {} * {} + {} = {}", a, b, a, result);
        result
    }
    
    calculate_numeric(5, 10);
    
    // 複合トレイト
    trait Drawable: Debug {
        fn draw(&self);
    }
    
    trait Clickable: Debug {
        fn on_click(&self);
    }
    
    trait Interactive: Drawable + Clickable {
        fn interact(&self) {
            self.draw();
            println!("インタラクティブ要素: {:?}", self);
        }
    }
    
    #[derive(Debug)]
    struct Button {
        label: String,
    }
    
    impl Drawable for Button {
        fn draw(&self) {
            println!("[{}]", self.label);
        }
    }
    
    impl Clickable for Button {
        fn on_click(&self) {
            println!("ボタン '{}' がクリックされました", self.label);
        }
    }
    
    impl Interactive for Button {}
    
    let button = Button { label: "Submit".to_string() };
    button.interact();
    button.on_click();
}

// 高階トレイト境界（HRTB）
fn higher_ranked_bounds() {
    println!("\n--- 高階トレイト境界（HRTB） ---");
    
    // for<'a> 構文
    fn apply_to_str<F>(f: F) -> String
    where
        F: for<'a> Fn(&'a str) -> String,
    {
        let temp = "temporary string";
        f(temp)
    }
    
    let result = apply_to_str(|s| format!("処理済み: {}", s.to_uppercase()));
    println!("{}", result);
    
    // クロージャとライフタイム
    fn take_closure<F>(f: F)
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s1 = String::from("short");
        let s2 = String::from("much longer string");
        
        let result1 = f(&s1);
        let result2 = f(&s2);
        
        println!("結果1: {}", result1);
        println!("結果2: {}", result2);
    }
    
    take_closure(|s| {
        if s.len() > 10 {
            "長い"
        } else {
            "短い"
        }
    });
    
    // より複雑なHRTBの例
    trait Closure<Args> {
        type Output;
        fn call(&self, args: Args) -> Self::Output;
    }
    
    fn higher_order<'a, F, T>(f: F, value: &'a T) -> String
    where
        F: for<'b> Closure<&'b T, Output = String>,
        T: Debug,
    {
        println!("高階関数に渡された値: {:?}", value);
        f.call(value)
    }
}

// 条件付き実装
fn conditional_implementations() {
    println!("\n--- 条件付きトレイト実装 ---");
    
    // ジェネリック型に対する条件付き実装
    struct Wrapper<T> {
        value: T,
    }
    
    // すべてのTに対する基本実装
    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }
    }
    
    // Displayを実装する型に対してのみ利用可能
    impl<T: Display> Wrapper<T> {
        fn show(&self) {
            println!("Wrapped value: {}", self.value);
        }
    }
    
    // Debugを実装する型に対してのみ利用可能
    impl<T: Debug> Wrapper<T> {
        fn debug(&self) {
            println!("Debug: {:?}", self.value);
        }
    }
    
    // CloneとDefaultを実装する型に対してのみ利用可能
    impl<T: Clone + Default> Wrapper<T> {
        fn reset(&mut self) {
            self.value = T::default();
        }
        
        fn duplicate(&self) -> Self {
            Wrapper {
                value: self.value.clone(),
            }
        }
    }
    
    let wrapper_str = Wrapper::new("Hello");
    wrapper_str.show();  // Displayを実装
    wrapper_str.debug(); // Debugを実装
    
    let mut wrapper_vec = Wrapper::new(vec![1, 2, 3]);
    wrapper_vec.debug();      // Debugを実装
    wrapper_vec.reset();      // Clone + Defaultを実装
    wrapper_vec.debug();
    
    // 否定的な境界（実験的機能の代替案）
    trait NotEmpty {
        fn is_not_empty(&self) -> bool;
    }
    
    // 特定の型以外に実装
    impl<T> NotEmpty for Vec<T> {
        fn is_not_empty(&self) -> bool {
            !self.is_empty()
        }
    }
    
    impl NotEmpty for String {
        fn is_not_empty(&self) -> bool {
            !self.is_empty()
        }
    }
    
    fn require_not_empty<T: NotEmpty>(value: &T) {
        if value.is_not_empty() {
            println!("値は空ではありません");
        } else {
            println!("値は空です");
        }
    }
    
    require_not_empty(&vec![1, 2, 3]);
    require_not_empty(&String::from("test"));
    
    // マーカートレイトの使用
    trait SendAndSync: Send + Sync {}
    impl<T: Send + Sync> SendAndSync for T {}
    
    fn thread_safe<T: SendAndSync + Debug>(value: T) {
        println!("スレッドセーフな値: {:?}", value);
    }
    
    thread_safe(42);
    thread_safe("static string");
    thread_safe(vec![1, 2, 3]);
}
