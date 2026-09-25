// src/bin/problem2_stream_trait.rs
// 復習問題2: 関連型を使ったStreamトレイトの設計

use std::collections::VecDeque;
use std::time::{Duration, Instant};

// 基本ストリームトレイト
trait Stream {
    type Item;
    type Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    // デフォルト実装付きの便利メソッド
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn is_terminated(&self) -> bool;
}

// 高度な機能を提供するトレイト
trait StreamExt: Stream {
    fn collect<C>(self) -> Result<C, Self::Error>
    where
        Self: Sized,
        C: Default + Extend<Self::Item>,
    {
        let mut collection = C::default();
        let mut stream = self;

        while let Some(item) = stream.next()? {
            collection.extend(std::iter::once(item));
        }

        Ok(collection)
    }

    fn fold<B, F>(self, init: B, mut f: F) -> Result<B, Self::Error>
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;
        let mut stream = self;

        while let Some(item) = stream.next()? {
            acc = f(acc, item);
        }

        Ok(acc)
    }

    fn map<F, U>(self, f: F) -> MapStream<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
    {
        MapStream {
            stream: self,
            map_fn: f,
        }
    }

    fn filter<P>(self, predicate: P) -> FilterStream<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        FilterStream {
            stream: self,
            predicate,
        }
    }

    fn take(self, n: usize) -> TakeStream<Self>
    where
        Self: Sized,
    {
        TakeStream {
            stream: self,
            remaining: n,
        }
    }
}

impl<S: Stream> StreamExt for S {}

// 数値ストリームの実装例
struct NumberStream {
    current: i32,
    max: i32,
    step: i32,
    error_at: Option<i32>,
}

#[derive(Debug)]
enum NumberError {
    Overflow,
    InvalidStep,
}

impl NumberStream {
    fn new(start: i32, max: i32, step: i32) -> Self {
        NumberStream {
            current: start,
            max,
            step,
            error_at: None,
        }
    }

    fn with_error_at(mut self, error_at: i32) -> Self {
        self.error_at = Some(error_at);
        self
    }
}

impl Stream for NumberStream {
    type Item = i32;
    type Error = NumberError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.current >= self.max {
            return Ok(None);
        }

        if let Some(error_at) = self.error_at {
            if self.current == error_at {
                return Err(NumberError::Overflow);
            }
        }

        let current = self.current;
        self.current += self.step;
        Ok(Some(current))
    }

    fn is_terminated(&self) -> bool {
        self.current >= self.max
    }
}

// FilterStreamの実装
struct FilterStream<S, P> {
    stream: S,
    predicate: P,
}

impl<S, P> Stream for FilterStream<S, P>
where
    S: Stream,
    P: FnMut(&S::Item) -> bool,
{
    type Item = S::Item;
    type Error = S::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            match self.stream.next()? {
                Some(item) => {
                    if (self.predicate)(&item) {
                        return Ok(Some(item));
                    }
                    // 条件に合わない場合は次のアイテムを試行
                }
                None => return Ok(None),
            }
        }
    }

    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

// MapStreamの実装
struct MapStream<S, F> {
    stream: S,
    map_fn: F,
}

impl<S, F, B> Stream for MapStream<S, F>
where
    S: Stream,
    F: FnMut(S::Item) -> B,
{
    type Item = B;
    type Error = S::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.stream.next()? {
            Some(item) => Ok(Some((self.map_fn)(item))),
            None => Ok(None),
        }
    }

    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

// TakeStreamの実装
struct TakeStream<S> {
    stream: S,
    remaining: usize,
}

