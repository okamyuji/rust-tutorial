// src/bin/trait_objects.rs

use std::fmt::Debug;

fn main() {
    println!("=== トレイトオブジェクトとオブジェクト安全性 ===\n");

    object_safe_traits();
    trait_object_performance();
    object_safety_rules();
    practical_trait_objects();
}

// オブジェクト安全なトレイトの例
fn object_safe_traits() {
    println!("--- オブジェクト安全なトレイト ---");

    // オブジェクト安全なトレイト
    trait Animal {
        fn name(&self) -> &str;
        fn sound(&self) -> &str;

        // デフォルト実装があってもオブジェクト安全
        fn describe(&self) -> String {
            format!("{} says {}", self.name(), self.sound())
        }
    }

    struct Dog {
        name: String,
    }

    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }

        fn sound(&self) -> &str {
            "Woof!"
        }
    }

    struct Cat {
        name: String,
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }

        fn sound(&self) -> &str {
            "Meow!"
        }
    }

    // トレイトオブジェクトの使用
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("Rex"),
        }),
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        Box::new(Dog {
            name: String::from("Buddy"),
        }),
    ];

    for animal in &animals {
        println!("{}", animal.describe());
    }

    // 関数でトレイトオブジェクトを受け取る
    fn make_sound(animal: &dyn Animal) {
        println!("{} が鳴いた: {}", animal.name(), animal.sound());
    }

    make_sound(&*animals[0]);
    make_sound(&*animals[1]);
}

// トレイトオブジェクトのパフォーマンス特性
fn trait_object_performance() {
    println!("\n--- トレイトオブジェクトのパフォーマンス ---");

    trait Compute {
        fn compute(&self, x: i32) -> i32;
    }

    struct Adder(i32);
    impl Compute for Adder {
        fn compute(&self, x: i32) -> i32 {
            x + self.0
        }
    }

    struct Multiplier(i32);
    impl Compute for Multiplier {
        fn compute(&self, x: i32) -> i32 {
            x * self.0
        }
    }

    // 静的ディスパッチ（コンパイル時に解決）
    fn static_dispatch<T: Compute>(computer: &T, values: &[i32]) -> Vec<i32> {
        values.iter().map(|&x| computer.compute(x)).collect()
    }

    // 動的ディスパッチ（実行時に解決）
    fn dynamic_dispatch(computer: &dyn Compute, values: &[i32]) -> Vec<i32> {
        values.iter().map(|&x| computer.compute(x)).collect()
    }

    let values = vec![1, 2, 3, 4, 5];
    let adder = Adder(10);
    let multiplier = Multiplier(2);

    // 静的ディスパッチの使用
    println!(
        "静的ディスパッチ（Adder）: {:?}",
        static_dispatch(&adder, &values)
    );
    println!(
        "静的ディスパッチ（Multiplier）: {:?}",
        static_dispatch(&multiplier, &values)
    );

    // 動的ディスパッチの使用
    let computers: Vec<Box<dyn Compute>> = vec![Box::new(Adder(10)), Box::new(Multiplier(2))];

    for (i, computer) in computers.iter().enumerate() {
        println!(
            "動的ディスパッチ[{}]: {:?}",
            i,
            dynamic_dispatch(&**computer, &values)
        );
    }
}

// オブジェクト安全性のルール
fn object_safety_rules() {
    println!("\n--- オブジェクト安全性のルール ---");

    // オブジェクト安全でないトレイトの例
    trait NotObjectSafe {
        // Selfを返すメソッド
        fn clone_self(&self) -> Self;

        // ジェネリックメソッド
        fn generic_method<T>(&self, value: T);

        // Sized境界が必要なメソッド
        fn by_value(self)
        where
            Self: Sized;
    }

    // オブジェクト安全にする方法
    trait MadeObjectSafe {
        // Selfの代わりにBox<dyn Trait>を返す
        fn clone_boxed(&self) -> Box<dyn MadeObjectSafe>;

        // ジェネリックメソッドをトレイト境界で制限
        fn process(&self, value: &dyn Debug);

        // where Self: Sizedを追加してオプショナルにする
        fn by_value(self)
        where
            Self: Sized,
        {
            // デフォルト実装
        }
    }

    // 実装例
    #[derive(Clone)]
    struct MyType {
        value: i32,
    }

    impl MadeObjectSafe for MyType {
        fn clone_boxed(&self) -> Box<dyn MadeObjectSafe> {
            Box::new(self.clone())
        }

        fn process(&self, value: &dyn Debug) {
            println!("Processing {:?} with value {}", value, self.value);
        }
    }

    let obj = MyType { value: 42 };
    let trait_obj: Box<dyn MadeObjectSafe> = Box::new(obj);

    let cloned = trait_obj.clone_boxed();
    cloned.process(&"test");

    println!("オブジェクト安全性の確認完了");
}

