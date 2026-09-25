// src/bin/trait_patterns.rs

use std::fmt::Debug;
use std::marker::PhantomData;

fn main() {
    println!("=== 高度なトレイト設計パターン ===\n");

    builder_pattern();
    state_pattern();
    visitor_pattern();
    strategy_pattern();
    extension_trait_pattern();
}

// Builderパターン
fn builder_pattern() {
    println!("--- Builderパターン ---");

    // 構築する構造体
    #[derive(Debug)]
    struct Server {
        host: String,
        port: u16,
        max_connections: usize,
        timeout: u64,
        use_tls: bool,
    }

    // 状態を表す型
    struct NoHost;
    struct NoPort;
    struct Configured;

    // ビルダー
    struct ServerBuilder<HostState = NoHost, PortState = NoPort> {
        host: Option<String>,
        port: Option<u16>,
        max_connections: usize,
        timeout: u64,
        use_tls: bool,
        _host_state: PhantomData<HostState>,
        _port_state: PhantomData<PortState>,
    }

    impl ServerBuilder<NoHost, NoPort> {
        fn new() -> Self {
            ServerBuilder {
                host: None,
                port: None,
                max_connections: 100,
                timeout: 30,
                use_tls: false,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl<P> ServerBuilder<NoHost, P> {
        fn host(self, host: impl Into<String>) -> ServerBuilder<Configured, P> {
            ServerBuilder {
                host: Some(host.into()),
                port: self.port,
                max_connections: self.max_connections,
                timeout: self.timeout,
                use_tls: self.use_tls,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl<H> ServerBuilder<H, NoPort> {
        fn port(self, port: u16) -> ServerBuilder<H, Configured> {
            ServerBuilder {
                host: self.host,
                port: Some(port),
                max_connections: self.max_connections,
                timeout: self.timeout,
                use_tls: self.use_tls,
                _host_state: PhantomData,
                _port_state: PhantomData,
            }
        }
    }

    impl<H, P> ServerBuilder<H, P> {
        fn max_connections(mut self, max: usize) -> Self {
            self.max_connections = max;
            self
        }

        fn timeout(mut self, timeout: u64) -> Self {
            self.timeout = timeout;
            self
        }

        fn use_tls(mut self, use_tls: bool) -> Self {
            self.use_tls = use_tls;
            self
        }
    }

    impl ServerBuilder<Configured, Configured> {
        fn build(self) -> Server {
            Server {
                host: self.host.unwrap(),
                port: self.port.unwrap(),
                max_connections: self.max_connections,
                timeout: self.timeout,
                use_tls: self.use_tls,
            }
        }
    }

    // 使用例
    let server = ServerBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(200)
        .timeout(60)
        .use_tls(true)
        .build();

    println!("構築されたサーバー: {:?}", server);

    // コンパイルエラー：必須フィールドが不足
    // let incomplete = ServerBuilder::new()
    //     .host("localhost")
    //     .build(); // エラー：portが設定されていない
}

// Stateパターン
fn state_pattern() {
    println!("\n--- Stateパターン ---");

    // 状態トレイト
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn state_name(&self) -> &str;
    }

    // ブログ記事
    struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        fn add_text(&mut self, text: &str) {
            if self.state.as_ref().unwrap().state_name() == "Draft" {
                self.content.push_str(text);
            }
        }

        fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }

        fn state_name(&self) -> &str {
            self.state.as_ref().unwrap().state_name()
        }
    }

    // 具体的な状態
    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview { approval_count: 0 })
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn state_name(&self) -> &str {
            "Draft"
        }
    }

    struct PendingReview {
        approval_count: u32,
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.approval_count += 1;
            if self.approval_count >= 2 {
                Box::new(Published {})
            } else {
                self
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }

        fn state_name(&self) -> &str {
            "PendingReview"
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn state_name(&self) -> &str {
            "Published"
        }
    }

    let mut post = Post::new();

    post.add_text("今日は素晴らしい一日でした。");
    println!("Draft: \"{}\"", post.content());

    post.request_review();
    println!("PendingReview: \"{}\"", post.content());

    post.approve(); // 1回目の承認
    println!("状態: {}", post.state_name());

    post.approve(); // 2回目の承認
    println!("Published: \"{}\"", post.content());
}

// Visitorパターン
fn visitor_pattern() {
    println!("\n--- Visitorパターン ---");

    // 要素トレイト
    trait Element: std::fmt::Debug {
        fn accept(&self, visitor: &mut dyn Visitor);
    }

    // ビジタートレイト
    trait Visitor {
        fn visit_file(&mut self, file: &File);
        fn visit_directory(&mut self, dir: &Directory);
    }

    // 具体的な要素
    #[derive(Debug)]
    struct File {
        name: String,
        size: u64,
    }

    impl Element for File {
        fn accept(&self, visitor: &mut dyn Visitor) {
            visitor.visit_file(self);
        }
    }

    #[derive(Debug)]
    struct Directory {
        name: String,
        children: Vec<Box<dyn Element>>,
    }

    impl Element for Directory {
        fn accept(&self, visitor: &mut dyn Visitor) {
            visitor.visit_directory(self);
            for child in &self.children {
                child.accept(visitor);
            }
        }
    }

    // 具体的なビジター
    struct SizeCalculator {
        total_size: u64,
    }

    impl Visitor for SizeCalculator {
        fn visit_file(&mut self, file: &File) {
            self.total_size += file.size;
            println!("  ファイル: {} ({}バイト)", file.name, file.size);
        }

        fn visit_directory(&mut self, dir: &Directory) {
            println!("ディレクトリ: {}/", dir.name);
        }
    }

    struct FileCounter {
        file_count: usize,
        dir_count: usize,
    }

    impl Visitor for FileCounter {
        fn visit_file(&mut self, file: &File) {
            self.file_count += 1;
            println!("  カウント: ファイル '{}'", file.name);
        }

        fn visit_directory(&mut self, dir: &Directory) {
            self.dir_count += 1;
            println!("カウント: ディレクトリ '{}'", dir.name);
        }
    }

    // ファイルシステムの構築
    let root = Directory {
        name: "root".to_string(),
        children: vec![
            Box::new(File {
                name: "file1.txt".to_string(),
                size: 1024,
            }),
            Box::new(Directory {
                name: "src".to_string(),
                children: vec![
                    Box::new(File {
                        name: "main.rs".to_string(),
                        size: 2048,
                    }),
                    Box::new(File {
                        name: "lib.rs".to_string(),
                        size: 512,
                    }),
                ],
            }),
            Box::new(File {
                name: "README.md".to_string(),
                size: 256,
            }),
        ],
    };

    println!("サイズ計算:");
    let mut size_calc = SizeCalculator { total_size: 0 };
    root.accept(&mut size_calc);
    println!("合計サイズ: {}バイト\n", size_calc.total_size);

    println!("ファイル数カウント:");
    let mut counter = FileCounter {
        file_count: 0,
        dir_count: 0,
    };
    root.accept(&mut counter);
    println!(
        "合計: {}個のファイル、{}個のディレクトリ",
        counter.file_count, counter.dir_count
    );
}

// Strategyパターン
fn strategy_pattern() {
    println!("\n--- Strategyパターン ---");

    // 戦略トレイト
    trait SortStrategy<T> {
        fn sort(&self, data: &mut [T]);
    }

    // 具体的な戦略
    struct QuickSort;
    impl<T: Ord> SortStrategy<T> for QuickSort {
        fn sort(&self, data: &mut [T]) {
            println!("  QuickSortを使用");
            data.sort_unstable();
        }
    }

    struct MergeSort;
    impl<T: Ord + Clone> SortStrategy<T> for MergeSort {
        fn sort(&self, data: &mut [T]) {
            println!("  MergeSortを使用");
            // 簡略化のため標準のsortを使用
            data.sort();
        }
    }

    struct InsertionSort;
    impl<T: Ord> SortStrategy<T> for InsertionSort {
        fn sort(&self, data: &mut [T]) {
            println!("  InsertionSortを使用");
            for i in 1..data.len() {
                let mut j = i;
                while j > 0 && data[j - 1] > data[j] {
                    data.swap(j - 1, j);
                    j -= 1;
                }
            }
        }
    }

    // コンテキスト
    struct Sorter<T> {
        data: Vec<T>,
    }

    impl<T: Ord + Clone + Debug> Sorter<T> {
        fn new(data: Vec<T>) -> Self {
            Sorter { data }
        }

        fn sort_with<S: SortStrategy<T>>(&mut self, strategy: S) {
            strategy.sort(&mut self.data);
        }

        fn get_data(&self) -> &[T] {
            &self.data
        }
    }

    // 使用例
    let data = vec![64, 34, 25, 12, 22, 11, 90];

    let mut sorter1 = Sorter::new(data.clone());
    sorter1.sort_with(QuickSort);
    println!("結果: {:?}", sorter1.get_data());

    let mut sorter2 = Sorter::new(data.clone());
    sorter2.sort_with(MergeSort);
    println!("結果: {:?}", sorter2.get_data());

    let mut sorter3 = Sorter::new(vec![5, 2, 4, 6, 1, 3]);
    sorter3.sort_with(InsertionSort);
    println!("結果: {:?}", sorter3.get_data());
}

// Extension Traitパターン
fn extension_trait_pattern() {
    println!("\n--- Extension Traitパターン ---");

    // 既存の型に機能を追加
    trait StringExt {
        fn indent(&self, level: usize) -> String;
        fn truncate_with_ellipsis(&self, max_len: usize) -> String;
        fn capitalize_words(&self) -> String;
    }

    impl StringExt for str {
        fn indent(&self, level: usize) -> String {
            let indent = " ".repeat(level * 2);
            self.lines()
                .map(|line| format!("{}{}", indent, line))
                .collect::<Vec<_>>()
                .join("\n")
        }

        fn truncate_with_ellipsis(&self, max_len: usize) -> String {
            if self.len() <= max_len {
                self.to_string()
            } else {
                format!("{}...", &self[..max_len.saturating_sub(3)])
            }
        }

        fn capitalize_words(&self) -> String {
            self.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }

    let text = "hello rust world";
    println!("元のテキスト: '{}'", text);
    println!("大文字化: '{}'", text.capitalize_words());
    println!("インデント:\n{}", text.indent(2));

    let long_text = "This is a very long text that needs to be truncated";
    println!("\n長いテキスト: '{}'", long_text);
    println!("切り詰め(20): '{}'", long_text.truncate_with_ellipsis(20));

    // イテレータ拡張
    trait IteratorExt: Iterator {
        fn collect_results<T, E>(self) -> Result<Vec<T>, E>
        where
            Self: Sized + Iterator<Item = Result<T, E>>,
        {
            self.collect()
        }

        fn average<T>(self) -> Option<f64>
        where
            Self: Sized + Iterator<Item = T>,
            T: Into<f64>,
        {
            let mut sum = 0.0;
            let mut count = 0;

            for item in self {
                sum += item.into();
                count += 1;
            }

            if count > 0 {
                Some(sum / count as f64)
            } else {
                None
            }
        }
    }

    impl<I: Iterator> IteratorExt for I {}

    let numbers = [1, 2, 3, 4, 5];
    let avg = numbers.iter().copied().average::<i32>();
    println!("\n平均値: {:?}", avg);

    let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];

    let collected: Result<Vec<i32>, &str> = results.into_iter().collect_results();
    println!("収集結果: {:?}", collected);
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
