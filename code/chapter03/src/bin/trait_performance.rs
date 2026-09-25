// src/bin/trait_performance.rs

use std::fmt::Display;
use std::time::Instant;

fn main() {
    println!("=== パフォーマンスとトレイト ===\n");

    static_vs_dynamic_dispatch();
    monomorphization_demo();
    trait_object_overhead();
    optimization_techniques();
    benchmark_example();
}

// 静的ディスパッチ vs 動的ディスパッチ
fn static_vs_dynamic_dispatch() {
    println!("--- 静的 vs 動的ディスパッチ ---");

    // 静的ディスパッチ（コンパイル時に解決）
    fn process_static<T: Display>(items: &[T]) {
        for item in items {
            println!("  Static: {}", item);
        }
    }

    // 動的ディスパッチ（実行時に解決）
    fn process_dynamic(items: &[&dyn Display]) {
        for item in items {
            println!("  Dynamic: {}", item);
        }
    }

    // テストデータ
    let numbers = vec![1, 2, 3];
    let strings = vec!["a", "b", "c"];

    println!("静的ディスパッチ（数値）:");
    process_static(&numbers);

    println!("\n静的ディスパッチ（文字列）:");
    process_static(&strings);

    // 動的ディスパッチは異なる型を混在可能
    let mixed: Vec<&dyn Display> = vec![&42, &"hello", &std::f64::consts::PI];
    println!("\n動的ディスパッチ（混在）:");
    process_dynamic(&mixed);

    // パフォーマンス比較
    const ITERATIONS: usize = 1_000_000;

    // 静的ディスパッチのベンチマーク
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = format_static(42);
    }
    let static_time = start.elapsed();

    // 動的ディスパッチのベンチマーク
    let value: &dyn Display = &42;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = format_dynamic(value);
    }
    let dynamic_time = start.elapsed();

    println!("\nパフォーマンス比較（{}回の実行）:", ITERATIONS);
    println!("  静的ディスパッチ: {:?}", static_time);
    println!("  動的ディスパッチ: {:?}", dynamic_time);
    println!(
        "  比率: {:.2}x",
        dynamic_time.as_nanos() as f64 / static_time.as_nanos() as f64
    );
}

fn format_static<T: Display>(value: T) -> String {
    format!("{}", value)
}

fn format_dynamic(value: &dyn Display) -> String {
    format!("{}", value)
}

// モノモーフィゼーション
fn monomorphization_demo() {
    println!("\n--- モノモーフィゼーション ---");

    // ジェネリック関数
    fn generic_function<T: Display + Clone>(value: T) -> String {
        let cloned = value.clone();
        format!("{} -> {}", value, cloned)
    }

    // コンパイラは各型に対して専用の関数を生成
    println!("i32版: {}", generic_function(42));
    println!("&str版: {}", generic_function("hello"));
    println!("f64版: {}", generic_function(std::f64::consts::PI));

    // バイナリサイズへの影響
    fn process_many_types() {
        // 各型に対して別々の関数が生成される
        let _: Vec<String> = vec![
            generic_function(1u8),
            generic_function(1u16),
            generic_function(1u32),
            generic_function(1u64),
            generic_function(1i8),
            generic_function(1i16),
            generic_function(1i32),
            generic_function(1i64),
        ];
    }

    process_many_types();

    println!("\nモノモーフィゼーションの影響:");
    println!("  利点: インライン化可能、最適化が効く");
    println!("  欠点: バイナリサイズが増加する可能性");
}

