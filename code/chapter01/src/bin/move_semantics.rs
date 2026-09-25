// src/bin/move_semantics.rs

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
    name: String,
}

impl Buffer {
    fn new(name: &str, size: usize) -> Self {
        println!("Buffer '{}' を作成（{}バイト）", name, size);
        Buffer {
            data: vec![0; size],
            name: name.to_string(),
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("Buffer '{}' を解放", self.name);
    }
}

fn main() {
    println!("=== ムーブセマンティクスの詳細 ===\n");

    // 基本的なムーブ
    let buf1 = Buffer::new("buf1", 1024);
    let buf2 = buf1; // ムーブ発生
    println!("buf2を使用: {}", buf2.name);
    // buf1は使用不可

    println!("\n--- 関数呼び出しでのムーブ ---");

    let buf3 = Buffer::new("buf3", 2048);
    process_buffer(buf3); // 所有権が関数に移動
                          // buf3はもう使えない

    println!("\n--- 所有権を返す関数 ---");

    let buf4 = create_buffer("buf4", 512);
    let buf5 = modify_and_return(buf4);
    println!("返されたバッファ: {}", buf5.name);
}

fn process_buffer(buffer: Buffer) {
    println!("処理中: {} ({}バイト)", buffer.name, buffer.data.len());
    // 関数終了時にbufferがドロップされる
}

fn create_buffer(name: &str, size: usize) -> Buffer {
    Buffer::new(name, size)
}

fn modify_and_return(mut buffer: Buffer) -> Buffer {
    buffer.name.push_str("_modified");
    buffer // 所有権を呼び出し元に返す
}
