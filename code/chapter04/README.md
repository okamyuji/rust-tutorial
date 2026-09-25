# 第4章：非同期プログラミングとFuture - 実行ガイド

## 概要

この章では、Rustの非同期プログラミングシステムの詳細な動作原理と高度な使用方法を学習します。async/await構文、Futureトレイト、Pin、非同期ランタイムの選択など、非同期プログラミングの深い理解を目指します。

## ビルドと実行

### プロジェクトのビルド

```bash
# このディレクトリに移動
cd rust-tutorial/code/chapter04

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
# async/awaitの内部動作
cargo run --bin async_await_internals

# Futureトレイトとピン留め
cargo run --bin future_and_pin

# 非同期トレイトの現状と回避策
cargo run --bin async_traits

# タスクスポーンとエグゼキュータ
cargo run --bin task_executors

# 非同期コードにおけるライフタイム
cargo run --bin async_lifetimes

# ストリームと非同期イテレータ
cargo run --bin async_streams

# 並行性と並列性
cargo run --bin concurrency_patterns

# エラーハンドリングとキャンセレーション
cargo run --bin error_cancellation

# WebSocketサーバーの実装
cargo run --bin websocket_server
```

## サンプルプログラムの内容

### 1. async_await_internals.rs

- async関数のステートマシンへの変換
- 手動でのFuture実装
- asyncブロックの特性
- ジェネレータへの変換の理解

### 2. future_and_pin.rs

- Futureトレイトの詳細な実装
- Pinの必要性と使用方法
- 自己参照構造体の安全な扱い
- Unpin トレイトの理解

### 3. async_traits.rs

- async-traitクレートの使用
- トレイトメソッドでの非同期処理
- 動的ディスパッチと非同期
- impl Futureパターン

### 4. task_executors.rs

- 主要な非同期ランタイムの比較
- タスクのスポーンと管理
- エグゼキュータの選択指針
- パフォーマンス特性の理解

### 5. async_lifetimes.rs

- 非同期コードでのライフタイムの課題
- 借用チェッカーとの戦い方
- 所有権による解決策
- 実践的なパターン

### 6. async_streams.rs

- Streamトレイトの使用
- 非同期イテレータパターン
- カスタムストリームの実装
- ストリーム変換と組み合わせ

### 7. concurrency_patterns.rs

- 並行実行（Concurrent）パターン
- 並列実行（Parallel）パターン
- join!とselect!の使い分け
- spawn_blockingの活用

### 8. error_cancellation.rs

- 非同期でのエラーハンドリング
- タイムアウト処理
- キャンセレーション機構
- グレースフルシャットダウン

### 9. websocket_server.rs

- WebSocketサーバーの完全実装
- 複数クライアントの同時接続管理
- メッセージのブロードキャスト機能
- 接続ライフサイクル管理
- エラーリカバリーとグレースフルシャットダウン

## 依存関係

このプロジェクトでは以下のクレートを使用しています：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
futures = "0.3"
async-trait = "0.1"
pin-project = "1"
num_cpus = "1"
```

## 学習のポイント

1. **非同期プログラミングの基本概念**
   - async/awaitはゼロコスト抽象化
   - Futureはpollベースの遅延評価
   - エグゼキュータが実際の実行を担当

2. **Pinの重要性**
   - 自己参照構造体の安全な扱い
   - メモリ上での値の固定
   - UnpinとPinの使い分け

3. **ライフタイムの課題**
   - .awaitポイントでの借用制限
   - 所有権による解決策
   - 'staticライフタイムの活用

4. **パフォーマンス考慮**
   - ランタイムの選択
   - タスクの粒度
   - CPU集約的処理の扱い方

## トラブルシューティング

### コンパイルエラーが発生する場合

1. Rust のバージョンを確認（1.70.0以上推奨）

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

### よくある非同期エラー

- "future cannot be shared between threads safely"
  → Send + Syncトレイト境界の追加
  
- "borrowed value does not live long enough"
  → 所有権による値の移動を検討

- "cannot be pinned"
  → Pin::new()またはBox::pin()の使用

- "await` is only allowed inside `async` functions and blocks"
  → async関数またはブロック内で.awaitを使用

## パフォーマンス測定

リリースモードでパフォーマンスを測定：

```bash
# 並行性パターンをリリースモードで実行
cargo run --release --bin concurrency_patterns
```

## 実践的な使用例

### Tokioランタイムが適している場合

- Webサーバー・APIサーバー
- I/O集約的アプリケーション
- ネットワーク通信（WebSocketサーバーなど）
- データベースアクセス
- リアルタイム通信システム

### async-stdが適している場合

- 学習目的
- 標準ライブラリに似たAPI
- 軽量なアプリケーション

## 次のステップ

実際のWebアプリケーションやネットワークサービスの開発に進み、学習した非同期プログラミングの概念を実践的に活用してください。
