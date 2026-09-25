// src/bin/mutex_example.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Mutex: スレッドセーフな内部可変性 ===\n");

    basic_mutex_example();
    deadlock_prevention();
}

fn basic_mutex_example() {
    println!("--- 基本的なMutexの使用 ---");

    #[derive(Debug)]
    struct SharedState {
        counter: Mutex<u32>,
        data: Mutex<Vec<String>>,
    }

    let state = Arc::new(SharedState {
        counter: Mutex::new(0),
        data: Mutex::new(Vec::new()),
    });

    let mut handles = vec![];

    // 10個のスレッドで共有状態を更新
    for i in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            // カウンターをインクリメント
            let mut counter = state_clone.counter.lock().unwrap();
            *counter += 1;
            let count_value = *counter;
            drop(counter); // 明示的にロックを解放

            // データを追加
            let mut data = state_clone.data.lock().unwrap();
            data.push(format!("スレッド{}が追加（カウント: {}）", i, count_value));

            // 少し待機（実際の処理をシミュレート）
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }

    // すべてのスレッドの完了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // 結果を表示
    println!("最終カウント: {}", *state.counter.lock().unwrap());
    println!("データ内容:");
    let data = state.data.lock().unwrap();
    for item in data.iter() {
        println!("  {}", item);
    }
}

fn deadlock_prevention() {
    println!("\n--- デッドロックの回避 ---");

    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));

    // 正しい順序でロックを取得
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        for _ in 0..1000 {
            // 常に同じ順序でロック
            let _lock1 = r1.lock().unwrap();
            let _lock2 = r2.lock().unwrap();
            // 処理...
        }
        println!("スレッド1完了");
    });

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        for _ in 0..1000 {
            // 同じ順序でロック（デッドロック回避）
            let _lock1 = r1.lock().unwrap();
            let _lock2 = r2.lock().unwrap();
            // 処理...
        }
        println!("スレッド2完了");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("デッドロックなしで完了！");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
