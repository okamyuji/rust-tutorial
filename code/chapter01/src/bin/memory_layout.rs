// src/bin/memory_layout.rs
use std::mem;

fn main() {
    println!("=== メモリレイアウトの解析 ===\n");

    // スタック上の値
    let x = 42i32;
    println!("i32のサイズ: {}バイト", mem::size_of::<i32>());
    println!("xのアドレス: {:p}", &x);

    // String型の構造（ヒープへのポインタを含む）
    let s = String::from("Hello, Rust!");
    println!("\nString構造体のサイズ: {}バイト", mem::size_of::<String>());
    println!("String構造体のアドレス: {:p}", &s);
    println!("文字列データのアドレス: {:p}", s.as_ptr());
    println!("容量: {}バイト", s.capacity());
    println!("長さ: {}バイト", s.len());

    // Vec型の構造
    let v = vec![1, 2, 3, 4, 5];
    println!("\nVec構造体のサイズ: {}バイト", mem::size_of::<Vec<i32>>());
    println!("Vec構造体のアドレス: {:p}", &v);
    println!("データのアドレス: {:p}", v.as_ptr());

    // Box型
    let b = Box::new(100);
    println!("\nBox<i32>のサイズ: {}バイト", mem::size_of::<Box<i32>>());
    println!("Box構造体のアドレス: {:p}", &b);
    println!("ヒープ上のデータアドレス: {:p}", &*b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
