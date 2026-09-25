// src/bin/async_streams.rs

use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use std::time::Duration;
use tokio::time::{sleep, interval};
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() {
    println!("=== ストリームと非同期イテレータ ===\n");
    
    stream_basics().await;
    stream_combinators().await;
    custom_streams().await;
    stream_processing_patterns().await;
}

// ストリームの基本
async fn stream_basics() {
    println!("--- ストリームの基本 ---");
    
    // ストリームの作成
    println!("  基本的なストリーム:");
    
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // pin_mut!マクロでピン留め
    futures::pin_mut!(stream);
    
    while let Some(value) = stream.next().await {
        println!("    値: {}", value);
    }
    
    // 非同期ストリーム
    println!("\n  非同期ストリーム:");
    
    let async_stream = stream::unfold(0, |state| async move {
        if state < 5 {
            sleep(Duration::from_millis(100)).await;
            Some((state * 2, state + 1))
        } else {
            None
        }
    });
    
    futures::pin_mut!(async_stream);
    
    while let Some(value) = async_stream.next().await {
        println!("    非同期値: {}", value);
    }
    
    // インターバルストリーム
    println!("\n  インターバルストリーム:");
    
    let mut interval_stream = interval(Duration::from_millis(200));
    
    for i in 0..3 {
        interval_stream.tick().await;
        println!("    Tick {}", i);
    }
    
    // ストリームの変換
    println!("\n  ストリーム変換:");
    
    let numbers = stream::iter(1..=5);
    let doubled = numbers.map(|x| x * 2);
    let filtered = doubled.filter(|x| futures::future::ready(*x > 5));
    
    let collected: Vec<i32> = filtered.collect().await;
    println!("    変換結果: {:?}", collected);
}

// ストリームコンビネータ
async fn stream_combinators() {
    println!("\n--- ストリームコンビネータ ---");
    
    // map, filter, fold
    println!("  map/filter/fold:");
    
    let sum = stream::iter(1..=10)
        .map(|x| x * x)
        .filter(|x| futures::future::ready(x % 2 == 0))
        .fold(0, |acc, x| async move {
            println!("    累積: {} + {} = {}", acc, x, acc + x);
            acc + x
        })
        .await;
    
    println!("    最終合計: {}", sum);
    
    // flat_map / flatten
    println!("\n  flat_map:");
    
    let nested = stream::iter(vec![vec![1, 2], vec![3, 4], vec![5]]);
    let flattened: Vec<i32> = nested
        .flat_map(|vec| stream::iter(vec))
        .collect()
        .await;
    
    println!("    フラット化: {:?}", flattened);
    
    // zip と chain
    println!("\n  zip と chain:");
    
    let stream1 = stream::iter(vec!["A", "B", "C"]);
    let stream2 = stream::iter(vec![1, 2, 3]);
    
    let zipped: Vec<(&str, i32)> = stream1.zip(stream2).collect().await;
    println!("    zip結果: {:?}", zipped);
    
    let stream_a = stream::iter(vec![1, 2, 3]);
    let stream_b = stream::iter(vec![4, 5, 6]);
    
    let chained: Vec<i32> = stream_a.chain(stream_b).collect().await;
    println!("    chain結果: {:?}", chained);
    
    // take, skip, take_while
    println!("\n  take/skip/take_while:");
    
    let taken: Vec<i32> = stream::iter(1..=10)
        .take(5)
        .collect()
        .await;
    println!("    最初の5個: {:?}", taken);
    
    let skipped: Vec<i32> = stream::iter(1..=10)
        .skip(5)
        .collect()
        .await;
    println!("    5個スキップ: {:?}", skipped);
    
    let taken_while: Vec<i32> = stream::iter(1..=10)
        .take_while(|&x| futures::future::ready(x < 5))
        .collect()
        .await;
    println!("    条件を満たす間: {:?}", taken_while);
}

// カスタムストリーム
async fn custom_streams() {
    println!("\n--- カスタムストリーム ---");
    
    // 手動でStreamトレイトを実装
    struct CounterStream {
        count: u32,
        max: u32,
    }
    
    impl Stream for CounterStream {
        type Item = u32;
        
        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.count < self.max {
                let current = self.count;
                self.count += 1;
                Poll::Ready(Some(current))
            } else {
                Poll::Ready(None)
            }
        }
    }
    
    let mut counter = CounterStream { count: 0, max: 5 };
    
    println!("  カスタムカウンターストリーム:");
    while let Some(value) = counter.next().await {
        println!("    カウント: {}", value);
    }
    
    // async-streamクレートスタイルの実装
    println!("\n  ジェネレータスタイルのストリーム:");
    
    fn fibonacci_stream() -> impl Stream<Item = u64> {
        stream::unfold((0u64, 1u64), |(a, b)| async move {
            let next = a + b;
            Some((a, (b, next)))
        })
    }
    
    let fib_stream = fibonacci_stream().take(10);
    futures::pin_mut!(fib_stream);
    
    print!("    フィボナッチ数列: ");
    while let Some(value) = fib_stream.next().await {
        print!("{} ", value);
    }
    println!();
    
    // エラーを含むストリーム
    println!("\n  エラーを含むストリーム:");
    
    #[derive(Debug)]
    struct MyError;
    
    let result_stream = stream::iter(vec![
        Ok(1),
        Ok(2),
        Err(MyError),
        Ok(3),
    ]);
    
    // try_collectでエラーハンドリング
    let collected: Result<Vec<i32>, MyError> = result_stream
        .try_collect()
        .await;
    
    match collected {
        Ok(values) => println!("    成功: {:?}", values),
        Err(_) => println!("    エラーが発生しました"),
    }
    
    // エラーをフィルタリング
    let result_stream2 = stream::iter(vec![
        Ok(1),
        Ok(2),
        Err(MyError),
        Ok(3),
    ]);
    
    let filtered: Vec<i32> = result_stream2
        .filter_map(|result| async move { result.ok() })
        .collect()
        .await;
    
    println!("    エラーを除外: {:?}", filtered);
}

