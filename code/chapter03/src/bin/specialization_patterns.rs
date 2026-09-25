// src/bin/specialization_patterns.rs

use std::fmt::{Debug, Display};
use std::marker::PhantomData;

fn main() {
    println!("=== 特殊化（Specialization）パターンと代替手法 ===\n");

    specialization_concept();
    marker_trait_pattern();
    type_state_pattern();
    static_dispatch_pattern();
    trait_object_dispatch();
}

// 特殊化の概念（将来の機能）
fn specialization_concept() {
    println!("--- 特殊化の概念 ---");

    // 現在は不安定な機能のため、コメントで説明
    println!("特殊化とは：");
    println!("- より具体的な実装が一般的な実装を上書きできる機能");
    println!("- パフォーマンスの最適化に有用");
    println!("- 現在はnightly版でのみ利用可能\n");

    // 理想的な特殊化の例（実際には動作しない）
    /*
    trait Example {
        fn method(&self) -> &str;
    }

    // 一般的な実装
    impl<T> Example for T {
        default fn method(&self) -> &str {
            "一般的な実装"
        }
    }

    // 特殊化された実装
    impl Example for String {
        fn method(&self) -> &str {
            "String専用の最適化された実装"
        }
    }
    */

    println!("代替パターンを使用して同様の効果を実現します");
}

// マーカートレイトによる特殊化の代替
fn marker_trait_pattern() {
    println!("\n--- マーカートレイトパターン ---");

    // マーカートレイトの定義
    trait Optimizable {}

    // 最適化可能な型にマーカーを実装
    impl Optimizable for String {}
    impl Optimizable for Vec<u8> {}

    // 処理用のトレイト
    trait Process {
        fn process(&self) -> String;
    }

    // デフォルト実装
    impl<T: Debug> Process for T {
        fn process(&self) -> String {
            format!("標準処理: {:?}", self)
        }
    }

    // ラッパー型で特殊化
    struct OptimizedWrapper<T>(T);

    impl Process for OptimizedWrapper<String> {
        fn process(&self) -> String {
            format!("最適化されたString処理: {}", self.0.to_uppercase())
        }
    }

    impl Process for OptimizedWrapper<Vec<u8>> {
        fn process(&self) -> String {
            format!("最適化されたVec<u8>処理: {} bytes", self.0.len())
        }
    }

    // ヘルパー関数
    fn process_value<T: Debug>(value: T) -> String {
        value.process()
    }

    fn process_optimized<T>(value: T) -> String
    where
        OptimizedWrapper<T>: Process,
    {
        OptimizedWrapper(value).process()
    }

    let s = String::from("hello");
    let v = vec![1, 2, 3, 4, 5];
    let n = 42;

    println!("標準処理:");
    println!("  {}", process_value(&s));
    println!("  {}", process_value(&v));
    println!("  {}", process_value(n));

    println!("\n最適化処理:");
    println!("  {}", process_optimized(s));
    println!("  {}", process_optimized(v));
}

// 型状態パターンによる特殊化
fn type_state_pattern() {
    println!("\n--- 型状態パターン ---");

    // 状態を表す型
    struct Generic;
    struct Specialized;

    // コンテナ型
    struct Container<T, State = Generic> {
        data: T,
        _state: PhantomData<State>,
    }

    // 生成は Generic 状態に限る（impl<T, S> に置くと S が決まらず推論できない）
    impl<T> Container<T, Generic> {
        fn new(data: T) -> Self {
            Container {
                data,
                _state: PhantomData,
            }
        }
    }

    // 共通の機能
    impl<T, S> Container<T, S> {
        fn get(&self) -> &T {
            &self.data
        }
    }

    // 一般的な処理
    impl<T: Display> Container<T, Generic> {
        fn process(&self) -> String {
            format!("一般的な処理: {}", self.data)
        }

        fn specialize(self) -> Container<T, Specialized> {
            Container {
                data: self.data,
                _state: PhantomData,
            }
        }
    }

    // 特殊化された処理
    impl<T: Display> Container<T, Specialized> {
        fn process(&self) -> String {
            format!("特殊化された処理: [[ {} ]]", self.data)
        }

        fn advanced_process(&self) -> String {
            format!("高度な処理: {{ {} }}", self.data)
        }
    }

    let generic = Container::new("test data");
    println!("{}", generic.process());

    let specialized = generic.specialize();
    println!("{}", specialized.process());
    println!("{}", specialized.advanced_process());
}