impl<S> Stream for TakeStream<S>
where
    S: Stream,
{
    type Item = S::Item;
    type Error = S::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.remaining == 0 {
            return Ok(None);
        }

        match self.stream.next()? {
            Some(item) => {
                self.remaining -= 1;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }

    fn is_terminated(&self) -> bool {
        self.remaining == 0 || self.stream.is_terminated()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.stream.size_hint();
        (
            lower.min(self.remaining),
            upper
                .map(|u| u.min(self.remaining))
                .or(Some(self.remaining)),
        )
    }
}

// バッチ処理ストリーム
struct BatchStream<S: Stream> {
    stream: S,
    batch_size: usize,
    buffer: Vec<S::Item>,
}

impl<S> BatchStream<S>
where
    S: Stream,
{
    fn new(stream: S, batch_size: usize) -> Self {
        BatchStream {
            stream,
            batch_size,
            buffer: Vec::with_capacity(batch_size),
        }
    }
}

impl<S> Stream for BatchStream<S>
where
    S: Stream,
{
    type Item = Vec<S::Item>;
    type Error = S::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.buffer.clear();

        while self.buffer.len() < self.batch_size {
            match self.stream.next()? {
                Some(item) => self.buffer.push(item),
                None => break,
            }
        }

        if self.buffer.is_empty() {
            Ok(None)
        } else {
            Ok(Some(std::mem::take(&mut self.buffer)))
        }
    }

    fn is_terminated(&self) -> bool {
        self.stream.is_terminated() && self.buffer.is_empty()
    }
}

// CSVパーサーストリーム
struct CsvStream {
    lines: Vec<String>,
    current_line: usize,
    delimiter: char,
}

impl CsvStream {
    fn new(csv_data: &str, delimiter: char) -> Self {
        let lines: Vec<String> = csv_data
            .lines()
            .skip(1) // ヘッダーをスキップ
            .map(|line| line.to_string())
            .collect();

        CsvStream {
            lines,
            current_line: 0,
            delimiter,
        }
    }
}

#[derive(Debug)]
struct CsvRecord {
    fields: Vec<String>,
}

#[derive(Debug)]
enum CsvError {
    ParseError(String),
    InvalidFormat,
}

impl Stream for CsvStream {
    type Item = CsvRecord;
    type Error = CsvError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.current_line >= self.lines.len() {
            return Ok(None);
        }

        let line = &self.lines[self.current_line];
        self.current_line += 1;

        if line.trim().is_empty() {
            return self.next(); // 空行をスキップ
        }

        let fields: Vec<String> = line
            .split(self.delimiter)
            .map(|field| field.trim().to_string())
            .collect();

        if fields.is_empty() {
            return Err(CsvError::InvalidFormat);
        }

