//! 復習問題1：カスタムアサートマクロ
//!
//! 条件をチェックし、失敗時にカスタムメッセージを表示するマクロを実装します。

// カスタムアサートマクロの実装
macro_rules! assert_custom {
    // 基本形：条件のみ
    ($cond:expr) => {
        assert_custom!($cond, "Assertion failed: {}", stringify!($cond));
    };

    // メッセージ付き
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            panic!($($arg)*);
        }
    };
}

// 比較演算子付きアサート
macro_rules! assert_op {
    ($left:expr, $op:tt, $right:expr) => {
        assert_op!($left, $op, $right,
            "Assertion failed: {} {} {} (left: {:?}, right: {:?})",
            stringify!($left), stringify!($op), stringify!($right),
            $left, $right
        );
    };

    ($left:expr, ==, $right:expr, $($msg:tt)*) => {
        if !($left == $right) {
            panic!($($msg)*);
        }
    };

    ($left:expr, !=, $right:expr, $($msg:tt)*) => {
        if !($left != $right) {
            panic!($($msg)*);
        }
    };

    ($left:expr, <, $right:expr, $($msg:tt)*) => {
        if !($left < $right) {
            panic!($($msg)*);
        }
    };

    ($left:expr, >, $right:expr, $($msg:tt)*) => {
        if !($left > $right) {
            panic!($($msg)*);
        }
    };

    ($left:expr, <=, $right:expr, $($msg:tt)*) => {
        if !($left <= $right) {
            panic!($($msg)*);
        }
    };

    ($left:expr, >=, $right:expr, $($msg:tt)*) => {
        if !($left >= $right) {
            panic!($($msg)*);
        }
    };
}

// デバッグ情報付きアサート
macro_rules! assert_debug {
    ($cond:expr) => {
        if !$cond {
            panic!(
                "Assertion failed at {}:{}\n  Condition: {}\n  Value: {:?}",
                file!(),
                line!(),
                stringify!($cond),
                $cond
            );
        }
    };
}

// 範囲チェックアサート
macro_rules! assert_in_range {
    ($value:expr, $min:expr, $max:expr) => {
        assert_in_range!($value, $min, $max,
            "Value {} is not in range [{}, {}]", $value, $min, $max);
    };

    ($value:expr, $min:expr, $max:expr, $($msg:tt)*) => {
        if !($min..=$max).contains(&$value) {
            panic!($($msg)*);
        }
    };
}

// エラー型のアサート
macro_rules! assert_err {
    ($result:expr) => {
        match $result {
            Ok(val) => panic!("Expected Err, but got Ok({:?})", val),
            Err(_) => {}
        }
    };

    ($result:expr, $expected_err:pat) => {
        match $result {
            Ok(val) => panic!("Expected Err, but got Ok({:?})", val),
            Err($expected_err) => {}
            Err(e) => panic!(
                "Expected error pattern {}, but got {:?}",
                stringify!($expected_err),
                e
            ),
        }
    };
}

// 近似値アサート（浮動小数点用）
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        assert_approx_eq!($left, $right, 1e-6);
    };

    ($left:expr, $right:expr, $tolerance:expr) => {{
        // 型のない浮動小数点リテラルでは abs() の型が決まらないため f64 に揃える。各式の評価も1回に限る
        let (left, right, tolerance) = ($left as f64, $right as f64, $tolerance as f64);
        let diff = (left - right).abs();
        if diff > tolerance {
            panic!("Values are not approximately equal\n  left: {}\n  right: {}\n  difference: {}\n  tolerance: {}",
                left, right, diff, tolerance);
        }
    }};
}

// パニックをキャッチしてテスト
macro_rules! assert_panics {
    ($body:expr) => {
        assert_panics!($body, "Expected panic, but no panic occurred");
    };

    ($body:expr, $msg:expr) => {
        let result = std::panic::catch_unwind(|| $body);
        if result.is_ok() {
            panic!($msg);
        }
    };
}

// コレクションのアサート
macro_rules! assert_contains {
    ($collection:expr, $item:expr) => {
        if !$collection.contains(&$item) {
            panic!("Collection does not contain {:?}", $item);
        }
    };
}