// ストリーム処理パターン
async fn stream_processing_patterns() {
    println!("\n--- ストリーム処理パターン ---");
    
    // バッファリング
    println!("  バッファリング:");
    
    let unbuffered = stream::iter(1..=5).then(|x| async move {
        sleep(Duration::from_millis(100)).await;
        x * 2
    });
    
    let start = std::time::Instant::now();
    let results: Vec<i32> = unbuffered.collect().await;
    println!("    バッファなし: {:?} ({:?})", results, start.elapsed());
    
    // buffer_unordered は Future を要素に持つストリームを並行実行するため、then ではなく map で Future を流す
    let buffered = stream::iter(1..=5)
        .map(|x| async move {
            sleep(Duration::from_millis(100)).await;
            x * 2
        })
        .buffer_unordered(3);
    
    let start = std::time::Instant::now();
    let results: Vec<i32> = buffered.collect().await;
    println!("    バッファあり: {:?} ({:?})", results, start.elapsed());
    
    // チャンク処理
    println!("\n  チャンク処理:");
    
    let chunked: Vec<Vec<i32>> = stream::iter(1..=10)
        .chunks(3)
        .collect()
        .await;
    
    for (i, chunk) in chunked.iter().enumerate() {
        println!("    チャンク{}: {:?}", i, chunk);
    }
    
    // タイムアウト付き処理
    println!("\n  タイムアウト付き処理:");
    
    use tokio::time::timeout;
    
    let slow_stream = stream::unfold(0, |state| async move {
        if state < 3 {
            sleep(Duration::from_millis(200)).await;
            Some((state, state + 1))
        } else {
            // 最後の要素は遅い
            sleep(Duration::from_secs(2)).await;
            None
        }
    });
    
    futures::pin_mut!(slow_stream);
    
    while let Ok(Some(value)) = timeout(Duration::from_millis(300), slow_stream.next()).await {
        println!("    タイムアウト内で受信: {}", value);
    }
    println!("    タイムアウト！");
    
    // マージとセレクト
    println!("\n  ストリームのマージ:");
    
    let stream1 = stream::iter(vec![1, 3, 5]).then(|x| async move {
        sleep(Duration::from_millis(100)).await;
        format!("Stream1: {}", x)
    });
    
    let stream2 = stream::iter(vec![2, 4, 6]).then(|x| async move {
        sleep(Duration::from_millis(150)).await;
        format!("Stream2: {}", x)
    });
    
    let merged = stream::select(stream1, stream2);
    futures::pin_mut!(merged);
    
    while let Some(value) = merged.next().await {
        println!("    {}", value);
    }
    
    // 実践的な例：イベントストリーム処理
    println!("\n  イベントストリーム処理:");
    
    #[derive(Debug)]
    enum Event {
        Data(String),
        Error(String),
        Complete,
    }
    
    let event_stream = stream::iter(vec![
        Event::Data("開始".to_string()),
        Event::Data("処理中".to_string()),
        Event::Error("一時的なエラー".to_string()),
        Event::Data("再開".to_string()),
        Event::Complete,
    ]);
    
    let processed = event_stream
        .scan(0, |state, event| {
            match event {
                Event::Data(data) => {
                    *state += 1;
                    futures::future::ready(Some(format!("データ{}: {}", state, data)))
                }
                Event::Error(err) => {
                    futures::future::ready(Some(format!("エラー: {} (状態: {})", err, state)))
                }
                Event::Complete => {
                    futures::future::ready(None)
                }
            }
        });
    
    futures::pin_mut!(processed);
    
    while let Some(result) = processed.next().await {
        println!("    {}", result);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    // 各デモが完走すること（Pending のまま wake されないなどのハングも検出する）
    #[tokio::test]
    async fn stream_processing_patterns_completes() {
        tokio::time::timeout(Duration::from_secs(30), super::stream_processing_patterns())
            .await
            .expect("stream_processing_patterns がハングした");
    }
}
