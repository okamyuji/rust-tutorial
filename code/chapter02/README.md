# 第2章：ライフタイムの深層理解 - 実行ガイド

## 概要

この章では、Rustのライフタイムシステムの詳細な動作原理と高度な使用方法を学習します。

## ビルドと実行

### プロジェクトのビルド

```bash
# このディレクトリに移動
cd rust-tutorial/code/chapter02

# プロジェクト全体をビルド
cargo build

# リリースモードでビルド（最適化あり）
cargo build --release
```

### メインプログラムの実行

```bash
cargo run
```

### 個別サンプルプログラムの実行

各サンプルプログラムは独立して実行可能です。

```bash
# ライフタイム注釈の基礎
cargo run --bin lifetime_annotations

# ライフタイム省略規則
cargo run --bin lifetime_elision

# 高階トレイト境界（HRTB）
cargo run --bin hrtb_examples

# 変性（variance）の理解
cargo run --bin variance_examples

# 'staticライフタイム
cargo run --bin static_lifetime

# 構造体とライフタイム
cargo run --bin lifetime_structs

# 複雑なライフタイムパターン
cargo run --bin complex_lifetimes

# ライフタイムとジェネリクス
cargo run --bin lifetime_generics

# 関数とライフタイム
cargo run --bin lifetime_functions
```

## サンプルプログラムの内容

### 1. lifetime_annotations.rs

- ライフタイム注釈の基本構文
- 参照の有効期間の明示的な指定
- 複数のライフタイムパラメータ
- ライフタイムの関係性の表現

### 2. lifetime_elision.rs

- 3つのライフタイム省略規則
- 関数における省略規則の適用
- メソッドにおける省略規則
- 省略できない場合の対処法

### 3. hrtb_examples.rs

- `for<'a>` 構文の理解
- クロージャとHRTB
- 高階関数でのライフタイム
- 実践的なHRTBの使用例

### 4. variance_examples.rs

- 共変性（covariance）
- 反変性（contravariance）
- 不変性（invariance）
- 型パラメータと変性の関係

### 5. static_lifetime.rs

- 'staticライフタイムの意味
- 静的変数と'static
- 文字列リテラルと'static
- Box::leakによる'static化

### 6. lifetime_structs.rs

- 構造体定義でのライフタイム
- 複数のライフタイムを持つ構造体
- implブロックでのライフタイム
- ライフタイムと可変性

### 7. complex_lifetimes.rs

- 自己参照構造体の問題
- Pinとライフタイム
- 複雑なライフタイム関係
- 実践的な解決パターン

### 8. lifetime_generics.rs

- ジェネリクスとライフタイムの組み合わせ
- トレイト実装でのライフタイム
- Associated Typesとライフタイム
- ライフタイム境界の活用

### 9. lifetime_functions.rs

- 関数シグネチャでのライフタイム
- コールバックとライフタイム
- 高階関数のライフタイム
- ライフタイムサブタイピング

## 学習のポイント

1. **ライフタイムは型の一部**
   - 参照型には常にライフタイムが存在する
   - 多くの場合は省略されているだけ

2. **借用チェッカーとの協調**
   - ライフタイムは借用チェッカーに情報を提供
   - コンパイル時の安全性保証

3. **実践での使用**
   - 必要最小限のライフタイム注釈
   - 省略規則の活用
   - エラーメッセージからの学習

## トラブルシューティング

### コンパイルエラーが発生する場合

1. Rust のバージョンを確認

   ```bash
   rustc --version
   cargo --version
   ```

2. 依存関係の更新

   ```bash
   cargo update
   ```

3. クリーンビルド

   ```bash
   cargo clean
   cargo build
   ```

### よくあるライフタイムエラー

- "lifetime may not live long enough"
  → ライフタイムの関係を見直す
  
- "borrowed value does not live long enough"
  → 値の所有権とスコープを確認

- "cannot infer an appropriate lifetime"
  → 明示的なライフタイム注釈を追加

## 次のステップ

第3章では「トレイトシステムの高度な活用」について学習します。
