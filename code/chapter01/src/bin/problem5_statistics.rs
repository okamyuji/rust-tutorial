// src/bin/problem5_statistics.rs
// 復習問題5: RefCellを使ったStatistics構造体の解答

use std::cell::RefCell;

#[derive(Debug, Clone)]
struct StatisticalData {
    count: u32,
    sum: f64,
    sum_of_squares: f64,
    min: Option<f64>,
    max: Option<f64>,
    values: Vec<f64>, // 中央値計算用
}

impl StatisticalData {
    fn new() -> Self {
        StatisticalData {
            count: 0,
            sum: 0.0,
            sum_of_squares: 0.0,
            min: None,
            max: None,
            values: Vec::new(),
        }
    }
}

struct AdvancedStatistics {
    data: RefCell<StatisticalData>,
    keep_values: bool, // 中央値計算のために値を保持するか
}

impl AdvancedStatistics {
    fn new(keep_values: bool) -> Self {
        AdvancedStatistics {
            data: RefCell::new(StatisticalData::new()),
            keep_values,
        }
    }
    
    fn record_value(&self, value: f64) {
        let mut data = self.data.borrow_mut();
        
        data.count += 1;
        data.sum += value;
        data.sum_of_squares += value * value;
        
        // 最小値・最大値の更新
        data.min = Some(data.min.map_or(value, |min| min.min(value)));
        data.max = Some(data.max.map_or(value, |max| max.max(value)));
        
        // 値の保持（中央値計算用）
        if self.keep_values {
            data.values.push(value);
        }
    }
    
    fn get_count(&self) -> u32 {
        self.data.borrow().count
    }
    
    fn get_mean(&self) -> Option<f64> {
        let data = self.data.borrow();
        if data.count > 0 {
            Some(data.sum / data.count as f64)
        } else {
            None
        }
    }
    
    fn get_variance(&self) -> Option<f64> {
        let data = self.data.borrow();
        if data.count > 1 {
            let mean = data.sum / data.count as f64;
            let variance = (data.sum_of_squares - data.count as f64 * mean * mean) / (data.count - 1) as f64;
            Some(variance)
        } else {
            None
        }
    }
    
    fn get_standard_deviation(&self) -> Option<f64> {
        self.get_variance().map(|v| v.sqrt())
    }
    
    fn get_min(&self) -> Option<f64> {
        self.data.borrow().min
    }
    
    fn get_max(&self) -> Option<f64> {
        self.data.borrow().max
    }
    
    fn get_median(&self) -> Option<f64> {
        if !self.keep_values {
            return None;
        }
        
        let mut data = self.data.borrow_mut();
        if data.values.is_empty() {
            return None;
        }
        
        data.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = data.values.len();
        
        if len.is_multiple_of(2) {
            Some((data.values[len / 2 - 1] + data.values[len / 2]) / 2.0)
        } else {
            Some(data.values[len / 2])
        }
    }
    
    fn get_summary(&self) -> String {
        let data = self.data.borrow();
        
        if data.count == 0 {
            return "No data recorded".to_string();
        }
        
        let mean = self.get_mean().unwrap_or(0.0);
        let std_dev = self.get_standard_deviation().unwrap_or(0.0);
        let min = self.get_min().unwrap_or(0.0);
        let max = self.get_max().unwrap_or(0.0);
        
        format!(
            "Statistics Summary:\n  Count: {}\n  Mean: {:.2}\n  Std Dev: {:.2}\n  Min: {:.2}\n  Max: {:.2}",
            data.count, mean, std_dev, min, max
        )
    }
    
    fn reset(&self) {
        *self.data.borrow_mut() = StatisticalData::new();
    }
}

fn main() {
    println!("=== 復習問題5: RefCellを使ったStatistics構造体 ===\n");
    
    // 基本的な統計構造体
    basic_statistics_example();
    
    // より高度な統計機能
    advanced_statistics_example();
    
    // エラーハンドリング
    error_handling_example();
    
    // マルチスレッド版の言及
    thread_safe_version_note();
}

