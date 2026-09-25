# 第5章：マクロとメタプログラミング

## 概要
Rustのマクロシステムは、コンパイル時にコードを生成する強力なメタプログラミング機能を提供します。本章では、宣言的マクロ（macro_rules!）と手続き的マクロの両方について、実践的な例を通じて学びます。

## 学習目標
- 宣言的マクロ（macro_rules!）の作成と活用
- 手続き的マクロ（derive、attribute、function-like）の実装
- マクロの衛生性（hygiene）とスコープ規則
- コンパイル時計算とconst fn
- 実践的なマクロパターンとベストプラクティス

## 1. 宣言的マクロ（macro_rules!）

### 1.1 基本構文とパターンマッチング

```rust
macro_rules! vec_of_strings {
    () => {
        Vec::<String>::new()
    };
    ($($element:expr),*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($element.to_string());
            )*
            temp
        }
    };
}
```

### 1.2 反復とフラグメント指定子

Rustのマクロは様々なフラグメント指定子をサポートします：

- `expr`: 式
- `ident`: 識別子
- `ty`: 型
- `pat`: パターン
- `stmt`: 文
- `item`: アイテム（関数、構造体など）
- `block`: ブロック
- `meta`: メタアイテム（属性の内容）
- `tt`: トークンツリー

### 1.3 マクロの衛生性

Rustのマクロは衛生的（hygienic）であり、マクロ内で定義された識別子が呼び出し元のスコープと衝突しません。

```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}

let a = 10;
let result = using_a!(a * 2); // 外側のaが使われる（20）
```

## 2. 手続き的マクロ

### 2.1 Deriveマクロ
カスタムderiveマクロを作成して、構造体や列挙型に自動的にトレイトを実装します。

```rust
#[derive(MyDerive)]
struct MyStruct {
    field: String,
}
```

### 2.2 属性マクロ
関数やモジュールに適用できる属性マクロ：

```rust
#[my_attribute(some_param)]
fn my_function() {
    // ...
}
```

### 2.3 関数風マクロ
関数のように呼び出せるマクロ：

```rust
let sql = sql!(SELECT * FROM users WHERE age > 18);
```

## 3. 高度なマクロパターン

### 3.1 再帰的マクロ

マクロは自身を再帰的に呼び出すことができます：

```rust
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}
```

### 3.2 トークンマンチング

複雑なパターンマッチングのテクニック：

```rust
macro_rules! parse_kv {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(parse_kv!(@single $rest)),*]));
    
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}
```

## 4. const関数とコンパイル時計算

### 4.1 const fn

コンパイル時に評価可能な関数：

```rust
const fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

const FIB_10: u32 = fibonacci(10);
```

### 4.2 const generics

型パラメータとして定数を使用：

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}
```

## 5. 実践的なマクロ開発

### 5.1 エラーハンドリング

マクロ内でのコンパイルエラーの生成：

```rust
macro_rules! assert_impl {
    ($t:ty: $($trait:path),+) => {
        const _: fn() = || {
            fn assert_impl<T: ?Sized $(+ $trait)+>() {}
            assert_impl::<$t>();
        };
    };
}
```

### 5.2 デバッグテクニック

- `cargo expand`を使用したマクロ展開の確認
- `trace_macros!`による展開過程の追跡
- 段階的なマクロ開発

## 6. ベストプラクティス

### 6.1 マクロ設計の原則

1. **明確な名前付け**: マクロの目的が明確にわかる名前を使用
2. **最小限の使用**: 関数で実現できる場合はマクロを避ける
3. **ドキュメント化**: マクロの使用方法と制限を明記
4. **エラーメッセージ**: わかりやすいコンパイルエラーを生成

### 6.2 パフォーマンス考慮事項

- マクロ展開によるコンパイル時間への影響
- 生成されるコードサイズの管理
- インライン化との相互作用

## 復習問題

### 問題1：カスタムアサートマクロ

条件をチェックし、失敗時にカスタムメッセージを表示するマクロを実装してください。
（実装例：code/chapter05/problem1_custom_assert.rs）

### 問題2：ビルダーパターンマクロ

構造体に対してビルダーパターンを自動生成するマクロを作成してください。
（実装例：code/chapter05/problem2_builder_macro.rs）

### 問題3：型安全な単位系

コンパイル時に単位の整合性をチェックする型システムをマクロで実装してください。
（実装例：code/chapter05/problem3_type_safe_units.rs）

## サンプルプログラム一覧

1. `declarative_macros.rs` - 宣言的マクロの基礎
2. `macro_patterns.rs` - 高度なパターンマッチング
3. `recursive_macros.rs` - 再帰的マクロの実装
4. `proc_macro_derive.rs` - カスタムderiveマクロ
5. `proc_macro_attribute.rs` - 属性マクロの実装
6. `const_functions.rs` - const関数とコンパイル時計算
7. `macro_hygiene.rs` - マクロの衛生性
8. `debugging_macros.rs` - マクロのデバッグテクニック
9. `practical_macros.rs` - 実用的なマクロ集

## まとめ

Rustのマクロシステムは、型安全性を保ちながら強力なメタプログラミングを可能にします。適切に使用することで、ボイラープレートコードを削減し、より表現力豊かなAPIを作成できます。しかし、過度な使用は可読性を損なう可能性があるため、必要性を慎重に判断することが重要です。

## 参考資料

- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/)
- [Rust Reference: Macros](https://doc.rust-lang.org/reference/macros.html)
- [The Rust Programming Language: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
