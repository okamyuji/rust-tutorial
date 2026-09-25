// src/bin/problem1_custom_timer.rs
// 復習問題1: 独自のタイマーFutureの実装

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 非同期ランタイムの選択によって使い分け
    println!("=== 復習問題1: 独自のタイマーFuture ===\n");
    
    // Tokioランタイムで実行
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        run_timer_examples().await;
    });
}

async fn run_timer_examples() {
    // 基本的なタイマーテスト
    basic_timer_test().await;
    
    // キャンセル可能タイマーテスト
    cancellable_timer_test().await;
    
    // 高精度タイマーテスト
    precision_timer_test().await;
    
    // 複数タイマーの並行テスト
    concurrent_timers_test().await;
}

// タイマーの内部状態
#[derive(Debug)]
struct TimerState {
    completed: bool,
    waker: Option<Waker>,
    cancelled: bool,
}

impl TimerState {
    fn new() -> Self {
        TimerState {
            completed: false,
            waker: None,
            cancelled: false,
        }
    }
}

// 基本的なタイマーFuture
pub struct Timer {
    state: Arc<Mutex<TimerState>>,
    duration: Duration,
    start_time: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState::new()));
        let timer_state = Arc::clone(&state);
        let timer_duration = duration;
        
        // バックグラウンドスレッドでタイマーを実行
        thread::spawn(move || {
            thread::sleep(timer_duration);
            
            let mut state = timer_state.lock().unwrap();
            if !state.cancelled {
                state.completed = true;
                
                // Wakerが登録されていれば呼び出し
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
        });
        
        Timer {
            state,
            duration,
            start_time: Instant::now(),
        }
    }
    
    // 残り時間を取得
    pub fn remaining(&self) -> Duration {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            Duration::from_secs(0)
        } else {
            self.duration - elapsed
        }
    }
    
    // 経過時間を取得
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    // 完了しているかチェック
    pub fn is_completed(&self) -> bool {
        self.state.lock().unwrap().completed
    }
}

impl Future for Timer {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        if state.completed {
            Poll::Ready(())
        } else if state.cancelled {
            // キャンセルされた場合もReadyを返す（別の処理が必要なら専用の型を定義）
            Poll::Ready(())
        } else {
            // Wakerを登録して、タイマー完了時に起こしてもらう
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// キャンセル可能タイマー
pub struct CancellableTimer {
    state: Arc<Mutex<TimerState>>,
    cancel_handle: Arc<Mutex<bool>>,
    start_time: Instant,
    duration: Duration,
}

impl CancellableTimer {
    pub fn new(duration: Duration) -> (Self, CancelHandle) {
        let state = Arc::new(Mutex::new(TimerState::new()));
        let cancel_flag = Arc::new(Mutex::new(false));
        
        let timer_state = Arc::clone(&state);
        let timer_cancel = Arc::clone(&cancel_flag);
        
        // より細かい間隔でキャンセルチェック
        thread::spawn(move || {
            let sleep_interval = Duration::from_millis(10);
            let mut elapsed = Duration::from_secs(0);
            
            while elapsed < duration {
                thread::sleep(sleep_interval);
                elapsed += sleep_interval;
                
                // キャンセルチェック
                if *timer_cancel.lock().unwrap() {
                    let mut state = timer_state.lock().unwrap();
                    state.cancelled = true;
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                    return;
                }
            }
            
            // 正常完了
            let mut state = timer_state.lock().unwrap();
            if !state.cancelled {
                state.completed = true;
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            }
        });
        
        let timer = CancellableTimer {
            state,
            cancel_handle: Arc::clone(&cancel_flag),
            start_time: Instant::now(),
            duration,
        };
        
        let cancel_handle = CancelHandle { 
            cancel_flag,
        };
        
        (timer, cancel_handle)
    }
    
    pub fn remaining(&self) -> Duration {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            Duration::from_secs(0)
        } else {
            self.duration - elapsed
        }
    }
    
    pub fn is_cancelled(&self) -> bool {
        self.state.lock().unwrap().cancelled
    }
}

impl Future for CancellableTimer {
    type Output = Result<(), TimerCancelled>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        if state.cancelled {
            Poll::Ready(Err(TimerCancelled))
        } else if state.completed {
            Poll::Ready(Ok(()))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimerCancelled;

impl std::fmt::Display for TimerCancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timer was cancelled")
    }
}

impl std::error::Error for TimerCancelled {}

pub struct CancelHandle {
    cancel_flag: Arc<Mutex<bool>>,
}

impl CancelHandle {
    pub fn cancel(&self) {
        *self.cancel_flag.lock().unwrap() = true;
    }
    
    pub fn is_cancelled(&self) -> bool {
        *self.cancel_flag.lock().unwrap()
    }
}

// 高精度タイマー（より正確な時間管理）
pub struct PrecisionTimer {
    state: Arc<Mutex<TimerState>>,
    target_time: Instant,
    check_interval: Duration,
}

impl PrecisionTimer {
    pub fn new(duration: Duration) -> Self {
        Self::with_precision(duration, Duration::from_millis(1))
    }
    
    pub fn with_precision(duration: Duration, check_interval: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState::new()));
        let target_time = Instant::now() + duration;
        
        let timer_state = Arc::clone(&state);
        let timer_target = target_time;
        let timer_interval = check_interval;
        
        thread::spawn(move || {
            loop {
                thread::sleep(timer_interval);
                
                let mut state = timer_state.lock().unwrap();
                if state.cancelled {
                    return;
                }
                
                if Instant::now() >= timer_target {
                    state.completed = true;
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                    return;
                }
            }
        });
        
