// src/bin/variance_examples.rs

use std::cell::Cell;
use std::marker::PhantomData;

fn main() {
    println!("=== ライフタイムサブタイピングと変性 ===\n");

    demonstrate_covariance();
    demonstrate_contravariance();
    demonstrate_invariance();
    practical_variance_examples();
    phantom_data_examples();
}

fn demonstrate_covariance() {
    println!("--- 共変（Covariance） ---");
    println!("&'a T は 'a に関して共変");

    // 'staticは任意のライフタイムのサブタイプ
    let static_str: &'static str = "静的文字列";
    let local_str: &str = static_str; // OK: 'static → 'a
    println!("静的文字列を短いライフタイムに割り当て: {}", local_str);

    // 実例：長いライフタイムから短いライフタイムへ
    covariance_in_practice();

    // 構造体での共変
    demonstrate_struct_covariance();
}

fn covariance_in_practice() {
    // 外側のスコープ - より長いライフタイム 'a
    let longer_string = String::from("長寿命の文字列");
    let longer_ref: &str = &longer_string;

    {
        // 内側のスコープ - より短いライフタイム 'b
        let result = accept_shorter_lifetime(longer_ref);
        println!("短いライフタイムでの使用: {}", result);
    }

    // longer_refはまだ有効
    println!("元の参照: {}", longer_ref);
}

// 'aは関数のライフタイムより短くてもOK（共変）
fn accept_shorter_lifetime<'a>(s: &'a str) -> &'a str {
    s
}

fn demonstrate_struct_covariance() {
    println!("\n[構造体での共変]");

    // 共変な構造体
    let static_data = CovariantStruct {
        data: "static data",
    };

    // より短いライフタイムの変数に代入可能
    let local_data: CovariantStruct<'_> = static_data;
    println!("共変構造体のデータ: {}", local_data.data);
}

// &'a T を含む構造体は 'a に関して共変
struct CovariantStruct<'a> {
    data: &'a str,
}

fn demonstrate_contravariance() {
    println!("\n--- 反変（Contravariance） ---");
    println!("関数の引数位置でのライフタイムは反変");

    // 関数ポインタの反変性
    demonstrate_function_contravariance();

    // トレイトオブジェクトでの反変性
    demonstrate_trait_object_contravariance();
}

fn demonstrate_function_contravariance() {
    println!("\n[関数ポインタの反変性]");

    // より制限的な関数（短いライフタイムを要求）は、
    // より寛容な関数（長いライフタイムを受け入れる）として使える

    // 'static を受け入れる関数
    let _static_fn: fn(&'static str) -> usize = |s| s.len();

    // より短いライフタイムを受け入れる関数として使用可能
    // let shorter_fn: fn(&str) -> usize = static_fn; // これはできない（反変）

    // 正しい方向：短い → 長い
    let _general_fn: fn(&str) -> usize = |s| s.len();
    // let specific_fn: fn(&'static str) -> usize = general_fn; // OK（概念的に）

    println!("関数の反変性を確認");
}

fn demonstrate_trait_object_contravariance() {
    println!("\n[トレイトオブジェクトでの反変性]");

    // トレイトオブジェクトも関数と同様に反変
    let handler: Box<dyn Fn(&str)> = Box::new(|s| {
        println!("処理: {}", s);
    });

    handler("テストデータ");
}

fn demonstrate_invariance() {
    println!("\n--- 不変（Invariance） ---");
    println!("&'a mut T は 'a に関して不変");

    // 可変参照の不変性
    demonstrate_mutable_ref_invariance();

    // Cell/RefCellも不変
    demonstrate_cell_invariance();

    // 型パラメータの不変性
    demonstrate_type_parameter_invariance();
}

fn demonstrate_mutable_ref_invariance() {
    println!("\n[可変参照の不変性]");

    let mut data = String::from("可変データ");

    // 可変参照は正確に同じライフタイムでなければならない
    let mutable_ref = &mut data;

    // 以下のような変換はできない：
    // let shorter_ref: &mut str = mutable_ref; // エラー（異なるライフタイム）

    modify_data(mutable_ref);
    println!("変更後: {}", data);
}

fn modify_data(data: &mut String) {
    data.push_str(" - 変更済み");
}

fn demonstrate_cell_invariance() {
    println!("\n[Cell/RefCellの不変性]");

    // Cell<T>は不変
    let cell = InvariantCell {
        data: Cell::new(42),
        _marker: PhantomData,
    };

    println!("Cellの値: {}", cell.data.get());
    cell.data.set(100);
    println!("変更後: {}", cell.data.get());
}

struct InvariantCell<'a> {
    data: Cell<i32>,
    _marker: PhantomData<&'a ()>, // ライフタイムを追跡
}

fn demonstrate_type_parameter_invariance() {
    println!("\n[型パラメータの不変性]");

    // ジェネリック型パラメータは通常不変
    let container: Container<i32> = Container {
        value: 42,
        _marker: PhantomData,
    };

    println!("コンテナの値: {}", container.value);
}

struct Container<T> {
    value: i32,
    _marker: PhantomData<T>,
}

