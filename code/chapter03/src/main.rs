// src/main.rs
// 第3章：トレイトシステムの高度な活用 - メインプログラム

fn main() {
    println!("=== 第3章：トレイトシステムの高度な活用 ===\n");

    basic_trait_demo();
    println!();
    generic_trait_demo();
    println!();
    trait_object_demo();
}

// 基本的なトレイトの定義と実装
fn basic_trait_demo() {
    println!("--- 基本的なトレイト ---");

    // トレイトの定義
    trait Greet {
        fn greet(&self) -> String;

        // デフォルト実装を持つメソッド
        fn greet_loudly(&self) -> String {
            format!("{}!!!", self.greet().to_uppercase())
        }
    }

    // 構造体への実装
    struct Person {
        name: String,
    }

    impl Greet for Person {
        fn greet(&self) -> String {
            format!("Hello, I'm {}", self.name)
        }
    }

    struct Robot {
        id: u32,
    }

    impl Greet for Robot {
        fn greet(&self) -> String {
            format!("Beep boop, Robot {} reporting", self.id)
        }

        // デフォルト実装をオーバーライド
        fn greet_loudly(&self) -> String {
            format!("!!! ROBOT {} ACTIVATED !!!", self.id)
        }
    }

    let person = Person {
        name: String::from("Alice"),
    };
    let robot = Robot { id: 42 };

    println!("Person: {}", person.greet());
    println!("Person (loud): {}", person.greet_loudly());
    println!("Robot: {}", robot.greet());
    println!("Robot (loud): {}", robot.greet_loudly());
}

// ジェネリックトレイトとトレイト境界
fn generic_trait_demo() {
    println!("--- ジェネリックトレイトとトレイト境界 ---");

    use std::fmt::Display;

    // ジェネリック関数でのトレイト境界
    fn print_twice<T: Display>(value: T) {
        println!("1回目: {}", value);
        println!("2回目: {}", value);
    }

    // 複数のトレイト境界
    fn debug_and_display<T: std::fmt::Debug + Display>(value: T) {
        println!("Debug: {:?}", value);
        println!("Display: {}", value);
    }

    // where句を使った読みやすい記法
    fn complex_bounds<T, U>(t: T, u: U)
    where
        T: Display + Clone,
        U: std::fmt::Debug + Default,
    {
        let t_clone = t.clone();
        println!("T: {}, T clone: {}", t, t_clone);
        println!("U: {:?}, U default: {:?}", u, U::default());
    }

    print_twice("Hello, Traits!");
    print_twice(123);

    debug_and_display("Rust");

    complex_bounds("test", vec![1, 2, 3]);
}

// トレイトオブジェクトと動的ディスパッチ
fn trait_object_demo() {
    println!("--- トレイトオブジェクトと動的ディスパッチ ---");

    trait Draw {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }

    impl Draw for Circle {
        fn draw(&self) {
            println!("○ 円を描画（半径: {}）", self.radius);
        }
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Draw for Rectangle {
        fn draw(&self) {
            println!("□ 長方形を描画（{}x{}）", self.width, self.height);
        }
    }

    // トレイトオブジェクトを使った動的ディスパッチ
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
        Box::new(Circle { radius: 3.0 }),
    ];

    println!("すべての図形を描画:");
    for shape in shapes.iter() {
        shape.draw();
    }

    // 関数でトレイトオブジェクトを受け取る
    fn draw_anything(drawable: &dyn Draw) {
        print!("何かを描画: ");
        drawable.draw();
    }

    let circle = Circle { radius: 7.0 };
    draw_anything(&circle);
}
