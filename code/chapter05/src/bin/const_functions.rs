//! const関数とコンパイル時計算
//! 
//! コンパイル時に実行される関数と定数評価の仕組みを示します。

// 基本的なconst関数
const fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 再帰的なconst関数
const fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

// フィボナッチ数列のconst関数
const fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 最大値を求めるconst関数
const fn const_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// 配列操作のconst関数
const fn array_sum<const N: usize>(arr: &[i32; N]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < N {
        sum += arr[i];
        i += 1;
    }
    sum
}

// const genericsを使った構造体
struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    const fn new() -> Self {
        Self {
            data: [[0.0; COLS]; ROWS],
        }
    }

    // const関数での配列初期化
    const fn identity() -> Self
    where
        [(); ROWS]: Sized,
        [(); COLS]: Sized,
    {
        let mut matrix = Self::new();
        let mut i = 0;
        while i < ROWS && i < COLS {
            matrix.data[i][i] = 1.0;
            i += 1;
        }
        matrix
    }
}

// コンパイル時の文字列操作
const fn const_str_len(s: &str) -> usize {
    s.len()
}

// const関数でのOption処理
const fn const_unwrap_or<T: Copy>(opt: Option<T>, default: T) -> T {
    match opt {
        Some(value) => value,
        None => default,
    }
}

// const関数での配列変換
const fn double_array<const N: usize>(arr: [i32; N]) -> [i32; N] {
    let mut result = arr;
    let mut i = 0;
    while i < N {
        result[i] *= 2;
        i += 1;
    }
    result
}

// const contextでの型レベル計算
trait TypeLevel {
    const VALUE: usize;
}

struct Zero;
impl TypeLevel for Zero {
    const VALUE: usize = 0;
}

struct Succ<T: TypeLevel> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: TypeLevel> TypeLevel for Succ<T> {
    const VALUE: usize = T::VALUE + 1;
}

// const評価可能なassert
const fn const_assert(condition: bool) {
    if !condition {
        panic!("Assertion failed at compile time");
    }
}

// const関数でのループ処理
const fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    let mut i = 0;
    while i < exp {
        result *= base;
        i += 1;
    }
    result
}

// const関数での配列検索
const fn find_in_array<const N: usize>(arr: &[i32; N], target: i32) -> Option<usize> {
    let mut i = 0;
    while i < N {
        if arr[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn main() {
    println!("=== const関数とコンパイル時計算 ===\n");

    // コンパイル時に計算される定数
    const SUM: i32 = add(10, 20);
    const FACT: u32 = factorial(5);
    const FIB: u32 = fibonacci(10);

    println!("--- コンパイル時定数 ---");
    println!("10 + 20 = {}", SUM);
    println!("5! = {}", FACT);
    println!("フィボナッチ(10) = {}", FIB);

    // const関数の実行時使用
    println!("\n--- 実行時のconst関数 ---");
    let runtime_sum = add(15, 25);
    println!("15 + 25 = {}", runtime_sum);

    // 配列操作
    const ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
    const ARRAY_SUM: i32 = array_sum(&ARRAY);
    println!("\n--- 配列操作 ---");
    println!("配列の合計: {}", ARRAY_SUM);

    // const generics
    println!("\n--- const generics ---");
    const IDENTITY: Matrix<3, 3> = Matrix::identity();
    println!("3x3単位行列が作成されました");

    // 文字列の長さ
    const HELLO_LEN: usize = const_str_len("Hello, World!");
    println!("\n--- 文字列操作 ---");
    println!("\"Hello, World!\"の長さ: {}", HELLO_LEN);

    // Option処理
    const SOME_VALUE: i32 = const_unwrap_or(Some(42), 0);
    const NONE_VALUE: i32 = const_unwrap_or(None, 100);
    println!("\n--- Option処理 ---");
    println!("Some(42)のunwrap_or(0): {}", SOME_VALUE);
    println!("Noneのunwrap_or(100): {}", NONE_VALUE);

    // 配列変換
    const ORIGINAL: [i32; 4] = [1, 2, 3, 4];
    const DOUBLED: [i32; 4] = double_array(ORIGINAL);
    println!("\n--- 配列変換 ---");
    println!("元の配列: {:?}", ORIGINAL);
    println!("2倍の配列: {:?}", DOUBLED);

    // 型レベル計算
    println!("\n--- 型レベル計算 ---");
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type Three = Succ<Two>;
    println!("Three::VALUE = {}", Three::VALUE);

    // コンパイル時assert
    const _: () = const_assert(std::mem::size_of::<u64>() == 8);
    println!("\n--- コンパイル時assert ---");
    println!("size_of::<u64>() == 8 がコンパイル時に検証されました");

    // べき乗計算
    const POWER: i32 = power(2, 10);
    println!("\n--- べき乗計算 ---");
    println!("2^10 = {}", POWER);

    // 配列検索
    const SEARCH_ARRAY: [i32; 6] = [10, 20, 30, 40, 50, 60];
    const FOUND: Option<usize> = find_in_array(&SEARCH_ARRAY, 30);
    println!("\n--- 配列検索 ---");
    println!("30の位置: {:?}", FOUND);

    // const評価の制限
    println!("\n--- const評価の制限 ---");
    println!("現在のRustでは、以下はconst関数で使用できません：");
    println!("- ヒープアロケーション（Vec、String等）");
    println!("- 関数ポインタの呼び出し");
    println!("- トレイトオブジェクト");
    println!("- 非決定的な操作");

    println!("\nconst関数とコンパイル時計算をマスターしました！");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