        PrecisionTimer {
            state,
            target_time,
            check_interval,
        }
    }
    
    pub fn remaining(&self) -> Duration {
        let now = Instant::now();
        if now >= self.target_time {
            Duration::from_secs(0)
        } else {
            self.target_time - now
        }
    }
    
    pub fn precision(&self) -> Duration {
        self.check_interval
    }
}

impl Future for PrecisionTimer {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        if state.completed || Instant::now() >= self.target_time {
            state.completed = true;
            Poll::Ready(())
        } else if state.cancelled {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// 複数タイマーの管理
pub struct TimerManager {
    timers: Vec<Box<dyn Future<Output = ()> + Send + Unpin>>,
}

impl Default for TimerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TimerManager {
    pub fn new() -> Self {
        TimerManager {
            timers: Vec::new(),
        }
    }
    
    pub fn add_timer(&mut self, timer: Timer) {
        self.timers.push(Box::new(timer));
    }
    
    pub async fn wait_all(self) {
        for timer in self.timers {
            timer.await;
        }
    }
    
    pub async fn wait_any(self) {
        if !self.timers.is_empty() {
            futures::future::select_all(self.timers).await;
        }
    }
}

// テスト関数群
async fn basic_timer_test() {
    println!("【基本的なタイマーテスト】");
    
    let start = Instant::now();
    let timer = Timer::new(Duration::from_millis(500));
    
    println!("  タイマー開始 (500ms)");
    println!("  残り時間: {:?}", timer.remaining());
    
    timer.await;
    
    let elapsed = start.elapsed();
    println!("  タイマー完了");
    println!("  実際の経過時間: {:?}", elapsed);
    println!("  精度: {:?}", elapsed.abs_diff(Duration::from_millis(500)));
}

async fn cancellable_timer_test() {
    println!("\n【キャンセル可能タイマーテスト】");
    
    // 正常完了のケース
    println!("  ケース1: 正常完了");
    let (timer, _cancel_handle) = CancellableTimer::new(Duration::from_millis(200));
    
    match timer.await {
        Ok(()) => println!("    タイマー正常完了"),
        Err(TimerCancelled) => println!("    タイマーキャンセル"),
    }
    
    // キャンセルのケース
    println!("  ケース2: キャンセル");
    let (timer, cancel_handle) = CancellableTimer::new(Duration::from_millis(1000));
    
    // 別のタスクでキャンセル
    let cancel_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        cancel_handle.cancel();
        println!("    タイマーをキャンセルしました");
    });
    
    match timer.await {
        Ok(()) => println!("    タイマー正常完了"),
        Err(TimerCancelled) => println!("    タイマーキャンセル完了"),
    }
    
    cancel_task.await.unwrap();
}

async fn precision_timer_test() {
    println!("\n【高精度タイマーテスト】");
    
    let timers = vec![
        ("標準精度", PrecisionTimer::new(Duration::from_millis(100))),
        ("高精度", PrecisionTimer::with_precision(Duration::from_millis(100), Duration::from_micros(500))),
        ("低精度", PrecisionTimer::with_precision(Duration::from_millis(100), Duration::from_millis(10))),
    ];
    
    for (name, timer) in timers {
        let start = Instant::now();
        println!("  {}タイマー開始 (100ms, 精度: {:?})", name, timer.precision());
        
        timer.await;
        
        let elapsed = start.elapsed();
        let accuracy = elapsed.abs_diff(Duration::from_millis(100));
        println!("    実際の時間: {:?}, 精度: {:?}", elapsed, accuracy);
    }
}

async fn concurrent_timers_test() {
    println!("\n【複数タイマーの並行テスト】");
    
    let timers = vec![
        Timer::new(Duration::from_millis(300)),
        Timer::new(Duration::from_millis(150)),
        Timer::new(Duration::from_millis(450)),
        Timer::new(Duration::from_millis(200)),
    ];
    
    println!("  4つのタイマーを並行実行");
    let start = Instant::now();
    
    // 全て並行で実行
    let results = futures::future::join_all(timers).await;
    
    let total_elapsed = start.elapsed();
    println!("  全タイマー完了: {:?}", total_elapsed);
    println!("  完了数: {}", results.len());
    
    // 最初の1つが完了するまで
    println!("\n  最初のタイマー完了まで測定");
    let start = Instant::now();
    
    let timers = vec![
        Timer::new(Duration::from_millis(300)),
        Timer::new(Duration::from_millis(150)),
        Timer::new(Duration::from_millis(450)),
    ];
    
    // 最初に完了するタイマーを待つ
    let (_, _index, _remaining) = futures::future::select_all(timers).await;
    
    let first_elapsed = start.elapsed();
    println!("  最初の完了時間: {:?}", first_elapsed);
    
    // タイマーマネージャーのテスト
    println!("\n  タイマーマネージャーテスト");
    let mut manager = TimerManager::new();
    manager.add_timer(Timer::new(Duration::from_millis(100)));
    manager.add_timer(Timer::new(Duration::from_millis(200)));
    manager.add_timer(Timer::new(Duration::from_millis(300)));
    
    let start = Instant::now();
    manager.wait_all().await;
    let manager_elapsed = start.elapsed();
    
    println!("  マネージャー完了時間: {:?}", manager_elapsed);
    
    println!("\n【タイマー実装のポイント】");
    println!("✓ 適切なWakerの使用とポーリング");
    println!("✓ Pin安全性の保証");
    println!("✓ キャンセレーション機能");
    println!("✓ 高精度タイミング制御");
    println!("✓ 並行タイマーの効率的管理");
    println!("✓ メモリ安全性とスレッド安全性");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
