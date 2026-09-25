# Chapter 01: 所有権システムとメモリ管理

## プロジェクト構造

```text
chapter01/
├── Cargo.toml
├── src/
│   ├── main.rs                    # メインプログラム
│   └── bin/
│       ├── memory_layout.rs       # メモリレイアウトの解析
│       ├── move_semantics.rs      # ムーブセマンティクス
│       ├── copy_vs_clone.rs       # CopyとCloneの比較
│       ├── borrowing_basics.rs    # 借用の基本
│       ├── advanced_borrowing.rs  # 高度な借用パターン
│       ├── interior_mutability.rs # 内部可変性
│       ├── mutex_example.rs       # Mutexの使用例
│       ├── smart_pointers.rs      # スマートポインタ
│       └── pointer_performance.rs # パフォーマンス比較
```

## ビルド方法

```bash
# プロジェクトディレクトリに移動
cd rust-tutorial/code/chapter01

# すべてのバイナリをビルド
cargo build

# リリースモードでビルド（最適化あり）
cargo build --release
```

## 実行方法

### メインプログラム

```bash
cargo run
```

### 個別のサンプルプログラム

```bash
# メモリレイアウトの解析
cargo run --bin memory_layout

# ムーブセマンティクス
cargo run --bin move_semantics

# CopyとCloneの比較
cargo run --bin copy_vs_clone

# 借用の基本
cargo run --bin borrowing_basics

# 高度な借用パターン
cargo run --bin advanced_borrowing

# 内部可変性
cargo run --bin interior_mutability

# Mutexの使用例
cargo run --bin mutex_example

# スマートポインタ
cargo run --bin smart_pointers

# パフォーマンス比較（リリースモード推奨）
cargo run --bin pointer_performance --release
```

## 学習の進め方

1. まず`cargo run`でメインプログラムを実行し、所有権の基本概念を理解する
2. 各サンプルプログラムを順番に実行し、出力を確認する
3. ソースコードを読み、コメントと実行結果を照らし合わせる
4. コードを変更して実験し、コンパイルエラーから学ぶ

## トラブルシューティング

### ビルドエラーが発生する場合

1. Rustが正しくインストールされているか確認

   ```bash
   rustc --version
   cargo --version
   ```

2. Rustのバージョンが1.70.0以上であることを確認

3. 依存関係の問題がある場合

   ```bash
   cargo clean
   cargo update
   cargo build
   ```

### 環境設定

Rustがインストールされていない場合

```bash
# Rustupを使用してインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# パスを通す
source $HOME/.cargo/env
```