// 静的ディスパッチによる特殊化
fn static_dispatch_pattern() {
    println!("\n--- 静的ディスパッチパターン ---");

    // アルゴリズムの選択
    trait Algorithm {
        fn compute(&self, data: &[i32]) -> i32;
    }

    struct SumAlgorithm;
    impl Algorithm for SumAlgorithm {
        fn compute(&self, data: &[i32]) -> i32 {
            data.iter().sum()
        }
    }

    struct ProductAlgorithm;
    impl Algorithm for ProductAlgorithm {
        fn compute(&self, data: &[i32]) -> i32 {
            data.iter().product()
        }
    }

    struct MaxAlgorithm;
    impl Algorithm for MaxAlgorithm {
        fn compute(&self, data: &[i32]) -> i32 {
            *data.iter().max().unwrap_or(&0)
        }
    }

    // 計算機
    struct Calculator<A: Algorithm> {
        algorithm: A,
    }

    impl<A: Algorithm> Calculator<A> {
        fn new(algorithm: A) -> Self {
            Calculator { algorithm }
        }

        fn calculate(&self, data: &[i32]) -> i32 {
            self.algorithm.compute(data)
        }
    }

    // 型によって異なるデフォルトアルゴリズム
    trait DefaultAlgorithm {
        type Algo: Algorithm;
        fn default_algorithm() -> Self::Algo;
    }

    struct IntData;
    impl DefaultAlgorithm for IntData {
        type Algo = SumAlgorithm;
        fn default_algorithm() -> Self::Algo {
            SumAlgorithm
        }
    }

    struct FactorData;
    impl DefaultAlgorithm for FactorData {
        type Algo = ProductAlgorithm;
        fn default_algorithm() -> Self::Algo {
            ProductAlgorithm
        }
    }

    let data = vec![1, 2, 3, 4, 5];

    let sum_calc = Calculator::new(SumAlgorithm);
    let prod_calc = Calculator::new(ProductAlgorithm);
    let max_calc = Calculator::new(MaxAlgorithm);

    println!("データ: {:?}", data);
    println!("  合計: {}", sum_calc.calculate(&data));
    println!("  積: {}", prod_calc.calculate(&data));
    println!("  最大: {}", max_calc.calculate(&data));
}

// トレイトオブジェクトによる動的ディスパッチ
fn trait_object_dispatch() {
    println!("\n--- 動的ディスパッチによる特殊化 ---");

    // ベーストレイト
    trait Renderer: Debug {
        fn render(&self, content: &str) -> String;
        fn name(&self) -> &str;
    }

    // 具体的な実装
    #[derive(Debug)]
    struct HtmlRenderer;
    impl Renderer for HtmlRenderer {
        fn render(&self, content: &str) -> String {
            format!("<p>{}</p>", content)
        }

        fn name(&self) -> &str {
            "HTML"
        }
    }

    #[derive(Debug)]
    struct MarkdownRenderer;
    impl Renderer for MarkdownRenderer {
        fn render(&self, content: &str) -> String {
            format!("**{}**", content)
        }

        fn name(&self) -> &str {
            "Markdown"
        }
    }

    #[derive(Debug)]
    struct PlainTextRenderer;
    impl Renderer for PlainTextRenderer {
        fn render(&self, content: &str) -> String {
            content.to_string()
        }

        fn name(&self) -> &str {
            "PlainText"
        }
    }

    // レンダリングエンジン
    struct RenderEngine {
        renderers: Vec<Box<dyn Renderer>>,
        default_index: usize,
    }

    impl RenderEngine {
        fn new() -> Self {
            let renderers: Vec<Box<dyn Renderer>> = vec![
                Box::new(PlainTextRenderer),
                Box::new(HtmlRenderer),
                Box::new(MarkdownRenderer),
            ];

            RenderEngine {
                renderers,
                default_index: 0,
            }
        }

        fn render(&self, content: &str, renderer_name: Option<&str>) -> String {
            let renderer = match renderer_name {
                Some(name) => self
                    .renderers
                    .iter()
                    .find(|r| r.name() == name)
                    .unwrap_or(&self.renderers[self.default_index]),
                None => &self.renderers[self.default_index],
            };

            renderer.render(content)
        }

        fn render_all(&self, content: &str) -> Vec<(String, String)> {
            self.renderers
                .iter()
                .map(|r| (r.name().to_string(), r.render(content)))
                .collect()
        }
    }

    let engine = RenderEngine::new();
    let content = "Hello, World!";

    println!("コンテンツ: '{}'", content);
    println!("\nデフォルトレンダリング:");
    println!("  {}", engine.render(content, None));

    println!("\n特定のレンダラー:");
    println!("  HTML: {}", engine.render(content, Some("HTML")));
    println!("  Markdown: {}", engine.render(content, Some("Markdown")));

    println!("\nすべてのレンダラー:");
    for (name, result) in engine.render_all(content) {
        println!("  {}: {}", name, result);
    }

    // パフォーマンスヒント付きの特殊化
    trait Cacheable {
        fn cache_key(&self) -> String;
        fn is_cacheable(&self) -> bool {
            true
        }
    }

    impl Cacheable for HtmlRenderer {
        fn cache_key(&self) -> String {
            "html_v1".to_string()
        }
    }

    impl Cacheable for MarkdownRenderer {
        fn cache_key(&self) -> String {
            "markdown_v1".to_string()
        }
    }

    impl Cacheable for PlainTextRenderer {
        fn cache_key(&self) -> String {
            "plain_v1".to_string()
        }

        fn is_cacheable(&self) -> bool {
            false // プレーンテキストはキャッシュ不要
        }
    }

    println!("\nキャッシュ情報:");
    let html = HtmlRenderer;
    let markdown = MarkdownRenderer;
    let plain = PlainTextRenderer;

    println!(
        "  HTML - キャッシュ可能: {}, キー: {}",
        html.is_cacheable(),
        html.cache_key()
    );
    println!(
        "  Markdown - キャッシュ可能: {}, キー: {}",
        markdown.is_cacheable(),
        markdown.cache_key()
    );
    println!(
        "  PlainText - キャッシュ可能: {}, キー: {}",
        plain.is_cacheable(),
        plain.cache_key()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_state_pattern_runs_without_panicking() {
        super::type_state_pattern();
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