        Ok(Some(CsvRecord { fields }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.lines.len() - self.current_line;
        (remaining, Some(remaining))
    }

    fn is_terminated(&self) -> bool {
        self.current_line >= self.lines.len()
    }
}

fn main() {
    println!("=== 復習問題2: Streamトレイト設計 ===\n");

    // 基本的なStreamトレイト
    basic_stream_examples();

    // 高度なStreamトレイト
    advanced_stream_examples();

    // 非同期ストリーム（概念的）
    async_stream_concepts();

    // 実用的なストリーム応用
    practical_stream_applications();
}

// 基本的なStreamトレイトの例
fn basic_stream_examples() {
    println!("【基本的なStreamトレイト】");
    // 数値ストリームの実装例を使った基本デモ
    let mut number_stream = NumberStream::new(0, 10, 2);
    println!("数値ストリームのテスト:");

    while let Ok(Some(value)) = number_stream.next() {
        println!("  値: {}", value);
        if value >= 8 {
            break;
        }
    }

    // 使用例
    println!("基本的な数値ストリーム:");
    let mut stream = NumberStream::new(0, 5, 1);

    while let Ok(Some(item)) = stream.next() {
        println!("  項目: {}", item);
    }

    // collect使用例
    println!("\ncollectの例:");
    let stream2 = NumberStream::new(1, 4, 1);
    match stream2.collect::<Vec<i32>>() {
        Ok(numbers) => println!("  収集された数値: {:?}", numbers),
        Err(e) => println!("  エラー: {:?}", e),
    }

    // fold使用例
    println!("\nfoldの例:");
    let stream3 = NumberStream::new(1, 6, 1);
    match stream3.fold(0, |acc, x| acc + x) {
        Ok(sum) => println!("  合計: {}", sum),
        Err(e) => println!("  エラー: {:?}", e),
    }

    // エラーハンドリング例
    println!("\nエラーハンドリングの例:");
    let mut error_stream = NumberStream::new(0, 10, 1).with_error_at(3);

    loop {
        match error_stream.next() {
            Ok(Some(item)) => println!("  項目: {}", item),
            Ok(None) => {
                println!("  ストリーム終了");
                break;
            }
            Err(e) => {
                println!("  エラー発生: {:?}", e);
                break;
            }
        }
    }
}

// 高度なStreamトレイトの例
fn advanced_stream_examples() {
    println!("\n【高度なStream機能】");

    // チェーンした変換操作
    println!("チェーンした変換操作:");
    let stream = NumberStream::new(0, 20, 1)
        .filter(|&x| x % 2 == 0) // 偶数のみ
        .map(|x| x * x) // 二乗
        .take(5); // 最初の5つ

    match stream.collect::<Vec<i32>>() {
        Ok(result) => println!("  結果: {:?}", result),
        Err(e) => println!("  エラー: {:?}", e),
    }

    // 複雑なストリーム：文字列処理
    struct LineStream {
        lines: VecDeque<String>,
        error_rate: f64,
        processed: usize,
    }

    impl LineStream {
        fn new(text: &str, error_rate: f64) -> Self {
            let lines: VecDeque<String> = text.lines().map(|line| line.to_string()).collect();

            LineStream {
                lines,
                error_rate,
                processed: 0,
            }
        }
    }

    #[derive(Debug)]
    enum LineError {
        ProcessingError,
        EmptyLine,
    }

    impl Stream for LineStream {
        type Item = String;
        type Error = LineError;

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            if let Some(line) = self.lines.pop_front() {
                self.processed += 1;

                // エラー率に基づく確率的エラー
                if (self.processed as f64 * self.error_rate) % 1.0 < self.error_rate {
                    return Err(LineError::ProcessingError);
                }

                if line.trim().is_empty() {
                    return Err(LineError::EmptyLine);
                }

                Ok(Some(line.trim().to_uppercase()))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lines.len(), Some(self.lines.len()))
        }

        fn is_terminated(&self) -> bool {
            self.lines.is_empty()
        }
    }

    println!("\n文字列処理ストリーム:");
    let text = "hello world\nrust programming\n\ngood bye\nawesome";
    let mut line_stream = LineStream::new(text, 0.2);

    loop {
        match line_stream.next() {
            Ok(Some(line)) => println!("  処理済み行: {}", line),
            Ok(None) => {
                println!("  ストリーム完了");
                break;
            }
            Err(LineError::EmptyLine) => {
                println!("  空行をスキップ");
                continue;
            }
            Err(e) => {
                println!("  エラー: {:?}", e);
                break;
            }
        }
    }
}

