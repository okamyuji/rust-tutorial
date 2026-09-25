// src/bin/problem1_object_safety.rs
// 復習問題1: オブジェクト安全性の解答

use std::any::Any;

fn main() {
    println!("=== 復習問題1: オブジェクト安全性 ===\n");

    // 問題のあるトレイトの分析
    analyze_problematic_trait();

    // 修正されたトレイト設計
    fixed_trait_design();

    // 実用的な代替設計パターン
    practical_design_patterns();

    // トレイトオブジェクトの活用例
    trait_object_examples();
}

// 問題のあるトレイトの分析
fn analyze_problematic_trait() {
    println!("【問題のあるトレイト】");

    /*
    // これはオブジェクト安全でない
    trait Container {
        fn new() -> Self;                    // 問題1: Selfを返す静的メソッド
        fn add<T>(&mut self, item: T);       // 問題2: ジェネリックメソッド
        fn count(&self) -> usize;            // OK: オブジェクト安全
    }
    */

    println!("問題点の詳細:");
    println!("1. fn new() -> Self");
    println!("   - Selfを返すメソッド（静的メソッド）");
    println!("   - トレイトオブジェクトでは実行時に型が不明");
    println!("   - どのサイズのメモリを確保すべきかわからない");

    println!("\n2. fn add<T>(&mut self, item: T)");
    println!("   - ジェネリックメソッド");
    println!("   - 無限の型に対して実装が必要");
    println!("   - 動的ディスパッチで解決不可能");

    println!("\n3. fn count(&self) -> usize");
    println!("   - ✓ オブジェクト安全");
    println!("   - selfを受け取り、具体的な戻り値型");

    // オブジェクト安全性の条件
    println!("\n【オブジェクト安全性の条件】");
    println!("✓ Selfを返すメソッドを持たない");
    println!("✓ ジェネリックメソッドを持たない");
    println!("✓ Selfに対するトレイト境界を持たない");
    println!("✓ 関連定数を持たない（一部例外あり）");
}

// 修正されたトレイト設計
fn fixed_trait_design() {
    println!("\n【修正されたトレイト設計】");

    // オブジェクト安全なContainer
    trait Container {
        fn count(&self) -> usize;
        fn add_item(&mut self, item: Box<dyn Any>);
        fn get_item(&self, index: usize) -> Option<&dyn Any>;
        fn is_empty(&self) -> bool {
            self.count() == 0
        }
        fn clear(&mut self);
    }

    // ファクトリトレイトで生成を分離
    trait ContainerFactory {
        type Container: Container;
        fn create() -> Self::Container;
        fn create_with_capacity(capacity: usize) -> Self::Container;
    }

    // 具体的な実装例
    struct VecContainer {
        items: Vec<Box<dyn Any>>,
        capacity: Option<usize>,
    }

    impl Container for VecContainer {
        fn count(&self) -> usize {
            self.items.len()
        }

        fn add_item(&mut self, item: Box<dyn Any>) {
            if let Some(cap) = self.capacity {
                if self.items.len() >= cap {
                    println!("容量制限に達しました ({})", cap);
                    return;
                }
            }
            self.items.push(item);
        }

        fn get_item(&self, index: usize) -> Option<&dyn Any> {
            self.items.get(index).map(|item| item.as_ref())
        }

        fn clear(&mut self) {
            self.items.clear();
        }
    }

    struct VecContainerFactory;

    impl ContainerFactory for VecContainerFactory {
        type Container = VecContainer;

        fn create() -> Self::Container {
            VecContainer {
                items: Vec::new(),
                capacity: None,
            }
        }

        fn create_with_capacity(capacity: usize) -> Self::Container {
            VecContainer {
                items: Vec::with_capacity(capacity),
                capacity: Some(capacity),
            }
        }
    }

    // 使用例
    println!("修正版の使用例:");

    let mut container = VecContainerFactory::create();
    container.add_item(Box::new(42i32));
    container.add_item(Box::new("hello".to_string()));
    container.add_item(Box::new(vec![1, 2, 3]));

    println!("Items: {}", container.count());

    // 型の復元例
    if let Some(item) = container.get_item(0) {
        if let Some(number) = item.downcast_ref::<i32>() {
            println!("最初のアイテム（数値）: {}", number);
        }
    }

    if let Some(item) = container.get_item(1) {
        if let Some(text) = item.downcast_ref::<String>() {
            println!("2番目のアイテム（文字列）: {}", text);
        }
    }

    // トレイトオブジェクトとして使用可能
    let containers: Vec<Box<dyn Container>> = vec![
        Box::new(container),
        Box::new(VecContainerFactory::create_with_capacity(5)),
    ];

    println!(
        "トレイトオブジェクトとしてのコンテナ数: {}",
        containers.len()
    );
    for (i, container) in containers.iter().enumerate() {
        println!("  コンテナ{}: {} items", i, container.count());
    }
}

