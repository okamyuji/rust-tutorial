// src/bin/advanced_borrowing.rs

fn main() {
    println!("=== 高度な借用パターン ===\n");

    split_borrowing();
    reborrowing();
    struct_field_borrowing();
}

fn split_borrowing() {
    println!("--- 分割借用 ---");

    let mut v = vec![1, 2, 3, 4, 5, 6];

    // split_at_mutは内部でunsafeを使用して、
    // 異なる部分への可変参照を安全に返す
    let (left, right) = v.split_at_mut(3);

    println!("左側: {:?}", left);
    println!("右側: {:?}", right);

    // 異なる部分なので同時に変更可能
    left[0] = 10;
    right[0] = 40;

    // 全体を表示するために一旦借用を終了させる
    let _ = left;
    let _ = right;

    println!("変更後の全体: {:?}", v);
}

fn reborrowing() {
    println!("\n--- 再借用 ---");

    let mut x = 42;
    let r1 = &mut x;

    // *r1は一時的に値を参照外しするが、
    // すぐに&mutで再借用される
    let r2 = &mut *r1;

    *r2 += 10;
    // r2の使用終了後、r1が再び使用可能
    let _ = r2; // 明示的にドロップ

    *r1 += 5;

    let _ = r1; // r1をドロップしてxを再び使えるようにする

    println!("最終的な値: {}", x);
}

fn struct_field_borrowing() {
    println!("\n--- 構造体フィールドの借用 ---");

    struct Player {
        name: String,
        score: i32,
        inventory: Vec<String>,
    }

    let mut player = Player {
        name: String::from("Alice"),
        score: 100,
        inventory: vec![String::from("Sword"), String::from("Shield")],
    };

    // 異なるフィールドは独立して借用可能
    let name_ref = &player.name;
    let score_ref = &mut player.score;
    let inv_ref = &player.inventory;

    println!("プレイヤー: {}", name_ref);
    *score_ref += 50;
    println!("アイテム数: {}", inv_ref.len());

    // 借用を終了させる
    let _ = name_ref;
    let _ = score_ref;
    let _ = inv_ref;

    println!("更新後のスコア: {}", player.score);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