// 非同期ストリーム（概念的説明）
fn async_stream_concepts() {
    println!("\n【非同期ストリーム（概念）】");

    // 非同期ストリームのトレイト設計（実際のasync/awaitは使用しない）
    trait AsyncStream {
        type Item;
        type Error;

        // 実際にはPinやFutureを使用するが、ここでは概念的に
        fn poll_next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
        fn is_ready(&self) -> bool;
    }

    // タイマーベースのストリーム（疑似実装）
    struct TimedStream {
        interval: Duration,
        last_emit: Instant,
        count: usize,
        max_count: usize,
    }

    impl TimedStream {
        fn new(interval: Duration, max_count: usize) -> Self {
            TimedStream {
                interval,
                last_emit: Instant::now(),
                count: 0,
                max_count,
            }
        }
    }

    #[derive(Debug)]
    enum TimedError {
        Timeout,
    }

    impl AsyncStream for TimedStream {
        type Item = usize;
        type Error = TimedError;

        fn poll_next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            if self.count >= self.max_count {
                return Ok(None);
            }

            if !self.is_ready() {
                return Ok(None); // まだ準備できていない
            }

            self.last_emit = Instant::now();
            let current = self.count;
            self.count += 1;
            Ok(Some(current))
        }

        fn is_ready(&self) -> bool {
            self.last_emit.elapsed() >= self.interval
        }
    }

    println!("非同期ストリームの概念:");
    println!("  • poll_next(): ノンブロッキングで次の項目を試行");
    println!("  • is_ready(): データが準備できているかチェック");
    println!("  • 実際の実装ではFutureとPinを使用");
    println!("  • async/awaitと組み合わせて非同期処理を実現");

    // 疑似的な使用例
    let mut timed_stream = TimedStream::new(Duration::from_millis(100), 3);
    println!("\nタイマーストリームの疑似実行:");

    for attempt in 0..10 {
        match timed_stream.poll_next() {
            Ok(Some(item)) => println!("  試行{}: アイテム {}", attempt, item),
            Ok(None) => {
                if timed_stream.count >= timed_stream.max_count {
                    println!("  試行{}: ストリーム完了", attempt);
                    break;
                } else {
                    println!("  試行{}: 準備未完了", attempt);
                }
            }
            Err(e) => println!("  試行{}: エラー {:?}", attempt, e),
        }

        // 実際の非同期環境では await を使用
        std::thread::sleep(Duration::from_millis(50));
    }
}