// 基本的なStatistics実装
fn basic_statistics_example() {
    
    struct Statistics {
        counter: RefCell<i32>,
        total: RefCell<i64>,
    }
    
    impl Statistics {
        fn new() -> Self {
            Statistics {
                counter: RefCell::new(0),
                total: RefCell::new(0),
            }
        }
        
        // 不変参照でも内部を更新可能
        fn record_value(&self, value: i32) {
            *self.counter.borrow_mut() += 1;
            *self.total.borrow_mut() += value as i64;
        }
        
        fn get_count(&self) -> i32 {
            *self.counter.borrow()
        }
        
        fn get_total(&self) -> i64 {
            *self.total.borrow()
        }
        
        fn get_average(&self) -> f64 {
            let count = *self.counter.borrow();
            let total = *self.total.borrow();
            
            if count > 0 {
                total as f64 / count as f64
            } else {
                0.0
            }
        }
        
        fn reset(&self) {
            *self.counter.borrow_mut() = 0;
            *self.total.borrow_mut() = 0;
        }
    }
    
    println!("【基本的なStatistics実装】");
    let stats = Statistics::new();
    
    // 不変参照でも更新可能
    stats.record_value(10);
    stats.record_value(20);
    stats.record_value(30);
    
    println!("データ記録: 10, 20, 30");
    println!("Count: {}", stats.get_count());         // 3
    println!("Total: {}", stats.get_total());         // 60
    println!("Average: {:.2}", stats.get_average());  // 20.00
    
    stats.record_value(40);
    println!("\n追加データ: 40");
    println!("Count: {}", stats.get_count());         // 4
    println!("Total: {}", stats.get_total());         // 100
    println!("Average: {:.2}", stats.get_average());  // 25.00
    
    stats.reset();
    println!("\nリセット後:");
    println!("Count: {}", stats.get_count());         // 0
    println!("Average: {:.2}", stats.get_average());  // 0.00
    
    println!("\n✓ 不変参照(&self)のメソッドからでも内部状態を変更可能");
    println!("✓ 内部可変性により、APIは不変に見えるが状態は変更可能");
}

// より高度な統計機能を持つ実装
fn advanced_statistics_example() {
    
    println!("\n【高度なStatistics実装】");
    let stats = AdvancedStatistics::new(true); // 中央値計算を有効化
    
    // テストデータの追加
    let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    for value in &test_data {
        stats.record_value(*value);
    }
    
    println!("テストデータ: {:?}", test_data);
    println!("\n{}", stats.get_summary());
    
    if let Some(median) = stats.get_median() {
        println!("  Median: {:.2}", median);
    }
    
    // 外れ値の追加
    stats.record_value(100.0);
    println!("\n外れ値追加後 (100.0):");
    println!("{}", stats.get_summary());
    
    if let Some(median) = stats.get_median() {
        println!("  Median: {:.2}", median);
    }
    
    // get_countメソッドの使用例
    println!("現在のデータ数: {}", stats.get_count());
    
    // resetメソッドの使用例
    stats.reset();
    println!("リセット後のデータ数: {}", stats.get_count());
}