// トレイトオブジェクトのオーバーヘッド
fn trait_object_overhead() {
    println!("\n--- トレイトオブジェクトのオーバーヘッド ---");

    use std::mem::size_of;

    trait MyTrait {
        fn method(&self) -> i32;
    }

    struct SmallStruct {
        value: i32,
    }

    impl MyTrait for SmallStruct {
        fn method(&self) -> i32 {
            self.value
        }
    }

    struct LargeStruct {
        data: [u8; 100],
    }

    impl MyTrait for LargeStruct {
        fn method(&self) -> i32 {
            self.data[0] as i32
        }
    }

    println!("メモリレイアウト:");
    println!("  SmallStruct: {}バイト", size_of::<SmallStruct>());
    println!("  LargeStruct: {}バイト", size_of::<LargeStruct>());
    println!(
        "  &dyn MyTrait: {}バイト（ファットポインタ）",
        size_of::<&dyn MyTrait>()
    );
    println!(
        "  Box<dyn MyTrait>: {}バイト",
        size_of::<Box<dyn MyTrait>>()
    );

    // vtableの説明
    println!("\nvtable（仮想関数テーブル）:");
    println!("  - 各トレイトオブジェクトは2つのポインタを持つ");
    println!("  - データへのポインタ");
    println!("  - vtableへのポインタ（メソッドのアドレス）");

    // 間接呼び出しのコスト
    let small = SmallStruct { value: 42 };
    let trait_obj: &dyn MyTrait = &small;

    const CALLS: usize = 10_000_000;

    // 直接呼び出し
    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..CALLS {
        sum += small.method();
    }
    let direct_time = start.elapsed();

    // sumを使用（最適化で削除されないように）
    println!("  計算結果: {}", sum);

    // 間接呼び出し（トレイトオブジェクト経由）
    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..CALLS {
        sum += trait_obj.method();
    }
    let indirect_time = start.elapsed();

    println!("\nメソッド呼び出しのパフォーマンス（{}回）:", CALLS);
    println!("  直接呼び出し: {:?}", direct_time);
    println!("  間接呼び出し: {:?}", indirect_time);
    println!(
        "  オーバーヘッド: {:.2}x",
        indirect_time.as_nanos() as f64 / direct_time.as_nanos() as f64
    );

    // sumを使用（最適化で削除されないように）
    std::hint::black_box(sum);
}

// 最適化テクニック
fn optimization_techniques() {
    println!("\n--- 最適化テクニック ---");

    // 1. Enum dispatch
    println!("1. Enum Dispatch:");

    #[derive(Debug)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { base, height } => 0.5 * base * height,
            }
        }
    }

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 20.0,
        },
        Shape::Triangle {
            base: 15.0,
            height: 8.0,
        },
    ];

    for shape in &shapes {
        println!("  {:?} -> 面積: {:.2}", shape, shape.area());
    }

    // 2. 特殊化によるパフォーマンス向上
    println!("\n2. 手動特殊化:");

    trait Process {
        fn process(&self, data: &[u8]) -> Vec<u8>;
    }

    struct GeneralProcessor;
    impl Process for GeneralProcessor {
        fn process(&self, data: &[u8]) -> Vec<u8> {
            data.iter().map(|&b| b.wrapping_add(1)).collect()
        }
    }

    struct OptimizedProcessor;
    impl Process for OptimizedProcessor {
        fn process(&self, data: &[u8]) -> Vec<u8> {
            // SIMDやその他の最適化を想定
            let mut result = Vec::with_capacity(data.len());
            for chunk in data.chunks(8) {
                // バッチ処理の例
                for &b in chunk {
                    result.push(b.wrapping_add(1));
                }
            }
            result
        }
    }

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let general = GeneralProcessor;
    let optimized = OptimizedProcessor;

    println!("  一般的な処理: {:?}", general.process(&data));
    println!("  最適化された処理: {:?}", optimized.process(&data));

    // 3. ゼロコスト抽象化
    println!("\n3. ゼロコスト抽象化:");

    trait Zero {
        fn zero() -> Self;
    }

    impl Zero for i32 {
        #[inline(always)]
        fn zero() -> Self {
            0
        }
    }

    impl Zero for f64 {
        #[inline(always)]
        fn zero() -> Self {
            0.0
        }
    }

    #[inline(always)]
    fn sum_with_zero<T: Zero + std::ops::Add<Output = T> + Copy>(slice: &[T]) -> T {
        slice.iter().fold(T::zero(), |acc, &x| acc + x)
    }

    let ints = vec![1, 2, 3, 4, 5];
    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];

    println!("  整数の合計: {}", sum_with_zero(&ints));
    println!("  浮動小数点の合計: {}", sum_with_zero(&floats));
}

