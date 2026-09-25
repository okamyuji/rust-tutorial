// src/bin/phantom_traits.rs

use std::fmt::Debug;
use std::marker::PhantomData;

fn main() {
    println!("=== ファントムデータとトレイト ===\n");

    phantom_data_basics();
    variance_control();
    type_state_with_phantom();
    lifetime_phantom();
}

// PhantomDataの基本
fn phantom_data_basics() {
    println!("--- PhantomDataの基本 ---");

    // 型パラメータを使用しない構造体
    struct Container<T> {
        data: Vec<u8>,
        _phantom: PhantomData<T>, // Tを「使用」する
    }

    impl<T> Container<T> {
        fn new() -> Self {
            Container {
                data: Vec::new(),
                _phantom: PhantomData,
            }
        }

        fn push_byte(&mut self, byte: u8) {
            self.data.push(byte);
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    // 型安全なAPI
    impl Container<String> {
        fn push_string(&mut self, s: &str) {
            self.data.extend(s.as_bytes());
        }

        fn as_string(&self) -> Result<String, std::string::FromUtf8Error> {
            String::from_utf8(self.data.clone())
        }
    }

    impl Container<i32> {
        fn push_i32(&mut self, n: i32) {
            self.data.extend(&n.to_le_bytes());
        }

        fn get_i32(&self, index: usize) -> Option<i32> {
            let start = index * 4;
            if start + 4 <= self.data.len() {
                let bytes = &self.data[start..start + 4];
                Some(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            } else {
                None
            }
        }
    }

    // 文字列コンテナ
    let mut str_container: Container<String> = Container::new();
    str_container.push_string("Hello, ");
    str_container.push_string("PhantomData!");

    // 直接バイトを追加
    str_container.push_byte(b'!');
    println!("コンテナサイズ: {} バイト", str_container.len());

    match str_container.as_string() {
        Ok(s) => println!("文字列コンテナ: {}", s),
        Err(e) => println!("エラー: {}", e),
    }

    // 整数コンテナ
    let mut int_container: Container<i32> = Container::new();
    int_container.push_i32(42);
    int_container.push_i32(100);
    int_container.push_i32(-7);

    println!("\n整数コンテナ:");
    for i in 0..3 {
        if let Some(n) = int_container.get_i32(i) {
            println!("  [{}] = {}", i, n);
        }
    }
}

// 変性（Variance）の制御
fn variance_control() {
    println!("\n--- 変性の制御 ---");

    // 共変（Covariant）
    struct Covariant<T> {
        _phantom: PhantomData<T>,
    }

    // 反変（Contravariant）
    struct Contravariant<T> {
        _phantom: PhantomData<fn(T)>,
    }

    // 不変（Invariant）
    struct Invariant<T> {
        _phantom: PhantomData<fn(T) -> T>,
    }

    // 所有権を表現
    struct Owner<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> Owner<T> {
        fn new() -> Self {
            Owner {
                _phantom: PhantomData,
            }
        }
    }

    // 借用を表現
    struct Borrower<'a, T> {
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T> Borrower<'a, T> {
        fn new() -> Self {
            Borrower {
                _phantom: PhantomData,
            }
        }
    }

    // 可変借用を表現
    struct MutBorrower<'a, T> {
        _phantom: PhantomData<&'a mut T>,
    }

    impl<'a, T> MutBorrower<'a, T> {
        fn new() -> Self {
            MutBorrower {
                _phantom: PhantomData,
            }
        }
    }

    let _owner: Owner<String> = Owner::new();
    let _borrower: Borrower<String> = Borrower::new();
    let _mut_borrower: MutBorrower<String> = MutBorrower::new();

    // 変性の例
    let _covariant: Covariant<&'static str> = Covariant {
        _phantom: PhantomData,
    };
    let _contravariant: Contravariant<String> = Contravariant {
        _phantom: PhantomData,
    };
    let _invariant: Invariant<i32> = Invariant {
        _phantom: PhantomData,
    };

    println!("変性の種類:");
    println!("  共変: サブタイプ関係が保持される");
    println!("  反変: サブタイプ関係が逆転する");
    println!("  不変: サブタイプ関係が成立しない");
}