// エラーハンドリングの例
fn error_handling_example() {
    
    struct SafeStatistics {
        data: RefCell<Vec<f64>>,
    }
    
    impl SafeStatistics {
        fn new() -> Self {
            SafeStatistics {
                data: RefCell::new(Vec::new()),
            }
        }
        
        // try_borrowを使った安全な操作
        fn try_record_value(&self, value: f64) -> Result<(), String> {
            match self.data.try_borrow_mut() {
                Ok(mut data) => {
                    data.push(value);
                    Ok(())
                },
                Err(_) => Err("データが他の場所で借用中です".to_string()),
            }
        }
        
        fn try_get_average(&self) -> Result<Option<f64>, String> {
            match self.data.try_borrow() {
                Ok(data) => {
                    if data.is_empty() {
                        Ok(None)
                    } else {
                        let sum: f64 = data.iter().sum();
                        Ok(Some(sum / data.len() as f64))
                    }
                },
                Err(_) => Err("データが他の場所で借用中です".to_string()),
            }
        }
        
        fn get_data_with_operation<F, R>(&self, operation: F) -> Result<R, String>
        where
            F: FnOnce(&Vec<f64>) -> R,
        {
            match self.data.try_borrow() {
                Ok(data) => Ok(operation(&data)),
                Err(_) => Err("データが他の場所で借用中です".to_string()),
            }
        }
    }
    
    println!("\n【エラーハンドリングの例】");
    let safe_stats = SafeStatistics::new();
    
    // 正常なケース
    match safe_stats.try_record_value(10.0) {
        Ok(()) => println!("値の記録成功: 10.0"),
        Err(e) => println!("エラー: {}", e),
    }
    
    match safe_stats.try_record_value(20.0) {
        Ok(()) => println!("値の記録成功: 20.0"),
        Err(e) => println!("エラー: {}", e),
    }
    
    // 平均値の取得
    match safe_stats.try_get_average() {
        Ok(Some(avg)) => println!("平均値: {:.2}", avg),
        Ok(None) => println!("データがありません"),
        Err(e) => println!("エラー: {}", e),
    }
    
    // 高次関数を使った操作
    let result = safe_stats.get_data_with_operation(|data| {
        (data.len(), data.iter().sum::<f64>(), data.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
    });
    
    match result {
        Ok((count, sum, max)) => {
            println!("操作結果 - Count: {}, Sum: {:.2}, Max: {:.2}", count, sum, max);
        },
        Err(e) => println!("操作エラー: {}", e),
    }
    
    println!("\n✓ try_borrow/try_borrow_mutでパニックを回避");
    println!("✓ Result型でエラーハンドリング");
    println!("✓ 高次関数による安全な操作");
}

// スレッドセーフ版の説明
fn thread_safe_version_note() {
    println!("\n【マルチスレッド環境での考慮事項】");
    println!("RefCell<T>はシングルスレッド専用です。");
    println!("マルチスレッド環境では以下を使用してください：");
    println!("  • Arc<Mutex<T>>: 単純な排他制御");
    println!("  • Arc<RwLock<T>>: 読み取り多数の場合");
    println!("  • 特化されたアトミック型: AtomicI32, AtomicU64など");
    
    println!("\n【簡単なスレッドセーフ版の例】");
    println!("```rust");
    println!("use std::sync::{{Arc, Mutex}};");
    println!();
    println!("struct ThreadSafeStatistics {{");
    println!("    counter: Arc<Mutex<i32>>,");
    println!("    total: Arc<Mutex<i64>>,");
    println!("}}");
    println!();
    println!("impl ThreadSafeStatistics {{");
    println!("    fn record_value(&self, value: i32) {{");
    println!("        *self.counter.lock().unwrap() += 1;");
    println!("        *self.total.lock().unwrap() += value as i64;");
    println!("    }}");
    println!("}}");
    println!("```");
    
    println!("\n【アトミック型を使った高性能版】");
    println!("```rust");
    println!("use std::sync::atomic::{{AtomicI32, AtomicI64, Ordering}};");
    println!();
    println!("struct AtomicStatistics {{");
    println!("    counter: AtomicI32,");
    println!("    total: AtomicI64,");
    println!("}}");
    println!();
    println!("impl AtomicStatistics {{");
    println!("    fn record_value(&self, value: i32) {{");
    println!("        self.counter.fetch_add(1, Ordering::Relaxed);");
    println!("        self.total.fetch_add(value as i64, Ordering::Relaxed);");
    println!("    }}");
    println!("}}");
    println!("```");
    
    println!("\n✓ 用途に応じて適切な同期プリミティブを選択");
    println!("✓ パフォーマンス要件と安全性のバランスを考慮");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}

#[cfg(test)]
mod advanced_statistics_tests {
    use super::AdvancedStatistics;

    fn stats_with(values: &[f64]) -> AdvancedStatistics {
        let stats = AdvancedStatistics::new(true);
        for &v in values {
            stats.record_value(v);
        }
        stats
    }

    const SAMPLE: [f64; 8] = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    #[test]
    fn median_of_odd_count_is_middle_value_after_sorting() {
        // 5個なら len / 2 = 2 と len % 2 = 1 が異なり、添字の取り違えを検出できる
        assert_eq!(stats_with(&[9.0, 1.0, 5.0, 7.0, 3.0]).get_median(), Some(5.0));
    }

    #[test]
    fn records_count_mean_min_and_max() {
        let stats = stats_with(&SAMPLE);

        assert_eq!(stats.get_count(), 8);
        assert_eq!(stats.get_mean(), Some(5.0));
        assert_eq!(stats.get_min(), Some(2.0));
        assert_eq!(stats.get_max(), Some(9.0));
    }

    #[test]
    fn computes_sample_variance_and_standard_deviation() {
        let stats = stats_with(&SAMPLE);
        let variance = 32.0 / 7.0;

        assert!((stats.get_variance().unwrap() - variance).abs() < 1e-12);
        assert!((stats.get_standard_deviation().unwrap() - variance.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn statistics_need_enough_values() {
        let empty = stats_with(&[]);
        assert_eq!(empty.get_count(), 0);
        assert_eq!(empty.get_mean(), None);
        assert_eq!(empty.get_min(), None);
        assert_eq!(empty.get_max(), None);

        let single = stats_with(&[3.0]);
        assert_eq!(single.get_mean(), Some(3.0));
        assert_eq!(single.get_variance(), None);
        assert_eq!(single.get_standard_deviation(), None);
    }

    #[test]
    fn summary_reports_values_or_no_data() {
        assert_eq!(stats_with(&[]).get_summary(), "No data recorded");
        assert_eq!(
            stats_with(&SAMPLE).get_summary(),
            format!(
                "Statistics Summary:\n  Count: 8\n  Mean: 5.00\n  Std Dev: {:.2}\n  Min: 2.00\n  Max: 9.00",
                (32.0f64 / 7.0).sqrt()
            )
        );
    }

    #[test]
    fn reset_clears_recorded_values() {
        let stats = stats_with(&SAMPLE);

        stats.reset();

        assert_eq!(stats.get_count(), 0);
        assert_eq!(stats.get_mean(), None);
        assert_eq!(stats.get_median(), None);
    }

    #[test]
    fn median_of_even_count_is_mean_of_middle_pair() {
        assert_eq!(stats_with(&[4.0, 1.0, 3.0, 2.0]).get_median(), Some(2.5));
    }

    #[test]
    fn median_is_none_when_empty_or_values_not_kept() {
        assert_eq!(stats_with(&[]).get_median(), None);

        let no_values = AdvancedStatistics::new(false);
        no_values.record_value(1.0);
        assert_eq!(no_values.get_median(), None);
    }
}