fn practical_variance_examples() {
    println!("\n--- 実践的な変性の例 ---");

    lifetime_subtyping_in_practice();
    variance_and_safety();
    complex_variance_scenarios();
}

fn lifetime_subtyping_in_practice() {
    println!("\n[ライフタイムサブタイピングの実践]");

    // より長いライフタイムはより短いライフタイムのサブタイプ
    let static_str: &'static str = "永続的";

    // 関数は短いライフタイムを期待できる
    fn process_string<'a>(s: &'a str) -> usize {
        s.len()
    }

    let len = process_string(static_str); // 'static → 'a
    println!("文字列の長さ: {}", len);

    // 複雑な例
    demonstrate_nested_lifetimes();
}

fn demonstrate_nested_lifetimes() {
    let outer = String::from("外側");

    {
        let inner = String::from("内側");

        // 両方の参照を受け取る
        let (outer_ref, inner_ref) = (&outer, &inner);

        // inner_refのライフタイムはこのスコープに制限される
        process_two_refs(outer_ref, inner_ref);
    }

    // outerはまだ有効
    println!("外側の値: {}", outer);
}

fn process_two_refs<'a, 'b>(longer: &'a str, shorter: &'b str)
where
    'a: 'b, // 'aは'bより長生きする
{
    println!("長い: {}, 短い: {}", longer, shorter);
}

fn variance_and_safety() {
    println!("\n[変性と安全性]");

    // なぜ&mut Tが不変なのか
    demonstrate_why_mut_is_invariant();

    // 変性による型安全性
    demonstrate_type_safety();
}

fn demonstrate_why_mut_is_invariant() {
    println!("\n可変参照が不変である理由:");

    // もし&mut Tが共変だったら...
    // let mut static_str: &'static str = "static";
    // let mut_ref: &mut &'static str = &mut static_str;
    //
    // // これが許されたら（実際は許されない）
    // let shorter_mut_ref: &mut &str = mut_ref;
    // *shorter_mut_ref = &String::from("temporary");
    //
    // // static_strが一時的な値を指すことになり、安全性が破られる

    println!("可変参照の不変性により、メモリ安全性が保証される");
}

fn demonstrate_type_safety() {
    // 型安全性の例
    let data = SafeContainer::new("安全なデータ");
    data.process();
}

struct SafeContainer<'a> {
    // 不変フィールド（共変）
    immutable: &'a str,
    // 可変フィールド（不変）
    mutable: Cell<&'a str>,
}

impl<'a> SafeContainer<'a> {
    fn new(data: &'a str) -> Self {
        SafeContainer {
            immutable: data,
            mutable: Cell::new(data),
        }
    }

    fn process(&self) {
        println!("不変データ: {}", self.immutable);
        println!("可変データ: {}", self.mutable.get());
    }
}

fn complex_variance_scenarios() {
    println!("\n[複雑な変性シナリオ]");

    // ネストした型の変性
    demonstrate_nested_variance();

    // 関数とライフタイムの組み合わせ
    demonstrate_function_lifetime_variance();
}

fn demonstrate_nested_variance() {
    // Option<&'a T> は 'a に関して共変
    let static_opt: Option<&'static str> = Some("static");
    let local_opt: Option<&str> = static_opt; // OK

    match local_opt {
        Some(s) => println!("Option内の値: {}", s),
        None => println!("None"),
    }

    // Vec<&'a T> も 'a に関して共変
    let static_vec: Vec<&'static str> = vec!["one", "two"];
    let local_vec: Vec<&str> = static_vec; // OK
    println!("Vec内の要素数: {}", local_vec.len());
}

fn demonstrate_function_lifetime_variance() {
    // 高階関数での変性
    let _processor: fn(&str) -> usize = |s| s.len();

    // Fn トレイトの変性
    let closure = |s: &str| s.to_uppercase();
    apply_closure(&closure, "test");
}

fn apply_closure<F>(f: &F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    f(s)
}

fn phantom_data_examples() {
    println!("\n--- PhantomDataと変性の制御 ---");

    // PhantomDataで変性を制御
    demonstrate_phantom_variance();
}

fn demonstrate_phantom_variance() {
    // 共変にする
    let _covariant = CovariantType::<&'static str>::new();

    // 反変にする
    let _contravariant = ContravariantType::<fn(&'static str)>::new();

    // 不変にする
    let _invariant = InvariantType::<&'static str>::new();

    println!("PhantomDataによる変性の制御完了");
}

// 共変型
struct CovariantType<T> {
    _marker: PhantomData<T>,
}

impl<T> CovariantType<T> {
    fn new() -> Self {
        CovariantType {
            _marker: PhantomData,
        }
    }
}

// 反変型
struct ContravariantType<T> {
    _marker: PhantomData<fn(T)>,
}

impl<T> ContravariantType<T> {
    fn new() -> Self {
        ContravariantType {
            _marker: PhantomData,
        }
    }
}

// 不変型
struct InvariantType<T> {
    _marker: PhantomData<fn(T) -> T>,
}

impl<T> InvariantType<T> {
    fn new() -> Self {
        InvariantType {
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