// 型状態とPhantomData
fn type_state_with_phantom() {
    println!("\n--- 型状態とPhantomData ---");

    // 状態を表す型
    struct Locked;
    struct Unlocked;

    // ドアの実装
    struct Door<State = Locked> {
        name: String,
        _state: PhantomData<State>,
    }

    impl Door<Locked> {
        fn new(name: String) -> Self {
            Door {
                name,
                _state: PhantomData,
            }
        }

        fn unlock(self) -> Door<Unlocked> {
            println!("{}のロックを解除", self.name);
            Door {
                name: self.name,
                _state: PhantomData,
            }
        }
    }

    impl Door<Unlocked> {
        fn open(&self) {
            println!("{}を開く", self.name);
        }

        fn close(&self) {
            println!("{}を閉じる", self.name);
        }

        fn lock(self) -> Door<Locked> {
            println!("{}をロック", self.name);
            Door {
                name: self.name,
                _state: PhantomData,
            }
        }
    }

    let door = Door::new("正面玄関".to_string());
    // door.open(); // コンパイルエラー：ロックされたドアは開けない

    let unlocked_door = door.unlock();
    unlocked_door.open();
    unlocked_door.close();

    let _locked_door = unlocked_door.lock();
    // unlocked_door.open(); // コンパイルエラー：所有権が移動

    // より複雑な状態機械
    struct Empty;
    struct Partial;
    struct Full;

    struct Buffer<T, State = Empty> {
        data: Vec<T>,
        capacity: usize,
        _state: PhantomData<State>,
    }

    impl<T> Buffer<T, Empty> {
        fn new(capacity: usize) -> Self {
            Buffer {
                data: Vec::with_capacity(capacity),
                capacity,
                _state: PhantomData,
            }
        }

        fn add_first(mut self, item: T) -> Buffer<T, Partial> {
            self.data.push(item);
            Buffer {
                data: self.data,
                capacity: self.capacity,
                _state: PhantomData,
            }
        }
    }

    impl<T> Buffer<T, Partial> {
        fn add(mut self, item: T) -> Result<Buffer<T, Partial>, Buffer<T, Full>> {
            self.data.push(item);

            if self.data.len() >= self.capacity {
                Err(Buffer {
                    data: self.data,
                    capacity: self.capacity,
                    _state: PhantomData,
                })
            } else {
                Ok(Buffer {
                    data: self.data,
                    capacity: self.capacity,
                    _state: PhantomData,
                })
            }
        }

        fn clear(self) -> Buffer<T, Empty> {
            Buffer {
                data: Vec::with_capacity(self.capacity),
                capacity: self.capacity,
                _state: PhantomData,
            }
        }
    }

    impl<T: Debug> Buffer<T, Full> {
        fn process(&self) {
            println!("\nバッファ処理（満杯）: {:?}", self.data);
        }

        fn clear(self) -> Buffer<T, Empty> {
            Buffer {
                data: Vec::with_capacity(self.capacity),
                capacity: self.capacity,
                _state: PhantomData,
            }
        }
    }

    let buffer = Buffer::new(3);
    let partial = buffer.add_first(1);

    match partial.add(2) {
        Ok(partial) => match partial.add(3) {
            Ok(_) => println!("まだ満杯ではない"),
            Err(full) => {
                full.process();
                let _empty = full.clear();
            }
        },
        Err(full) => full.process(),
    }
}

// ライフタイムとPhantomData
fn lifetime_phantom() {
    println!("\n--- ライフタイムとPhantomData ---");

    // ライフタイムを持つ構造体
    struct Ref<'a, T> {
        _phantom: PhantomData<&'a T>,
        id: usize,
    }

    impl<'a, T> Ref<'a, T> {
        fn new(id: usize) -> Self {
            Ref {
                _phantom: PhantomData,
                id,
            }
        }

        fn id(&self) -> usize {
            self.id
        }
    }

    // Refの使用例
    let ref_example: Ref<i32> = Ref::new(42);
    println!("Ref ID: {}", ref_example.id());

    // スコープ付きリソース
    struct Resource<'a> {
        name: String,
        _phantom: PhantomData<&'a ()>,
    }

    impl<'a> Resource<'a> {
        fn new(name: String) -> Self {
            println!("リソース '{}' を取得", name);
            Resource {
                name,
                _phantom: PhantomData,
            }
        }
    }

    impl<'a> Drop for Resource<'a> {
        fn drop(&mut self) {
            println!("リソース '{}' を解放", self.name);
        }
    }

    // スコープマネージャー
    struct Scope<'a> {
        _phantom: PhantomData<&'a ()>,
    }

    impl<'a> Scope<'a> {
        fn new() -> Self {
            println!("スコープ開始");
            Scope {
                _phantom: PhantomData,
            }
        }

        fn create_resource(&self, name: String) -> Resource<'a> {
            Resource::new(name)
        }
    }

    impl<'a> Drop for Scope<'a> {
        fn drop(&mut self) {
            println!("スコープ終了");
        }
    }

    {
        let scope = Scope::new();
        let _res1 = scope.create_resource("データベース接続".to_string());
        let _res2 = scope.create_resource("ファイルハンドル".to_string());

        println!("スコープ内で作業中...");
    } // スコープとリソースが自動的に解放される

    // 複数のライフタイムとPhantomData
    struct MultiRef<'a, 'b, T, U> {
        _phantom_t: PhantomData<&'a T>,
        _phantom_u: PhantomData<&'b U>,
        data: String,
    }

    impl<'a, 'b, T, U> MultiRef<'a, 'b, T, U> {
        fn new(data: String) -> Self {
            MultiRef {
                _phantom_t: PhantomData,
                _phantom_u: PhantomData,
                data,
            }
        }

        fn get_data(&self) -> &str {
            &self.data
        }
    }

    let multi_ref: MultiRef<i32, String> = MultiRef::new("複数のライフタイム".to_string());
    println!("\n{}", multi_ref.get_data());

    println!("✓ PhantomDataによる型安全性の確保");
}

#[cfg(test)]
mod tests {
    #[test]
    fn variance_and_lifetime_demos_run_without_panicking() {
        super::variance_control();
        super::lifetime_phantom();
    }
}
