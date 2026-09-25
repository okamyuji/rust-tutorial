# 第1章：所有権システムとメモリ管理

## 1.1 所有権の規則とメモリセーフティ

Rustの所有権システムは、ガベージコレクタなしでメモリ安全性を保証するために設計されました。このシステムは、コンパイル時に以下の問題を防ぎます。

- ダングリングポインタ（解放済みメモリへの参照）
- 二重解放（同じメモリを複数回解放）
- データ競合（複数スレッドからの同時書き込み）

### 所有権の規則

1. 各値には、所有者（owner）と呼ばれる変数が1つだけ存在します。
2. 所有者は、同時に1つしか存在できません。
3. 所有者がスコープを抜けると、値は破棄されます。

### プロジェクトのセットアップ

```bash
# プロジェクトディレクトリに移動
cd code/chapter01

# ビルド（すべての警告とエラーを解消済み）
cargo build

# 基本例の実行
cargo run --bin borrowing_basics
cargo run --bin memory_layout
cargo run --bin move_semantics
cargo run --bin copy_vs_clone

# 応用例の実行
cargo run --bin advanced_borrowing
cargo run --bin interior_mutability
cargo run --bin smart_pointers
cargo run --bin mutex_example
cargo run --bin pointer_performance
```

### メモリレイアウトの理解

`String`型は3つの要素から構成されます。

- ポインタ（ヒープ上のデータへの参照）
- 長さ（現在使用しているバイト数）
- 容量（確保されているバイト数）

これらはスタック上に配置され、実際の文字列データはヒープ上に配置されます。

## 1.2 ムーブセマンティクスとCopy/Cloneトレイト

### ムーブセマンティクス

Rustでは、ヒープにデータを持つ型の代入時に「ムーブ」が発生します。これは二重解放を防ぐための仕組みです。

```rust
let s1 = String::from("hello");
let s2 = s1; // s1の所有権がs2に移動
// s1は使用不可
```

### Copy vs Clone

- Copyトレイト スタック上のデータを、暗黙にビット単位でコピーします。
    - プリミティブ型（i32, f64, bool, charなど）
    - タプル（すべての要素がCopyの場合）
    - 配列（要素がCopyの場合）

- Cloneトレイト 明示的に呼び出して、深いコピーを作ります。
    - String, Vec<T>などのヒープを使用する型
    - カスタム型で`#[derive(Clone)]`を使用

## 1.3 借用チェッカーの動作原理

### 借用の基本ルール

1. ある時点で持てるのは、1つの可変参照か、任意の数の不変参照のどちらかです。
2. 参照は、常に有効な値を指していなければなりません。

### Non-Lexical Lifetimes (NLL)

Rust 2018エディション以降、借用チェッカーはより賢くなりました。参照の有効期間は、最後に使用された時点までとなります。

```rust
let mut vec = vec![1, 2, 3];
let r1 = &vec[0];
println!("{}", r1); // r1の最後の使用
vec.push(4); // NLLにより、ここで可変借用が可能
```

## 1.4 内部可変性パターン

### 使い分けのガイドライン

| パターン | 用途 | スレッドセーフ | オーバーヘッド |
|---------|------|--------------|---------------|
| Cell<T> | Copyな型の単純な変更 | ✗ | 最小 |
| RefCell<T> | 複雑な型、実行時借用チェック | ✗ | 小 |
| Mutex<T> | マルチスレッド環境 | ✓ | 中 |
| RwLock<T> | 読み取り多数の場合 | ✓ | 中 |

### RefCellの借用ルール

- `borrow()`: 不変借用（複数可）
- `borrow_mut()`: 可変借用（排他的）
- 借用ルールに違反すると、実行時にパニックが発生します。

## 1.5 スマートポインタの選択ガイド

### 決定木

```text
単一の所有者？
├─ Yes → 再帰的データ構造？
│        ├─ Yes → Box<T>
│        └─ No → 通常の所有権
└─ No → マルチスレッド？
         ├─ Yes → Arc<T>（+ Mutex<T>で可変性）
         └─ No → Rc<T>（+ RefCell<T>で可変性）
```

### パフォーマンス特性

| 型 | 作成コスト | クローンコスト | メモリオーバーヘッド |
|----|-----------|--------------|-------------------|
| Box<T> | 低 | - | 0バイト |
| Rc<T> | 中 | 低（参照カウント増加） | 8バイト |
| Arc<T> | 中〜高 | 低（アトミック操作） | 8バイト |

## 復習問題

### 問題1: 所有権とムーブ

次のコードがコンパイルエラーになる理由を説明し、修正してください。

```rust
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{:?}", v);
    println!("{:?}", v2);
}
```

