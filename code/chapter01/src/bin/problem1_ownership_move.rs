// src/bin/problem1_ownership_move.rs
// 復習問題1: 所有権とムーブの解答

fn main() {
    println!("=== 復習問題1: 所有権とムーブ ===\n");

    // 元のコード（コンパイルエラー）
    println!("【問題のあるコード】");
    println!("let v = vec![1, 2, 3];");
    println!("let v2 = v;  // v の所有権が v2 に移動");
    println!("println!(\"{{:?}}\", v);  // エラー: v は使用不可");
    println!("println!(\"{{:?}}\", v2);");

    println!("\n【問題点の説明】");
    println!("・vの所有権がv2にムーブされているため、vは使用できなくなる");
    println!("・Rustの所有権システムでは、ヒープを使用する型の代入時にムーブが発生");
    println!("・これにより、ダングリングポインタや二重解放を防いでいる");

    // 修正方法1: クローンを使用
    println!("\n=== 修正方法1: クローンを使用 ===");
    {
        let v = vec![1, 2, 3];
        let v2 = v.clone(); // 深いコピーを作成
        println!("v: {:?}", v); // v は使用可能
        println!("v2: {:?}", v2);

        println!("解説: clone()で深いコピーを作成することで、両方の変数が使用可能");
    }

    // 修正方法2: 借用を使用
    println!("\n=== 修正方法2: 借用を使用 ===");
    {
        let v = vec![1, 2, 3];
        let v2 = &v; // 借用（参照）を使用
        println!("v: {:?}", v);
        println!("v2: {:?}", v2);

        println!("解説: 借用を使用することで所有権の移動を回避");
    }

    // 追加例: 関数との組み合わせ
    println!("\n=== 追加例: 関数での所有権の扱い ===");
    {
        let v = vec![1, 2, 3];

        // 所有権を渡す場合
        fn takes_ownership(vec: Vec<i32>) {
            println!("関数内で使用: {:?}", vec);
            // vec はここでドロップされる
        }

        // 借用を渡す場合
        fn borrows_vec(vec: &Vec<i32>) {
            println!("借用で使用: {:?}", vec);
        }

        println!("元のベクター: {:?}", v);

        // 借用で渡す（v は引き続き使用可能）
        borrows_vec(&v);
        println!("借用後も使用可能: {:?}", v);

        // 所有権を渡す（この後 v は使用不可）
        takes_ownership(v);
        // println!("所有権移動後: {:?}", v); // エラー

        println!("所有権を渡した後、元の変数は使用不可");
    }

    // パフォーマンスの考慮
    println!("\n=== パフォーマンスの考慮 ===");
    {
        let large_vec: Vec<i32> = (0..1000000).collect();

        // clone() は重い操作
        let start = std::time::Instant::now();
        let _cloned = large_vec.clone();
        println!("clone()の時間: {:?}", start.elapsed());

        // 借用は軽い操作
        let start = std::time::Instant::now();
        let _borrowed = &large_vec;
        println!("借用の時間: {:?}", start.elapsed());

        println!("借用は参照の作成のみなので、ほぼコストゼロ");
    }
}