// ベンチマークの例
fn benchmark_example() {
    println!("\n--- 実践的なベンチマーク ---");

    // 異なる実装の比較
    trait Sort {
        fn sort(&self, data: &mut [i32]);
        fn name(&self) -> &str;
    }

    struct StandardSort;
    impl Sort for StandardSort {
        fn sort(&self, data: &mut [i32]) {
            data.sort();
        }
        fn name(&self) -> &str {
            "標準ソート"
        }
    }

    struct UnstableSort;
    impl Sort for UnstableSort {
        fn sort(&self, data: &mut [i32]) {
            data.sort_unstable();
        }
        fn name(&self) -> &str {
            "不安定ソート"
        }
    }

    struct CustomSort;
    impl Sort for CustomSort {
        fn sort(&self, data: &mut [i32]) {
            // 小さな配列用の最適化
            if data.len() < 10 {
                // 挿入ソート
                for i in 1..data.len() {
                    let key = data[i];
                    let mut j = i;
                    while j > 0 && data[j - 1] > key {
                        data[j] = data[j - 1];
                        j -= 1;
                    }
                    data[j] = key;
                }
            } else {
                data.sort_unstable();
            }
        }
        fn name(&self) -> &str {
            "カスタムソート"
        }
    }

    // ベンチマーク実行
    fn benchmark_sort<S: Sort + ?Sized>(sort: &S, size: usize) -> std::time::Duration {
        let mut total_time = std::time::Duration::new(0, 0);
        let iterations = 1000;

        for _ in 0..iterations {
            let mut data: Vec<i32> = (0..size).map(|x| x as i32).rev().collect();
            let start = Instant::now();
            sort.sort(&mut data);
            total_time += start.elapsed();
        }

        total_time / iterations as u32
    }

    let sorters: Vec<Box<dyn Sort>> = vec![
        Box::new(StandardSort),
        Box::new(UnstableSort),
        Box::new(CustomSort),
    ];

    println!("ソートアルゴリズムのベンチマーク:");

    for size in &[10, 100, 1000] {
        println!("\n配列サイズ: {}", size);
        for sorter in &sorters {
            let time = benchmark_sort(&**sorter, *size);
            println!("  {}: {:?}", sorter.name(), time);
        }
    }

    // メモリ効率の比較
    println!("\n--- メモリ効率 ---");

    trait DataStructure {
        fn insert(&mut self, value: i32);
        fn contains(&self, value: i32) -> bool;
        fn memory_usage(&self) -> usize;
        fn name(&self) -> &str;
    }

    struct VecStructure {
        data: Vec<i32>,
    }

    impl DataStructure for VecStructure {
        fn insert(&mut self, value: i32) {
            if !self.data.contains(&value) {
                self.data.push(value);
            }
        }

        fn contains(&self, value: i32) -> bool {
            self.data.contains(&value)
        }

        fn memory_usage(&self) -> usize {
            std::mem::size_of::<Vec<i32>>() + self.data.capacity() * std::mem::size_of::<i32>()
        }

        fn name(&self) -> &str {
            "Vec"
        }
    }

    struct HashSetStructure {
        data: std::collections::HashSet<i32>,
    }

    impl DataStructure for HashSetStructure {
        fn insert(&mut self, value: i32) {
            self.data.insert(value);
        }

        fn contains(&self, value: i32) -> bool {
            self.data.contains(&value)
        }

        fn memory_usage(&self) -> usize {
            std::mem::size_of::<std::collections::HashSet<i32>>()
                + self.data.capacity() * std::mem::size_of::<i32>() * 2 // 概算
        }

        fn name(&self) -> &str {
            "HashSet"
        }
    }

    let mut vec_struct = VecStructure { data: Vec::new() };
    let mut hash_struct = HashSetStructure {
        data: std::collections::HashSet::new(),
    };

    // データ挿入
    for i in 0..100 {
        vec_struct.insert(i);
        hash_struct.insert(i);
    }

    println!("データ構造の比較（100要素）:");
    println!(
        "  {} - メモリ使用量: {}バイト",
        vec_struct.name(),
        vec_struct.memory_usage()
    );
    println!(
        "  {} - メモリ使用量: {}バイト",
        hash_struct.name(),
        hash_struct.memory_usage()
    );
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
