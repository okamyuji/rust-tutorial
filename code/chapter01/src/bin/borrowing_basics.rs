// src/bin/borrowing_basics.rs

fn main() {
    println!("=== 借用チェッカーの動作原理 ===\n");

    basic_borrowing_rules();
    mutable_borrowing_rules();
    nll_demonstration();
}

fn basic_borrowing_rules() {
    println!("--- 不変借用のルール ---");

    let data = vec![1, 2, 3, 4, 5];

    // 複数の不変借用は同時に可能
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;

    println!("r1[0] = {}", r1[0]);
    println!("r2の長さ = {}", r2.len());
    println!("r3の合計 = {}", r3.iter().sum::<i32>());

    // 不変借用は元の値の読み取りを妨げない
    println!("元のdata = {:?}", data);
}

fn mutable_borrowing_rules() {
    println!("\n--- 可変借用のルール ---");

    let mut data = vec![1, 2, 3];
    println!("変更前: {:?}", data);

    // 可変借用は排他的（同時に1つだけ）
    let r_mut = &mut data;
    r_mut.push(4);
    r_mut[0] = 10;

    // この時点でr_mutの使用が終わる
    println!("変更後: {:?}", data);

    // 可変借用の後は再び借用可能
    let r_immut = &data;
    println!("不変借用で読み取り: {:?}", r_immut);
}

fn nll_demonstration() {
    println!("\n--- Non-Lexical Lifetimes (NLL) ---");

    let mut vec = vec![1, 2, 3];

    let r1 = &vec[0]; // 不変借用開始
    println!("最初の要素: {}", r1);
    // r1の最後の使用

    // NLLにより、r1の使用が終わったので可変借用が可能
    let r_mut = &mut vec;
    r_mut.push(4);

    println!("更新後のベクタ: {:?}", vec);

    // 古いコンパイラではr1のスコープがブロック終了まで続いていたが、
    // NLLでは最後の使用箇所で借用が終了する
}
