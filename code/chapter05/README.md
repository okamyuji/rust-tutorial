# 第5章：マクロとメタプログラミング

このディレクトリには、Rustのマクロシステムを学ぶためのサンプルコードが含まれています。

## ビルドと実行

### 全体のビルド
```bash
cargo build
```

### メインプログラムの実行
```bash
cargo run
```

### 個別のサンプルプログラムの実行

各サンプルは独立して実行できます：

```bash
# 宣言的マクロの基礎
cargo run --bin declarative_macros

# 高度なパターンマッチング
cargo run --bin macro_patterns

# 再帰的マクロ
cargo run --bin recursive_macros

# const関数とコンパイル時計算
cargo run --bin const_functions

# マクロの衛生性
cargo run --bin macro_hygiene

# デバッグテクニック
cargo run --bin debugging_macros

# 実用的なマクロ集
cargo run --bin practical_macros
```

### 復習問題の実行

```bash
# 問題1：カスタムアサートマクロ
cargo run --bin problem1_custom_assert

# 問題2：ビルダーパターンマクロ
cargo run --bin problem2_builder_macro

# 問題3：型安全な単位系
cargo run --bin problem3_type_safe_units
```

## サンプルプログラムの内容

1. **declarative_macros.rs** - macro_rules!の基本的な使い方
2. **macro_patterns.rs** - TTマンチング、内部ルールなど高度なパターン
3. **recursive_macros.rs** - 再帰的なマクロ展開の実装
4. **const_functions.rs** - コンパイル時計算とconst generics
5. **macro_hygiene.rs** - マクロの衛生性とスコープ規則
6. **debugging_macros.rs** - マクロ開発時のデバッグ手法
7. **practical_macros.rs** - 実用的なマクロパターン集

## 復習問題

### 問題1：カスタムアサートマクロ
様々な種類のアサーションマクロを実装し、より表現力豊かなテストを書けるようにします。

### 問題2：ビルダーパターンマクロ
構造体に対してビルダーパターンを自動生成するマクロを作成し、ボイラープレートコードを削減します。

### 問題3：型安全な単位系
物理単位をコンパイル時にチェックする型システムを実装し、単位の間違いを防ぎます。

## 学習のポイント

- マクロは強力ですが、過度な使用は可読性を損なう可能性があります
- 関数で実現できる場合は、マクロよりも関数を優先しましょう
- `cargo expand`を使用してマクロの展開結果を確認することが重要です
- マクロの衛生性により、予期しない名前の衝突を防げます

## デバッグのヒント

マクロのデバッグに役立つツール：

```bash
# マクロ展開の確認
cargo expand --bin declarative_macros

# トレースマクロの有効化（Rustのナイトリービルドが必要）
# trace_macros!(true);
```

## 注意事項

- 手続き的マクロ（proc_macro）は別クレートとして実装する必要があります
- const関数には現在いくつかの制限があります（ヒープアロケーションなど）
- マクロの再帰には深さ制限があります（デフォルトで128）