### 問題2: 借用チェッカー

次のコードの問題点を指摘し、修正してください。

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let r1 = &data;
    let r2 = &mut data;
    println!("{:?} {:?}", r1, r2);
}
```

### 問題3: 内部可変性

`Cell`と`RefCell`の使い分けについて、それぞれが適している場面を例を挙げて説明してください。

### 問題4: スマートポインタの選択

次の要件に対して、最適なスマートポインタを選択し、理由を説明してください。

1. グラフ構造で、ノードが複数の他のノードから参照される
2. 設定データを複数のスレッドで共有したい
3. 親子関係を持つツリー構造を実装したい

### 問題5: 実装課題

`RefCell`を使用して、不変参照でも内部のカウンタを更新できる`Statistics`構造体を実装してください。

## 模範解答

模範解答の実装とコード例は、次のソースコードファイルにあります。

### 問題1の解答: 所有権とムーブ

この解答では、問題点の分析と修正方法を示します。

- ファイル [`code/chapter01/src/bin/problem1_ownership_move.rs`](../code/chapter01/src/bin/problem1_ownership_move.rs)
- 実行方法 `cargo run --bin problem1_ownership_move`

このファイルの内容は次のとおりです。

- 問題点の詳細な説明
- 複数の修正方法（clone()、借用、関数との組み合わせ）
- パフォーマンスの考慮事項
- 所有権システムの動作原理の実例

### 問題2の解答: 借用チェッカー

この解答では、借用ルールの詳細解説を示します。

- ファイル [`code/chapter01/src/bin/problem2_borrowing_rules.rs`](../code/chapter01/src/bin/problem2_borrowing_rules.rs)
- 実行方法 `cargo run --bin problem2_borrowing_rules`

このファイルの内容は次のとおりです。

- 借用ルール違反の解決方法
- Non-Lexical Lifetimes (NLL) の例
- ダングリング参照の防止
- 複数の修正手法の比較

### 問題3の解答: 内部可変性の使い分け

この解答では、Cell<T>とRefCell<T>の実践的な使い分けを示します。

- ファイル [`code/chapter01/src/bin/problem3_interior_mutability.rs`](../code/chapter01/src/bin/problem3_interior_mutability.rs)
- 実行方法 `cargo run --bin problem3_interior_mutability`

このファイルの内容は次のとおりです。

- Cell<T>の使用場面（カウンター、設定管理）
- RefCell<T>の使用場面（図書館システム、キャッシュ）
- パフォーマンス比較と機能比較表
- エラーハンドリングの実例

### 問題4の解答: スマートポインタの選択

この解答では、各種スマートポインタの実践的な選択ガイドを示します。

- ファイル [`code/chapter01/src/bin/problem4_smart_pointers.rs`](../code/chapter01/src/bin/problem4_smart_pointers.rs)
- 実行方法 `cargo run --bin problem4_smart_pointers`

このファイルの内容は次のとおりです。

- グラフ構造の実装（Rc<T> + RefCell<T> + Weak<T>）
- マルチスレッド環境での設定共有（Arc<RwLock<T>>）
- ツリー構造の実装（Box<T>）
- 選択フローチャートとパフォーマンス比較

### 問題5の解答: RefCellを使った実装

この解答では、Statistics構造体の包括的な実装を示します。

- ファイル [`code/chapter01/src/bin/problem5_statistics.rs`](../code/chapter01/src/bin/problem5_statistics.rs)
- 実行方法 `cargo run --bin problem5_statistics`

このファイルの内容は次のとおりです。

- 基本的なStatistics実装
- 高度な統計機能（分散、標準偏差、中央値）
- エラーハンドリングの実例
- スレッドセーフ版の設計指針

### 実行手順

すべての模範解答は、次のコマンドで確認できます。

```bash
# プロジェクトディレクトリに移動
cd code/chapter01

# 全問題の実行
cargo run --bin problem1_ownership_move
cargo run --bin problem2_borrowing_rules  
cargo run --bin problem3_interior_mutability
cargo run --bin problem4_smart_pointers
cargo run --bin problem5_statistics

# または、すべてをビルドして確認
cargo build
```

### 学習のポイント

各ソースコードファイルは実行でき、コメントと実例を含みます。その特徴は次のとおりです。

- 理論と実践の結合 概念の説明と、動作するコード例を並べています。
- 段階的な学習 基本から応用まで、順を追って解説しています。
- エラーハンドリング 実際の開発で必要になるエラー処理を示しています。
- パフォーマンス考慮 実用的な最適化のヒントを載せています。
- ベストプラクティス Rustらしいコードの書き方を示しています。

実際にコードを実行して動作を確かめると、所有権システムとメモリ管理の理解が深まります。