// 実用的な代替設計パターン
fn practical_design_patterns() {
    println!("\n【実用的な代替設計パターン】");

    // パターン1: 型指定されたContainer
    trait TypedContainer<T> {
        fn add(&mut self, item: T);
        fn get(&self, index: usize) -> Option<&T>;
        fn count(&self) -> usize;
    }

    // パターン2: 拡張トレイト
    trait ContainerExt<T> {
        fn add_multiple(&mut self, items: Vec<T>);
        fn find(&self, predicate: Box<dyn Fn(&T) -> bool>) -> Option<&T>;
    }

    // 具体的な実装
    struct SimpleContainer<T> {
        items: Vec<T>,
    }

    impl<T> TypedContainer<T> for SimpleContainer<T> {
        fn add(&mut self, item: T) {
            self.items.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }

        fn count(&self) -> usize {
            self.items.len()
        }
    }

    impl<T> ContainerExt<T> for SimpleContainer<T>
    where
        T: PartialEq,
    {
        fn add_multiple(&mut self, items: Vec<T>) {
            self.items.extend(items);
        }

        fn find(&self, predicate: Box<dyn Fn(&T) -> bool>) -> Option<&T> {
            self.items.iter().find(|item| predicate(item))
        }
    }

    // パターン3: Builder パターンでnew()を回避
    struct ContainerBuilder<T> {
        capacity: Option<usize>,
        initial_items: Vec<T>,
    }

    impl<T> ContainerBuilder<T> {
        fn new() -> Self {
            ContainerBuilder {
                capacity: None,
                initial_items: Vec::new(),
            }
        }

        fn with_capacity(mut self, capacity: usize) -> Self {
            self.capacity = Some(capacity);
            self
        }

        fn with_items(mut self, items: Vec<T>) -> Self {
            self.initial_items = items;
            self
        }

        fn build(self) -> SimpleContainer<T> {
            let mut items = if let Some(cap) = self.capacity {
                Vec::with_capacity(cap)
            } else {
                Vec::new()
            };
            items.extend(self.initial_items);

            SimpleContainer { items }
        }
    }

    // 使用例
    println!("型安全なコンテナの例:");

    let mut string_container = ContainerBuilder::new()
        .with_capacity(10)
        .with_items(vec!["hello".to_string(), "world".to_string()])
        .build();

    string_container.add("rust".to_string());
    println!("文字列コンテナのアイテム数: {}", string_container.count());

    if let Some(item) = string_container.get(0) {
        println!("最初のアイテム: {}", item);
    }

    // 検索機能
    let finder = Box::new(|s: &String| s.contains("rust"));
    if let Some(found) = string_container.find(finder) {
        println!("見つかったアイテム: {}", found);
    }

    // 数値コンテナ
    let mut number_container = SimpleContainer { items: Vec::new() };
    number_container.add(1);
    number_container.add(2);
    number_container.add(3);
    number_container.add_multiple(vec![4, 5, 6]);

    println!("数値コンテナのアイテム数: {}", number_container.count());
}