fn main() {
    println!("=== 復習問題1：カスタムアサートマクロ ===\n");

    // 基本的なアサート
    println!("--- 基本的なアサート ---");
    assert_custom!(true);
    let two = 2;
    assert_custom!(two + two == 4, "数学が壊れています！");
    println!("✓ 基本的なアサートが成功");

    // 比較演算子アサート
    println!("\n--- 比較演算子アサート ---");
    assert_op!(5, >, 3);
    assert_op!(10, ==, 10);
    assert_op!(7, !=, 8);
    println!("✓ 比較演算子アサートが成功");

    // デバッグ情報付きアサート
    println!("\n--- デバッグ情報付きアサート ---");
    let x = 42;
    assert_debug!(x > 0);
    println!("✓ デバッグアサートが成功");

    // 範囲チェック
    println!("\n--- 範囲チェック ---");
    let score = 85;
    assert_in_range!(score, 0, 100);
    assert_in_range!(score, 80, 90, "スコア{}は期待範囲外です", score);
    println!("✓ 範囲チェックが成功");

    // エラー型のアサート
    println!("\n--- エラー型アサート ---");
    let result: Result<i32, &str> = Err("エラーが発生");
    assert_err!(result);

    let specific_err: Result<i32, &str> = Err("特定のエラー");
    assert_err!(specific_err, "特定のエラー");
    println!("✓ エラーアサートが成功");

    // 近似値アサート
    println!("\n--- 近似値アサート ---");
    let third = 1.0 / 3.0;
    let approx_third = 0.3333;
    assert_approx_eq!(third, approx_third, 0.0001);
    println!("✓ 近似値アサートが成功");

    // パニックのテスト
    println!("\n--- パニックテスト ---");
    assert_panics!({
        panic!("意図的なパニック");
    });
    println!("✓ パニックアサートが成功");

    // コレクションのアサート
    println!("\n--- コレクションアサート ---");
    let vec = [1, 2, 3, 4, 5];
    assert_contains!(vec, 3);
    println!("✓ コレクションアサートが成功");

    // 失敗するアサートのデモ（コメントアウト）
    println!("\n--- 失敗例（コメントアウト） ---");
    // assert_custom!(false, "これは失敗します");
    // assert_op!(5, <, 3);
    // assert_in_range!(150, 0, 100);
    // assert_err!(Ok::<i32, &str>(42));
    // assert_approx_eq!(1.0, 2.0);
    // assert_panics!(println!("パニックしません"));
    // assert_contains!(vec![1, 2, 3], 4);

    println!("\nカスタムアサートマクロの実装完了！");
    println!("これらのマクロは、より表現力豊かなテストを書くのに役立ちます。");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_custom() {
        assert_custom!(true);
        assert_custom!(1 + 1 == 2, "Math is broken");
    }

    #[test]
    fn test_assert_op() {
        assert_op!(5, >, 3);
        assert_op!(10, ==, 10);
        assert_op!(7, !=, 8);
    }

    #[test]
    fn test_assert_in_range() {
        assert_in_range!(50, 0, 100);
        assert_in_range!(0.5, 0.0, 1.0);
    }

    #[test]
    fn test_assert_err() {
        let err_result: Result<(), &str> = Err("error");
        assert_err!(err_result);
    }

    #[test]
    fn test_assert_approx_eq() {
        // 差 1e-5 と同じ許容誤差だと浮動小数点誤差で上回るため、余裕のある値にする
        assert_approx_eq!(0.33333, 0.33334, 0.0001);
        assert_approx_eq!(1.0f32, 1.0f32 + 1e-7);
        assert_approx_eq!(2, 2);
    }

    #[test]
    #[should_panic(expected = "Values are not approximately equal")]
    fn test_assert_approx_eq_panics_outside_tolerance() {
        assert_approx_eq!(1.0, 1.1, 0.01);
    }

    #[test]
    #[should_panic(expected = "Values are not approximately equal")]
    fn test_assert_approx_eq_default_tolerance_is_strict() {
        assert_approx_eq!(1.0, 1.00001);
    }

    #[test]
    fn test_assert_panics() {
        assert_panics!(panic!("test panic"));
    }

    #[test]
    fn test_assert_contains() {
        let vec = ["apple", "banana", "orange"];
        assert_contains!(vec, "banana");
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
