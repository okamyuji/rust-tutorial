// src/bin/hrtb_examples.rs

// use std::fmt::Debug; // 現在未使用

fn main() {
    println!("=== 高階トレイト境界（HRTB）===\n");

    basic_hrtb();
    closure_with_hrtb();
    trait_objects_and_hrtb();
    practical_hrtb_examples();
    advanced_hrtb_patterns();
    why_hrtb_is_needed();
}

fn basic_hrtb() {
    println!("--- HRTBの基本 ---");

    // 基本的なクロージャ
    let closure = |s: &str| s.len();
    let result = apply_to_string(closure, "Hello");
    println!("文字列の長さ: {}", result);

    // より複雑なクロージャ
    let prefix_closure = |s: &str| format!("prefix: {}", s);
    let prefixed = apply_string_transform(prefix_closure, "test");
    println!("変換結果: {}", prefixed);
}

// for<'a> を使用したHRTB
fn apply_to_string<F>(f: F, s: &str) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    // fは任意のライフタイム'aに対して動作する必要がある
    f(s)
}

fn apply_string_transform<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> String,
{
    f(s)
}

fn closure_with_hrtb() {
    println!("\n--- クロージャとHRTB ---");

    // 異なるライフタイムで動作するクロージャ
    let processor = StringProcessor::new();

    // 短いライフタイム
    {
        let temp = String::from("一時的な文字列");
        let result = processor.process(&temp);
        println!("一時的な結果: {}", result);
    }

    // 長いライフタイム
    let permanent = "永続的な文字列";
    let result = processor.process(permanent);
    println!("永続的な結果: {}", result);

    // 複数の参照を扱うクロージャ
    demonstrate_multi_ref_closure();
}

struct StringProcessor;

impl StringProcessor {
    fn new() -> Self {
        StringProcessor
    }

    // HRTBを使用したメソッドは削除（重複のため）

    // 実際の処理メソッド
    fn process(&self, s: &str) -> String {
        format!("処理済み: {}", s)
    }
}

fn demonstrate_multi_ref_closure() {
    // 複数の参照を扱うクロージャ
    let comparator = |x: &str, y: &str| x.len().cmp(&y.len());

    let result = compare_with_closure(comparator, "short", "much longer");
    println!("比較結果: {:?}", result);
}

fn compare_with_closure<F>(f: F, x: &str, y: &str) -> std::cmp::Ordering
where
    F: for<'a, 'b> Fn(&'a str, &'b str) -> std::cmp::Ordering,
{
    f(x, y)
}

fn trait_objects_and_hrtb() {
    println!("\n--- トレイトオブジェクトとHRTB ---");

    // トレイトオブジェクトでのHRTB
    let handler: Box<dyn for<'a> Fn(&'a str) -> String> =
        Box::new(|s| format!("ボックス化: {}", s));

    let result = handler("テスト");
    println!("トレイトオブジェクトの結果: {}", result);

    // 複数のハンドラ
    let handlers: Vec<Box<dyn for<'a> Fn(&'a str) -> String>> = vec![
        Box::new(|s| format!("ハンドラ1: {}", s)),
        Box::new(|s| format!("ハンドラ2: {}", s)),
        Box::new(|s| s.to_uppercase()),
    ];

    for (i, handler) in handlers.iter().enumerate() {
        let result = handler("入力");
        println!("ハンドラ{}: {}", i, result);
    }
}

fn practical_hrtb_examples() {
    println!("\n--- 実践的なHRTBの例 ---");

    // イテレータ変換
    demonstrate_iterator_hrtb();

    // コールバックシステム
    demonstrate_callback_system();

    // フィルタリングシステム
    demonstrate_filter_system();
}

fn demonstrate_iterator_hrtb() {
    println!("\n[イテレータ変換]");

    let numbers = vec![1, 2, 3, 4, 5];
    let strings = vec!["one", "two", "three"];

    // 任意のイテレータに対して動作する関数
    let sum = process_iterator(&numbers, |iter| iter.sum::<i32>());
    println!("数値の合計: {}", sum);

    let concatenated = process_iterator(&strings, |iter| {
        iter.copied().collect::<Vec<_>>().join(", ")
    });
    println!("文字列の結合: {}", concatenated);
}