// トレイトオブジェクトの活用例
fn trait_object_examples() {
    println!("\n【トレイトオブジェクトの活用例】");

    // オブジェクト安全なトレイト設計
    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }

    trait Resizable {
        fn resize(&mut self, factor: f64);
    }

    // 具体的な図形
    struct Circle {
        radius: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("円を描画 (半径: {})", self.radius);
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn name(&self) -> &str {
            "Circle"
        }
    }

    impl Resizable for Circle {
        fn resize(&mut self, factor: f64) {
            self.radius *= factor;
        }
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("長方形を描画 ({}x{})", self.width, self.height);
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn name(&self) -> &str {
            "Rectangle"
        }
    }

    impl Resizable for Rectangle {
        fn resize(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }
    }

    // DrawableとResizableを組み合わせたトレイト
    trait DrawableResizable: Drawable + Resizable {}

    // 自動実装
    impl<T: Drawable + Resizable> DrawableResizable for T {}

    // Canvas（描画領域）
    struct Canvas {
        shapes: Vec<Box<dyn Drawable>>,
        resizable_shapes: Vec<Box<dyn DrawableResizable>>,
    }

    impl Canvas {
        fn new() -> Self {
            Canvas {
                shapes: Vec::new(),
                resizable_shapes: Vec::new(),
            }
        }

        fn add_shape(&mut self, shape: Box<dyn Drawable>) {
            self.shapes.push(shape);
        }

        fn add_resizable_shape(&mut self, shape: Box<dyn DrawableResizable>) {
            self.resizable_shapes.push(shape);
        }

        fn draw_all(&self) {
            println!("すべての図形を描画:");
            for shape in &self.shapes {
                shape.draw();
            }
            for shape in &self.resizable_shapes {
                shape.draw();
            }
        }

        fn total_area(&self) -> f64 {
            let fixed_area: f64 = self.shapes.iter().map(|s| s.area()).sum();
            let resizable_area: f64 = self.resizable_shapes.iter().map(|s| s.area()).sum();
            fixed_area + resizable_area
        }

        fn resize_all(&mut self, factor: f64) {
            for shape in &mut self.resizable_shapes {
                shape.resize(factor);
            }
        }

        fn get_shape_info(&self) -> Vec<(String, f64)> {
            let mut info = Vec::new();

            for shape in &self.shapes {
                info.push((shape.name().to_string(), shape.area()));
            }

            for shape in &self.resizable_shapes {
                info.push((shape.name().to_string(), shape.area()));
            }

            info
        }
    }

    // 使用例
    let mut canvas = Canvas::new();

    // 固定サイズの図形
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));

    // リサイズ可能な図形
    canvas.add_resizable_shape(Box::new(Circle { radius: 3.0 }));
    canvas.add_resizable_shape(Box::new(Rectangle {
        width: 4.0,
        height: 6.0,
    }));

    println!("初期状態:");
    canvas.draw_all();
    println!("総面積: {:.2}", canvas.total_area());

    // リサイズ
    println!("\n1.5倍にリサイズ後:");
    canvas.resize_all(1.5);
    canvas.draw_all();
    println!("総面積: {:.2}", canvas.total_area());

    // 図形情報の表示
    println!("\n図形情報:");
    let info = canvas.get_shape_info();
    for (name, area) in info {
        println!("  {}: 面積 {:.2}", name, area);
    }

    println!("\n【設計のポイント】");
    println!("✓ ファクトリパターンで生成を分離");
    println!("✓ Box<dyn Any>で型消去を活用");
    println!("✓ トレイト境界の組み合わせで柔軟性を確保");
    println!("✓ ビルダーパターンで設定可能な生成");
    println!("✓ 複数のトレイトオブジェクトで機能分離");
}