// 実践的なトレイトオブジェクトの使用
fn practical_trait_objects() {
    println!("\n--- 実践的なトレイトオブジェクト ---");

    // プラグインシステムの例
    trait Plugin {
        fn name(&self) -> &str;
        fn execute(&self, input: &str) -> String;
        fn version(&self) -> &str {
            "1.0.0"
        }
    }

    struct UpperCasePlugin;
    impl Plugin for UpperCasePlugin {
        fn name(&self) -> &str {
            "UpperCase"
        }

        fn execute(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    struct ReversePlugin;
    impl Plugin for ReversePlugin {
        fn name(&self) -> &str {
            "Reverse"
        }

        fn execute(&self, input: &str) -> String {
            input.chars().rev().collect()
        }
    }

    struct RepeatPlugin {
        times: usize,
    }
    impl Plugin for RepeatPlugin {
        fn name(&self) -> &str {
            "Repeat"
        }

        fn execute(&self, input: &str) -> String {
            input.repeat(self.times)
        }

        fn version(&self) -> &str {
            "2.0.0"
        }
    }

    // プラグインマネージャー
    struct PluginManager {
        plugins: Vec<Box<dyn Plugin>>,
    }

    impl PluginManager {
        fn new() -> Self {
            PluginManager {
                plugins: Vec::new(),
            }
        }

        fn register(&mut self, plugin: Box<dyn Plugin>) {
            println!(
                "プラグイン '{}' v{} を登録",
                plugin.name(),
                plugin.version()
            );
            self.plugins.push(plugin);
        }

        fn execute_all(&self, input: &str) -> Vec<(String, String)> {
            self.plugins
                .iter()
                .map(|plugin| (plugin.name().to_string(), plugin.execute(input)))
                .collect()
        }
    }

    let mut manager = PluginManager::new();
    manager.register(Box::new(UpperCasePlugin));
    manager.register(Box::new(ReversePlugin));
    manager.register(Box::new(RepeatPlugin { times: 3 }));

    let input = "hello";
    println!("\n入力: '{}'", input);
    println!("プラグイン実行結果:");

    for (name, result) in manager.execute_all(input) {
        println!("  {}: '{}'", name, result);
    }

    // エラーハンドリング付きのトレイトオブジェクト
    trait Processor: Debug {
        fn process(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    }

    #[derive(Debug)]
    struct CompressProcessor;
    impl Processor for CompressProcessor {
        fn process(&self, data: &[u8]) -> Result<Vec<u8>, String> {
            // 簡略化された圧縮シミュレーション
            if data.is_empty() {
                Err("空のデータは圧縮できません".to_string())
            } else {
                Ok(vec![data.len() as u8]) // 長さだけを返す
            }
        }
    }

    #[derive(Debug)]
    struct EncryptProcessor;
    impl Processor for EncryptProcessor {
        fn process(&self, data: &[u8]) -> Result<Vec<u8>, String> {
            // 簡略化された暗号化シミュレーション
            Ok(data.iter().map(|&b| b ^ 0x42).collect())
        }
    }

    let processors: Vec<Box<dyn Processor>> =
        vec![Box::new(CompressProcessor), Box::new(EncryptProcessor)];

    let data = b"secret data";
    println!("\n処理パイプライン:");

    let mut current_data = data.to_vec();
    for processor in &processors {
        println!("  {:?} を実行中...", processor);
        match processor.process(&current_data) {
            Ok(result) => {
                println!(
                    "    成功: {} バイト -> {} バイト",
                    current_data.len(),
                    result.len()
                );
                current_data = result;
            }
            Err(e) => {
                println!("    エラー: {}", e);
                break;
            }
        }
    }
}
