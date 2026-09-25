// src/bin/future_and_pin.rs

use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    println!("=== Futureトレイトとピン留め（Pinning） ===\n");

    future_trait_basics().await;
    pin_demonstration().await;
    self_referential_demo().await;
    pin_project_demo().await;
}

// Futureトレイトの基本
async fn future_trait_basics() {
    println!("--- Futureトレイトの基本 ---");

    // シンプルなFutureの実装
    struct SimpleFuture {
        value: i32,
        ready: bool,
    }

    impl Future for SimpleFuture {
        type Output = i32;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.ready {
                println!("  SimpleFuture: Ready({})", self.value);
                Poll::Ready(self.value)
            } else {
                self.ready = true;
                println!("  SimpleFuture: Pending -> wake()を呼び出し");
                _cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let future = SimpleFuture {
        value: 42,
        ready: false,
    };
    let result = future.await;
    println!("  結果: {}\n", result);

    // より複雑なFuture
    struct TimerFuture {
        deadline: Instant,
        waker_sender: Option<std::sync::mpsc::Sender<Waker>>,
    }

    impl TimerFuture {
        fn new(duration: Duration) -> Self {
            let deadline = Instant::now() + duration;

            TimerFuture {
                deadline,
                waker_sender: None,
            }
        }
    }

    impl Future for TimerFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if Instant::now() >= self.deadline {
                println!("  TimerFuture: タイマー完了");
                Poll::Ready(())
            } else {
                // 実際のタイマー実装では、システムタイマーにWakerを登録する
                println!("  TimerFuture: まだ時間が来ていない");

                // 簡略化のため、すぐにwakeを呼ぶ
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    println!("  100msタイマー開始");
    let timer = TimerFuture::new(Duration::from_millis(100));
    timer.await;
    println!("  タイマー完了");
}

// Pinのデモンストレーション
async fn pin_demonstration() {
    println!("\n--- Pinの必要性 ---");

    // 移動可能な構造体
    #[derive(Debug)]
    struct Movable {
        data: String,
    }

    let movable = Movable {
        data: "移動可能".to_string(),
    };
    let ptr1 = &movable as *const Movable;
    println!("  移動前のアドレス: {:p}", ptr1);

    // 移動
    let moved = movable;
    let ptr2 = &moved as *const Movable;
    println!("  移動後のアドレス: {:p}", ptr2);
    println!("  アドレスが変わった: {}", ptr1 != ptr2);

    // Pinされた値
    println!("\n  Pinされた値:");
    let pinned = Box::pin(Movable {
        data: "固定された".to_string(),
    });
    println!("  Pinned<Box<Movable>>: {:?}", pinned);

    // Pin APIの使用
    let mut pinned_value = Box::pin(42);

    // Pin::as_mut()で可変参照を取得
    let pinned_ref: Pin<&mut i32> = pinned_value.as_mut();

    // get_mut()は安全に可変参照を取得（Unpinの場合のみ）
    *pinned_ref.get_mut() = 100;
    println!("  更新された値: {}", *pinned_value);

    // Pin::new()は値がUnpinの場合のみ使用可能
    let unpinned = 42;
    let pinned_ref = Pin::new(&unpinned);
    println!("  Unpinな値のPin: {}", *pinned_ref);
}

// 自己参照構造体のデモ
async fn self_referential_demo() {
    println!("\n--- 自己参照構造体 ---");

    // 問題のある自己参照構造体の例（実際にはコンパイルできない）
    /*
    struct SelfReferential {
        data: String,
        ptr: *const String,  // dataを指す
    }

    impl SelfReferential {
        fn new(data: String) -> Self {
            let mut sr = SelfReferential {
                data,
                ptr: std::ptr::null(),
            };
            sr.ptr = &sr.data;  // 自己参照！
            sr
        }
    }
    */

    // Pinを使った安全な実装
    #[pin_project]
    struct SafeSelfReferential {
        #[pin]
        data: String,
        ptr: *const String,
    }

    impl SafeSelfReferential {
        fn new(data: String) -> Pin<Box<Self>> {
            let mut boxed = Box::new(SafeSelfReferential {
                data,
                ptr: std::ptr::null(),
            });

            let ptr = &boxed.data as *const String;
            boxed.ptr = ptr;

            Box::into_pin(boxed)
        }

        fn get_data(&self) -> &str {
            // ptrが常にdataを正しく指していることが保証される
            unsafe { &*self.ptr }
        }
    }

    let pinned = SafeSelfReferential::new("自己参照データ".to_string());
    println!("  自己参照経由のデータ: {}", pinned.get_data());

    // 非同期コンテキストでの自己参照
    struct AsyncSelfRef {
        data: Vec<u8>,
        // .awaitを跨いで借用を保持
        processing: Option<Box<dyn Future<Output = Vec<u8>> + Send>>,
    }

    println!("\n  非同期コンテキストでの自己参照:");
    println!("  - .awaitポイントで状態が保存される");
    println!("  - 借用が.awaitを跨ぐ場合、Pinが必要");
    println!("  - async関数は自動的にPinを処理");
}

// pin-projectクレートの使用
async fn pin_project_demo() {
    println!("\n--- pin-projectの使用 ---");

    // pin-projectを使った構造体
    #[pin_project]
    struct MyFuture<F> {
        #[pin]
        future: F,
        extra_data: String,
    }

    impl<F: Future> Future for MyFuture<F> {
        type Output = (F::Output, String);

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.project();

            match this.future.poll(cx) {
                Poll::Ready(value) => {
                    println!("  MyFuture: 内部Futureが完了");
                    Poll::Ready((value, this.extra_data.clone()))
                }
                Poll::Pending => {
                    println!("  MyFuture: まだ待機中");
                    Poll::Pending
                }
            }
        }
    }

    // 使用例
    let inner_future = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "内部の結果"
    };

    let my_future = MyFuture {
        future: inner_future,
        extra_data: "追加データ".to_string(),
    };

    let (result, extra) = my_future.await;
    println!("  結果: {}, 追加: {}", result, extra);

    // 複雑な例：ストリームアダプター
    #[pin_project]
    struct StreamAdapter<S> {
        #[pin]
        stream: S,
        count: usize,
    }

    use futures::stream::{Stream, StreamExt};

    impl<S: Stream> Stream for StreamAdapter<S> {
        type Item = (usize, S::Item);

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let mut this = self.project();

            match this.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(item)) => {
                    let count = *this.count;
                    *this.count += 1;
                    Poll::Ready(Some((count, item)))
                }
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        }
    }

    // ストリームの使用
    let stream = futures::stream::iter(vec![1, 2, 3]);
    let mut adapted = StreamAdapter { stream, count: 0 };

    println!("\n  ストリームアダプターの使用:");
    while let Some((index, value)) = adapted.next().await {
        println!("    [{}] = {}", index, value);
    }

    // Pinの実践的なガイドライン
    println!("\n--- Pinの実践的なガイドライン ---");
    println!("  1. ほとんどの型はUnpinを実装（自動的に）");
    println!("  2. async関数内では通常Pinを意識する必要なし");
    println!("  3. カスタムFuture実装時にPinが重要");
    println!("  4. 自己参照構造体にはPinが必須");
    println!("  5. pin-projectクレートで安全に実装");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