fn process_iterator<T, F, R>(items: &[T], f: F) -> R
where
    F: for<'a> FnOnce(std::slice::Iter<'a, T>) -> R,
{
    f(items.iter())
}

fn demonstrate_callback_system() {
    println!("\n[コールバックシステム]");

    let mut event_handler = EventHandler::new();

    // 異なるライフタイムのコールバックを登録
    event_handler.register(|data: &str| {
        println!("イベント受信: {}", data);
    });

    event_handler.trigger("イベント1");
    event_handler.trigger("イベント2");
}

struct EventHandler {
    callbacks: Vec<Box<dyn for<'a> Fn(&'a str)>>,
}

impl EventHandler {
    fn new() -> Self {
        EventHandler {
            callbacks: Vec::new(),
        }
    }

    fn register<F>(&mut self, callback: F)
    where
        F: for<'a> Fn(&'a str) + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    fn trigger(&self, data: &str) {
        for callback in &self.callbacks {
            callback(data);
        }
    }
}

fn demonstrate_filter_system() {
    println!("\n[フィルタリングシステム]");

    let items = vec!["apple", "banana", "cherry", "date"];

    // 長さフィルタ
    let long_items = filter_items(&items, |s| s.len() > 5);
    println!("長い項目: {:?}", long_items);

    // 文字フィルタ
    let a_items = filter_items(&items, |s| s.starts_with('a'));
    println!("'a'で始まる項目: {:?}", a_items);
}

fn filter_items<'c, T, F>(items: &'c [T], predicate: F) -> Vec<&'c T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|item| predicate(item)).collect()
}

fn advanced_hrtb_patterns() {
    println!("\n--- 高度なHRTBパターン ---");

    // 複雑なトレイト境界
    demonstrate_complex_bounds();

    // HRTBとasync/await（概念的な例）
    demonstrate_async_concepts();
}

fn demonstrate_complex_bounds() {
    println!("\n[複雑なトレイト境界]");

    // 複数の制約を持つHRTB
    let processor = ComplexProcessor;
    let result = processor.process_with_context("データ", "コンテキスト");
    println!("複雑な処理結果: {}", result);
}

struct ComplexProcessor;

impl ComplexProcessor {
    fn process_with_context(&self, data: &str, context: &str) -> String {
        // 実際の実装
        format!("処理: {} (コンテキスト: {})", data, context)
    }
}

fn demonstrate_async_concepts() {
    println!("\n[非同期処理の概念]");

    // HRTBは非同期トレイトでも重要
    let handler = AsyncHandler::new();
    handler.simulate_async_process("非同期データ");
}

struct AsyncHandler;

impl AsyncHandler {
    fn new() -> Self {
        AsyncHandler
    }

    // 非同期処理のシミュレーション（実際のasyncではない）
    fn simulate_async_process(&self, data: &str) {
        println!("非同期処理をシミュレート: {}", data);

        // 実際の非同期コードでは、以下のようなHRTBが使われることがある：
        // where F: for<'a> Fn(&'a str) -> Pin<Box<dyn Future<Output = String> + 'a>>
    }
}

// HRTBが必要な理由を示す例
fn why_hrtb_is_needed() {
    // この関数は特定のライフタイムに依存しない
    fn needs_hrtb<F>(f: F)
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        // 異なるライフタイムで呼び出される可能性がある
        let s1 = String::from("temporary");
        let result1 = f(&s1);

        let s2 = "static string";
        let result2 = f(s2);

        println!("結果1: {}, 結果2: {}", result1, result2);
    }

    // HRTBなしでは、特定のライフタイムに制限される
    // fn without_hrtb<'a, F>(f: F)
    // where F: Fn(&'a str) -> &'a str
    // この場合、'aは関数全体で固定される

    // needs_hrtb関数を使用
    needs_hrtb(|s| s.trim());
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