// 実用的なストリーム応用
fn practical_stream_applications() {
    println!("\n【実用的なストリーム応用】");

    // 使用例
    println!("CSVストリーム処理:");
    let csv_data = r#"name,age,city
Alice,30,Tokyo
Bob,25,Osaka
Charlie,35,Kyoto
Diana,28,Nagoya"#;

    let csv_stream = CsvStream::new(csv_data, ',');
    let batch_stream = BatchStream::new(csv_stream, 2);

    match batch_stream.collect::<Vec<Vec<CsvRecord>>>() {
        Ok(batches) => {
            for (i, batch) in batches.iter().enumerate() {
                println!("  バッチ {}: {} レコード", i + 1, batch.len());
                for record in batch {
                    println!("    {:?}", record.fields);
                }
            }
        }
        Err(e) => println!("  CSVエラー: {:?}", e),
    }

    // 統計ストリーム
    struct StatsStream<S> {
        stream: S,
        count: usize,
        sum: f64,
        min: Option<f64>,
        max: Option<f64>,
    }

    impl<S> StatsStream<S> {
        fn new(stream: S) -> Self {
            StatsStream {
                stream,
                count: 0,
                sum: 0.0,
                min: None,
                max: None,
            }
        }

        fn get_stats(&self) -> (usize, f64, Option<f64>, Option<f64>) {
            (self.count, self.sum, self.min, self.max)
        }
    }

    impl<S> Stream for StatsStream<S>
    where
        S: Stream<Item = f64>,
    {
        type Item = f64;
        type Error = S::Error;

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            match self.stream.next()? {
                Some(value) => {
                    self.count += 1;
                    self.sum += value;
                    self.min = Some(self.min.map_or(value, |min| min.min(value)));
                    self.max = Some(self.max.map_or(value, |max| max.max(value)));
                    Ok(Some(value))
                }
                None => Ok(None),
            }
        }

        fn is_terminated(&self) -> bool {
            self.stream.is_terminated()
        }
    }

    // 数値データの統計処理
    println!("\n統計ストリーム処理:");

    struct FloatStream {
        values: Vec<f64>,
        index: usize,
    }

    impl FloatStream {
        fn new(values: Vec<f64>) -> Self {
            FloatStream { values, index: 0 }
        }
    }

    impl Stream for FloatStream {
        type Item = f64;
        type Error = ();

        fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn is_terminated(&self) -> bool {
            self.index >= self.values.len()
        }
    }

    let data = vec![1.5, 2.3, 0.8, 4.1, 3.7, 1.2, 5.0];
    let mut stats_stream = StatsStream::new(FloatStream::new(data.clone()));

    // collect は self を消費して統計が読めなくなるため、next で最後まで読む
    while let Ok(Some(_)) = stats_stream.next() {}

    let (count, sum, min, max) = stats_stream.get_stats();
    println!("  データ数: {}", count);
    println!("  合計: {:.2}", sum);
    println!("  平均: {:.2}", sum / count as f64);
    println!("  最小値: {:.2}", min.unwrap_or(0.0));
    println!("  最大値: {:.2}", max.unwrap_or(0.0));

    println!("\n【設計の重要ポイント】");
    println!("✓ 関連型で型安全性を確保");
    println!("✓ エラーハンドリングを組み込んだ設計");
    println!("✓ 遅延評価による効率的なデータ処理");
    println!("✓ コンポーザブルな変換操作");
    println!("✓ サイズヒントによる最適化支援");
    println!("✓ 実用的なドメインへの応用");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_stream_groups_items_and_emits_partial_last_batch() {
        let mut batches = BatchStream::new(NumberStream::new(0, 5, 1), 2);

        assert_eq!(batches.next().unwrap(), Some(vec![0, 1]));
        assert_eq!(batches.next().unwrap(), Some(vec![2, 3]));
        assert!(!batches.is_terminated());
        assert_eq!(batches.next().unwrap(), Some(vec![4]));
        assert!(batches.is_terminated());
        assert_eq!(batches.next().unwrap(), None);
    }

    #[test]
    fn batch_stream_returns_none_for_empty_source() {
        let mut batches = BatchStream::new(NumberStream::new(0, 0, 1), 3);

        assert_eq!(batches.next().unwrap(), None);
        assert!(batches.is_terminated());
    }

    #[test]
    fn batch_stream_propagates_source_error() {
        let mut batches = BatchStream::new(NumberStream::new(0, 10, 1).with_error_at(1), 3);

        assert!(matches!(batches.next(), Err(NumberError::Overflow)));
    }

    const CSV: &str = "name,age\nAlice,30\n\nBob,25";

    #[test]
    fn csv_stream_skips_header_and_blank_lines() {
        let mut csv = CsvStream::new(CSV, ',');

        assert_eq!(csv.next().unwrap().unwrap().fields, vec!["Alice", "30"]);
        assert_eq!(csv.next().unwrap().unwrap().fields, vec!["Bob", "25"]);
        assert!(csv.next().unwrap().is_none());
    }

    #[test]
    fn csv_stream_splits_on_given_delimiter_and_trims_fields() {
        let mut csv = CsvStream::new("h\n a ; b ", ';');

        assert_eq!(csv.next().unwrap().unwrap().fields, vec!["a", "b"]);
    }

    #[test]
    fn csv_stream_is_terminated_only_after_last_line_is_consumed() {
        let mut csv = CsvStream::new("h\nx\ny", ',');
        assert!(!csv.is_terminated());
        assert_eq!(csv.size_hint(), (2, Some(2)));

        csv.next().unwrap();
        assert!(!csv.is_terminated());
        assert_eq!(csv.size_hint(), (1, Some(1)));

        csv.next().unwrap();
        assert!(csv.is_terminated());
        assert_eq!(csv.size_hint(), (0, Some(0)));
    }

    #[test]
    fn csv_stream_with_header_only_is_terminated_immediately() {
        let mut csv = CsvStream::new("h", ',');

        assert!(csv.is_terminated());
        assert!(csv.next().unwrap().is_none());
    }

    #[test]
    fn demos_run_without_panicking() {
        basic_stream_examples();
        practical_stream_applications();
    }
}
