// src/bin/async_lifetimes.rs

use std::time::Duration;
use tokio::time::sleep;
use std::sync::Arc;
use std::marker::PhantomData;

#[tokio::main]
async fn main() {
    println!("=== 非同期コードにおけるライフタイムの課題 ===\n");
    
    lifetime_across_await().await;
    self_referential_futures().await;
    lifetime_elision_in_async().await;
    workarounds_and_patterns().await;
}

// .awaitを跨ぐライフタイム
async fn lifetime_across_await() {
    println!("--- .awaitを跨ぐライフタイム ---");
    
    // 問題のあるコード（コンパイルエラー）
    /*
    async fn process_borrowed<'a>(data: &'a str) -> &'a str {
        println!("処理開始: {}", data);
        sleep(Duration::from_millis(100)).await; // ここで借用が.awaitを跨ぐ
        data
    }
    */
    
    // なぜ問題か
    println!("  問題の理由:");
    println!("  1. async関数は内部的にステートマシンを生成");
    println!("  2. .awaitポイントで状態が保存される");
    println!("  3. 借用がFuture内に保存される必要がある");
    println!("  4. Futureのライフタイムが複雑になる");
    
    // 解決策1: 所有権を取る
    async fn process_owned(data: String) -> String {
        println!("  解決策1 - 所有権を取る: {}", data);
        sleep(Duration::from_millis(100)).await;
        format!("処理済み: {}", data)
    }
    
    let result = process_owned("データ".to_string()).await;
    println!("  結果: {}", result);
    
    // 解決策2: Arcを使用
    async fn process_shared(data: Arc<String>) -> String {
        println!("\n  解決策2 - Arc<T>を使用: {}", data);
        sleep(Duration::from_millis(100)).await;
        format!("処理済み: {}", data)
    }
    
    let shared_data = Arc::new("共有データ".to_string());
    let result = process_shared(shared_data.clone()).await;
    println!("  結果: {}", result);
    
    // 解決策3: スコープを制限
    async fn process_with_callback<F, R>(data: &str, callback: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        println!("\n  解決策3 - コールバックでスコープ制限");
        // .awaitの前に処理を完了
        let processed = format!("前処理: {}", data);
        let result = callback(&processed);
        
        // .awaitは借用の後
        sleep(Duration::from_millis(100)).await;
        result
    }
    
    let result = process_with_callback("データ", |s| s.to_uppercase()).await;
    println!("  結果: {}", result);
}

// 自己参照するFuture
async fn self_referential_futures() {
    println!("\n--- 自己参照するFuture ---");
    
    // 自己参照の問題
    struct ProblematicFuture<'a> {
        data: String,
        reference: Option<&'a str>, // dataへの参照を保持したい
    }
    
    // これは安全に実装できない
    /*
    impl<'a> ProblematicFuture<'a> {
        fn new(data: String) -> Self {
            let mut future = ProblematicFuture {
                data,
                reference: None,
            };
            future.reference = Some(&future.data); // 自己参照！
            future
        }
    }
    */
    
    println!("  自己参照の問題:");
    println!("  - Futureが移動すると参照が無効になる");
    println!("  - Pinが必要になる理由");
    
    // 安全な実装方法
    use pin_project::pin_project;
    
    #[pin_project]
    struct SafeFuture {
        #[pin]
        data: String,
        processed: bool,
    }
    
    impl SafeFuture {
        fn new(data: String) -> Self {
            SafeFuture {
                data,
                processed: false,
            }
        }
    }
    
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    
    impl Future for SafeFuture {
        type Output = String;
        
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.project();

            if !*this.processed {
                *this.processed = true;
                // Pending を返すなら自分で wake しないと、executor は二度と poll しない
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(format!("安全に処理: {}", this.data))
            }
        }
    }
    
    let future = SafeFuture::new("安全なデータ".to_string());
    let result = future.await;
    println!("  {}", result);
}

// async関数でのライフタイム省略
async fn lifetime_elision_in_async() {
    println!("\n--- async関数でのライフタイム省略 ---");
    
    // 通常の関数でのライフタイム省略
    fn sync_first(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    
    // async関数では省略が効かない場合がある
    // これはコンパイルエラー
    /*
    async fn async_first(s: &str) -> &str {
        sleep(Duration::from_millis(10)).await;
        s.split_whitespace().next().unwrap_or("")
    }
    */
    
    // 明示的なライフタイムが必要
    async fn async_first_explicit<'a>(s: &'a str) -> &'a str {
        // ただし、.awaitを使わない場合のみ
        s.split_whitespace().next().unwrap_or("")
    }
    
    println!("  ライフタイム省略規則:");
    println!("  1. 通常の関数では多くの場合省略可能");
    println!("  2. async関数では制限がある");
    println!("  3. .awaitを含む場合は特に注意");
    
    let text = "hello world";
    let first_sync = sync_first(text);
    let first_async = async_first_explicit(text).await;
    
    println!("  同期版: {}", first_sync);
    println!("  非同期版: {}", first_async);
}

// 回避策とパターン
async fn workarounds_and_patterns() {
    println!("\n--- 回避策とパターン ---");
    
    // パターン1: バッファリング
    println!("  パターン1: バッファリング");
    
    struct BufferedProcessor {
        buffer: Vec<String>,
    }
    
    impl BufferedProcessor {
        fn new() -> Self {
            BufferedProcessor {
                buffer: Vec::new(),
            }
        }
        
        async fn process(&mut self, data: &str) {
            // データをコピーしてバッファに保存
            self.buffer.push(data.to_string());
            
            // 非同期処理
            sleep(Duration::from_millis(50)).await;
            
            // バッファから処理
            if let Some(item) = self.buffer.pop() {
                println!("    処理: {}", item);
            }
        }
    }
    
    let mut processor = BufferedProcessor::new();
    processor.process("データ1").await;
    processor.process("データ2").await;
    
    // パターン2: チャンネルを使用
    println!("\n  パターン2: チャンネルを使用");
    
    use tokio::sync::mpsc;
    
    let (tx, mut rx) = mpsc::channel(100);
    
    // プロデューサー
    tokio::spawn(async move {
        let data = vec!["A", "B", "C"];
        for item in data {
            tx.send(item.to_string()).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
    });
    
    // コンシューマー
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            println!("    受信: {}", item);
        }
    });
    
    sleep(Duration::from_millis(200)).await;
    
    // パターン3: ステート分離
    println!("\n  パターン3: ステート分離");
    
    struct ProcessorState {
        count: usize,
    }
    
    struct AsyncProcessor<'a> {
        state: &'a mut ProcessorState,
    }
    
    impl<'a> AsyncProcessor<'a> {
        async fn process(&mut self, _data: &str) {
            self.state.count += 1;
            println!("    処理カウント: {}", self.state.count);
            
            // 非同期操作
            sleep(Duration::from_millis(50)).await;
        }
    }
    
    let mut state = ProcessorState { count: 0 };
    {
        let mut processor = AsyncProcessor { state: &mut state };
        processor.process("データ").await;
        processor.process("データ2").await;
    }
    println!("    最終カウント: {}", state.count);
    
    // パターン4: ライフタイムを持つトレイト
    println!("\n  パターン4: ライフタイムを持つトレイト");
    
    // Rust 1.75 以降はトレイト内の async fn をそのまま書ける
    trait LifetimeProcessor<'a> {
        type Output;
        async fn process(&self, data: &'a str) -> Self::Output;
    }
    
    struct MyProcessor<'a> {
        prefix: &'a str,
    }
    
    impl<'a> LifetimeProcessor<'a> for MyProcessor<'a> {
        type Output = String;
        
        async fn process(&self, data: &'a str) -> String {
            format!("{}: {}", self.prefix, data)
        }
    }
    
    let processor = MyProcessor { prefix: "処理" };
    let result = processor.process("テストデータ").await;
    println!("    {}", result);
    
    // ベストプラクティス
    println!("\n--- ベストプラクティス ---");
    println!("  1. 可能な限り所有権を使用");
    println!("  2. 必要に応じてArc/Rcを活用");
    println!("  3. .awaitを跨ぐ借用は避ける");
    println!("  4. チャンネルでデータを分離");
    println!("  5. ステートとロジックを分離");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    // 各デモが完走すること（Pending のまま wake されないなどのハングも検出する）
    #[tokio::test]
    async fn lifetime_demos_complete() {
        tokio::time::timeout(Duration::from_secs(30), super::self_referential_futures())
            .await
            .expect("self_referential_futures がハングした");
        tokio::time::timeout(Duration::from_secs(30), super::workarounds_and_patterns())
            .await
            .expect("workarounds_and_patterns がハングした");
    }
}
