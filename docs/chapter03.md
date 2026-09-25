# 第3章：トレイトシステムの高度な活用

## 概要

Rustのトレイトシステムは、インターフェースの定義と実装を分離し、ジェネリックプログラミングを強力にサポートする中核機能です。本章では、トレイトシステムの高度な機能と実践的な活用方法を深く探求します。

## 3.1 トレイトオブジェクトとオブジェクト安全性

### 動的ディスパッチとトレイトオブジェクト

トレイトオブジェクトは、実行時に型が決定される動的ディスパッチを可能にします。

```rust
trait Draw {
    fn draw(&self);
}

// dyn Drawはトレイトオブジェクト
let drawable: Box<dyn Draw> = Box::new(Circle { radius: 5.0 });
```

### オブジェクト安全性の条件

トレイトがオブジェクト安全である要件は以下です。

- Selfを返すメソッドを持たない
- ジェネリックメソッドを持たない
- Selfに対するトレイト境界を持たない
- 関連定数を持たない（一部の場合）

```rust
// オブジェクト安全
trait ObjectSafe {
    fn method(&self);
}

// オブジェクト安全でない
trait NotObjectSafe {
    fn new() -> Self;  // Selfを返す
    fn generic<T>(&self, x: T);  // ジェネリックメソッド
}
```

### 実行例

```bash
cargo run --bin trait_objects
```

## 3.2 関連型（Associated Types）vs ジェネリックパラメータ

### 関連型の利点

関連型は、トレイトの実装ごとに一意の型を定義します。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        // 実装
    }
}
```

### ジェネリックパラメータとの比較

```rust
// ジェネリックパラメータ：複数の実装が可能
trait Container<T> {
    fn get(&self) -> &T;
}

// 関連型：実装は一つだけ
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

### 使い分けの指針

- **関連型を使用**: トレイトの実装が型に対して一意の場合
- **ジェネリックパラメータを使用**: 同じ型に対して複数の実装が必要な場合

### 実行例

```bash
cargo run --bin associated_types
```

## 3.3 トレイト境界の高度なパターン

### where句の効果的な使用

```rust
fn complex_function<T, U, V>(t: T, u: U) -> V
where
    T: Display + Clone,
    U: Debug + Default,
    V: From<T> + From<U>,
{
    // 実装
}
```

### 高階トレイト境界（HRTB）との組み合わせ

```rust
fn higher_ranked<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    // すべてのライフタイムで動作
}
```

### トレイト境界の合成

```rust
// 複数のトレイトを要求
trait Combined: Debug + Display + Clone {}

// 自動実装
impl<T: Debug + Display + Clone> Combined for T {}
```

### 実行例

```bash
cargo run --bin trait_bounds
```

## 3.4 コヒーレンスルールと孤児ルール

### 孤児ルール（Orphan Rule）

外部クレートの型に外部クレートのトレイトを実装することはできません。

```rust
// ❌ エラー：Vec<T>もDisplayも外部で定義
impl Display for Vec<String> {
    // ...
}

// ✅ OK：MyTypeは自分で定義
struct MyType;
impl Display for MyType {
    // ...
}

// ✅ OK：ラッパー型を使用
struct Wrapper(Vec<String>);
impl Display for Wrapper {
    // ...
}
```

### コヒーレンスの保証

トレイト実装の一意性を保証するルール

```rust
// トレイトとその実装
trait MyTrait {
    fn method(&self);
}

// ブランケット実装
impl<T: Display> MyTrait for T {
    fn method(&self) {
        println!("{}", self);
    }
}
```

### 実行例

```bash
cargo run --bin coherence_rules
```

## 3.5 特殊化（Specialization）と将来の展望

### 特殊化の概念（不安定機能）

より具体的な実装が一般的な実装を上書きできる機能です。

```rust
#![feature(specialization)]

trait Example {
    fn method(&self);
}

// 一般的な実装
impl<T> Example for T {
    default fn method(&self) {
        println!("一般的な実装");
    }
}

// 特殊化された実装
impl Example for String {
    fn method(&self) {
        println!("String専用の実装");
    }
}
```

### 現在の代替手法

```rust
// マーカートレイトによる分岐
trait SpecialMarker {}

trait MyTrait {
    fn method(&self);
}

impl<T> MyTrait for T {
    default fn method(&self) {
        // デフォルト実装
    }
}

impl<T: SpecialMarker> MyTrait for T {
    fn method(&self) {
        // 特殊な実装
    }
}
```

### 実行例

```bash
cargo run --bin specialization_patterns
```

## 高度なトレイトパターン

### 3.6 ファントムデータとトレイト

```rust
use std::marker::PhantomData;

struct Container<T> {
    data: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container {
            data: Vec::new(),
            _phantom: PhantomData,
        }
    }
}
```

### 実行例

```bash
cargo run --bin phantom_traits
```

### 3.7 トレイトの自動実装

```rust
// 派生マクロによる自動実装
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// カスタム派生マクロの例
trait MyTrait {
    fn my_method(&self);
}

// procマクロで自動実装を生成
```

### 実行例

```bash
cargo run --bin auto_traits
```

### 3.8 高度なトレイト設計パターン

```rust
// Builder パターン
trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}

// State パターン
trait State {
    fn transition(self: Box<Self>) -> Box<dyn State>;
}

// Visitor パターン
trait Visitor {
    fn visit_node(&mut self, node: &Node);
}
```

### 実行例

```bash
cargo run --bin trait_patterns
```

### 3.9 パフォーマンスとトレイト

```rust
// 静的ディスパッチ（コンパイル時に解決）
fn static_dispatch<T: Display>(item: T) {
    println!("{}", item);
}

// 動的ディスパッチ（実行時に解決）
fn dynamic_dispatch(item: &dyn Display) {
    println!("{}", item);
}
```

### 実行例

```bash
cargo run --bin trait_performance
```

## 復習問題

### 問題1：オブジェクト安全性

次のトレイトがオブジェクト安全でない理由を説明し、修正してください。

```rust
trait Container {
    fn new() -> Self;
    fn add<T>(&mut self, item: T);
    fn count(&self) -> usize;
}
```

### 問題2：関連型の使用

Iteratorトレイトを参考に、独自のStreamトレイトを関連型を使って設計してください。
要件：

- アイテムの型
- エラーの型
- 次のアイテムを取得するメソッド

### 問題3：トレイト境界

複数のトレイト境界を持つジェネリック関数を実装してください。

- 入力はDisplay + Debug
- 出力はClone + Default
- 変換処理を行う

### 問題4：コヒーレンスルール

次のコードがコンパイルエラーになる理由を説明し、回避策を提示してください。

```rust
use std::fmt::Display;

impl Display for Option<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 実装
    }
}
```

### 問題5：実装課題

キャッシュ機能を持つトレイトを設計してください。
要件：

- get/setメソッド
- 容量制限
- 統計情報の取得
- 異なるキャッシュ戦略（LRU、FIFO）のサポート

## 模範解答

### 問題1の解答: オブジェクト安全性

**問題点の分析:**
元のトレイトがオブジェクト安全でない理由

1. `fn new() -> Self` - `Self`を返すメソッド（静的メソッド）
2. `fn add<T>(&mut self, item: T)` - ジェネリックメソッド

**修正されたトレイト:**

```rust
// オブジェクト安全なバージョン
trait Container {
    fn count(&self) -> usize;
    fn add_item(&mut self, item: Box<dyn std::any::Any>);
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

// より実用的な設計パターン
trait ContainerFactory {
    type Container: Container;
    fn create() -> Self::Container;
}

// 具体的な実装例
use std::any::Any;

struct VecContainer {
    items: Vec<Box<dyn Any>>,
}

impl Container for VecContainer {
    fn count(&self) -> usize {
        self.items.len()
    }
    
    fn add_item(&mut self, item: Box<dyn Any>) {
        self.items.push(item);
    }
}

struct VecContainerFactory;

impl ContainerFactory for VecContainerFactory {
    type Container = VecContainer;
    
    fn create() -> Self::Container {
        VecContainer {
            items: Vec::new(),
        }
    }
}

// 使用例
fn main() {
    let mut container = VecContainerFactory::create();
    container.add_item(Box::new(42i32));
    container.add_item(Box::new("hello".to_string()));
    
    println!("Items: {}", container.count());
    
    // トレイトオブジェクトとして使用可能
    let containers: Vec<Box<dyn Container>> = vec![
        Box::new(container),
    ];
}
```

**解説:**

- `new()`メソッドを`ContainerFactory`トレイトに分離
- ジェネリックメソッドを`Box<dyn Any>`を使用する方法で回避
- これによりトレイトオブジェクト（`Box<dyn Container>`）として使用可能

### 問題2の解答: 関連型によるStreamトレイト設計

```rust
use std::future::Future;
use std::pin::Pin;

// 基本的なStreamトレイト
trait Stream {
    type Item;
    type Error;
    
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
}

// 非同期バージョン
trait AsyncStream {
    type Item;
    type Error;
    
    fn poll_next(
        &mut self
    ) -> Pin<Box<dyn Future<Output = Result<Option<Self::Item>, Self::Error>> + '_>>;
}

// より高度なStreamトレイト
trait AdvancedStream: Stream {
    // ストリームのメタデータ
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
    
    // ストリームの状態
    fn is_terminated(&self) -> bool;
    
    // 便利なメソッド
    fn collect<C>(self) -> Result<C, Self::Error>
    where
        Self: Sized,
        C: Default + Extend<Self::Item>,
    {
        let mut collection = C::default();
        let mut stream = self;
        
        while let Some(item) = stream.next()? {
            collection.extend(std::iter::once(item));
        }
        
        Ok(collection)
    }
}

// 具体的な実装例：数値ストリーム
struct NumberStream {
    current: i32,
    max: i32,
    fail_at: Option<i32>,
}

impl NumberStream {
    fn new(max: i32) -> Self {
        NumberStream {
            current: 0,
            max,
            fail_at: None,
        }
    }
    
    fn with_failure_at(mut self, fail_at: i32) -> Self {
        self.fail_at = Some(fail_at);
        self
    }
}

#[derive(Debug)]
enum StreamError {
    NumberTooLarge,
    UnexpectedFailure,
}

impl Stream for NumberStream {
    type Item = i32;
    type Error = StreamError;
    
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(fail_at) = self.fail_at {
            if self.current == fail_at {
                return Err(StreamError::UnexpectedFailure);
            }
        }
        
        if self.current >= self.max {
            return Ok(None);
        }
        
        if self.current > 100 {
            return Err(StreamError::NumberTooLarge);
        }
        
        let current = self.current;
        self.current += 1;
        Ok(Some(current))
    }
}

impl AdvancedStream for NumberStream {
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.max - self.current).max(0) as usize;
        (remaining, Some(remaining))
    }
    
    fn is_terminated(&self) -> bool {
        self.current >= self.max
    }
}

// 使用例
fn main() -> Result<(), StreamError> {
    let mut stream = NumberStream::new(5);
    
    while let Some(item) = stream.next()? {
        println!("Item: {}", item);
    }
    
    // collect使用例
    let stream2 = NumberStream::new(3);
    let numbers: Vec<i32> = stream2.collect()?;
    println!("Collected: {:?}", numbers);
    
    // エラーハンドリング例
    let mut failing_stream = NumberStream::new(10).with_failure_at(3);
    loop {
        match failing_stream.next() {
            Ok(Some(item)) => println!("Item: {}", item),
            Ok(None) => break,
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

**設計のポイント:**

- `Item`と`Error`を関連型として定義
- 基本的な`next()`メソッドに加え、便利なメソッドを提供
- エラーハンドリングを組み込んだ設計
- 非同期版も考慮した拡張可能な設計

### 問題3の解答: 複数トレイト境界を持つジェネリック関数

```rust
use std::fmt::{Debug, Display};

// 基本的なパターン
fn process_and_convert<T, U>(input: T) -> U
where
    T: Display + Debug + Clone,
    U: Clone + Default + From<String>,
{
    println!("Debug: {:?}", input);
    println!("Display: {}", input);
    
    let string_repr = format!("{}", input);
    U::from(string_repr)
}

// より複雑な例：複数の入力と出力
fn complex_transformation<T, U, V, W>(
    input1: T,
    input2: U,
    config: V,
) -> Result<W, String>
where
    T: Display + Debug + Clone + PartialEq,
    U: Debug + Into<String>,
    V: Clone + Default,
    W: From<String> + Clone + Debug,
{
    println!("Processing input1: {:?}", input1);
    println!("Processing input2: {:?}", input2);
    
    if input1 == input1.clone() {
        let combined = format!("{}-{}", input1, input2.into());
        Ok(W::from(combined))
    } else {
        Err("Processing failed".to_string())
    }
}

// 実用的な例：データの検証と変換
trait Validatable {
    fn is_valid(&self) -> bool;
}

trait Serializable {
    fn serialize(&self) -> String;
}

fn validate_and_serialize<T, U>(data: T) -> Result<U, String>
where
    T: Validatable + Serializable + Display + Debug,
    U: From<String> + Default + Clone,
{
    println!("Validating data: {}", data);
    
    if !data.is_valid() {
        return Err(format!("Invalid data: {:?}", data));
    }
    
    let serialized = data.serialize();
    Ok(U::from(serialized))
}

// 具体的な型の実装例
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

impl Validatable for Person {
    fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.age > 0 && self.age < 150
    }
}

impl Serializable for Person {
    fn serialize(&self) -> String {
        format!("{}:{}", self.name, self.age)
    }
}

// カスタム出力型
#[derive(Debug, Clone, Default)]
struct PersonRecord(String);

impl From<String> for PersonRecord {
    fn from(s: String) -> Self {
        PersonRecord(s)
    }
}

// where句の代替表記法
fn alternative_syntax<T: Display + Debug + Clone, U: Clone + Default + From<String>>(
    input: T
) -> U {
    process_and_convert(input)
}

// 高階トレイト境界との組み合わせ
fn with_higher_ranked_bounds<T, F, R>(data: T, processor: F) -> R
where
    T: Display + Debug,
    F: for<'a> Fn(&'a str) -> R,
    R: Clone + Debug,
{
    let string_data = format!("{}", data);
    processor(&string_data)
}

// 使用例
fn main() -> Result<(), String> {
    // 基本的な使用例
    let number = 42;
    let result: String = process_and_convert(number);
    println!("Result: {}", result);
    
    // 複雑な変換例
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let record: PersonRecord = validate_and_serialize(person)?;
    println!("Record: {:?}", record);
    
    // 複数入力の例
    let input1 = "test";
    let input2 = 123;
    let config = ();
    let result: String = complex_transformation(input1, input2, config)?;
    println!("Complex result: {:?}", result);
    
    // 高階トレイト境界の例
    let data = "hello world";
    let processed: String = with_higher_ranked_bounds(data, |s| {
        s.to_uppercase()
    });
    println!("Processed: {:?}", processed);
    
    Ok(())
}
```

**トレイト境界設計のポイント:**

- `where`句を使用した読みやすい記述
- 複数の制約を組み合わせた柔軟な設計
- エラーハンドリングの組み込み
- 実際の用途を想定した実装例

### 問題4の解答: コヒーレンスルールとコンパイルエラー

**エラーの理由:**

```rust
// ❌ エラー：孤児ルール違反
impl Display for Option<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 実装
    }
}
```

**問題点:**

- `Option<T>`は標準ライブラリで定義された外部型
- `Display`も標準ライブラリで定義された外部トレイト
- 孤児ルールにより、両方とも外部の型とトレイトには実装できない

**回避策:**

**方法1: New Type パターン**:

```rust
use std::fmt::{Display, Formatter, Result as FmtResult};

// ラッパー型を作成
struct DisplayOption(Option<String>);

impl Display for DisplayOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            Some(s) => write!(f, "Some({})", s),
            None => write!(f, "None"),
        }
    }
}

impl From<Option<String>> for DisplayOption {
    fn from(opt: Option<String>) -> Self {
        DisplayOption(opt)
    }
}

impl From<DisplayOption> for Option<String> {
    fn from(display_opt: DisplayOption) -> Self {
        display_opt.0
    }
}

// 使用例
fn main() {
    let opt = Some("hello".to_string());
    let display_opt = DisplayOption::from(opt);
    println!("{}", display_opt); // "Some(hello)"
    
    let none_opt = DisplayOption(None);
    println!("{}", none_opt); // "None"
}
```

**方法2: 拡張トレイトパターン**:

```rust
trait DisplayExt {
    fn display_pretty(&self) -> String;
}

impl DisplayExt for Option<String> {
    fn display_pretty(&self) -> String {
        match self {
            Some(s) => format!("Some({})", s),
            None => "None".to_string(),
        }
    }
}

// 使用例
fn main() {
    let opt = Some("hello".to_string());
    println!("{}", opt.display_pretty());
    
    let none_opt: Option<String> = None;
    println!("{}", none_opt.display_pretty());
}
```

**方法3: ジェネリック関数アプローチ**:

```rust
fn display_option<T: Display>(opt: &Option<T>) -> String {
    match opt {
        Some(value) => format!("Some({})", value),
        None => "None".to_string(),
    }
}

// より汎用的な実装
trait OptionDisplay {
    fn to_display_string(&self) -> String;
}

impl<T: Display> OptionDisplay for Option<T> {
    fn to_display_string(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value),
            None => "None".to_string(),
        }
    }
}

// 使用例
fn main() {
    let opt = Some("hello".to_string());
    println!("{}", display_option(&opt));
    println!("{}", opt.to_display_string());
}
```

**方法4: マクロを使った実装**:

```rust
macro_rules! impl_display_for_option {
    ($type:ty) => {
        struct paste! { [<Display $type:camel Option>] }(Option<$type>);
        
        impl Display for paste! { [<Display $type:camel Option>] } {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match &self.0 {
                    Some(value) => write!(f, "Some({})", value),
                    None => write!(f, "None"),
                }
            }
        }
    };
}

// 使用例（実際のプロジェクトでは paste クレートが必要）
// impl_display_for_option!(String);
```

**推奨アプローチ:**
最も実用的で安全な方法は**New Type パターン**です。これにより型安全性を保ちながら、必要な機能を実装できます。

### 問題5の解答: キャッシュトレイトの設計

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

// 基本的なキャッシュトレイト
trait Cache<K, V> {
    type Error;
    
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }
}

// 統計情報を提供するトレイト
trait CacheStats {
    fn hit_count(&self) -> u64;
    fn miss_count(&self) -> u64;
    fn hit_rate(&self) -> f64 {
        let total = self.hit_count() + self.miss_count();
        if total == 0 {
            0.0
        } else {
            self.hit_count() as f64 / total as f64
        }
    }
    fn reset_stats(&mut self);
}

// キャッシュ戦略の列挙型
#[derive(Debug, Clone, Copy)]
enum EvictionStrategy {
    LRU,    // Least Recently Used
    FIFO,   // First In, First Out
    LFU,    // Least Frequently Used
}

// エラー型
#[derive(Debug, Clone)]
enum CacheError {
    CapacityExceeded,
    InvalidKey,
    StorageError(String),
}

// LRUキャッシュの実装
struct LRUCache<K, V> {
    map: HashMap<K, V>,
    order: Vec<K>,
    capacity: usize,
    hits: u64,
    misses: u64,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            order: Vec::new(),
            capacity,
            hits: 0,
            misses: 0,
        }
    }
    
    fn update_access_order(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let key = self.order.remove(pos);
            self.order.push(key);
        }
    }
    
    fn evict_lru(&mut self) -> Option<(K, V)> {
        if let Some(oldest_key) = self.order.first().cloned() {
            self.order.remove(0);
            if let Some(value) = self.map.remove(&oldest_key) {
                return Some((oldest_key, value));
            }
        }
        None
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    type Error = CacheError;
    
    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    
    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        // 既存のキーの場合は更新
        if self.map.contains_key(&key) {
            let old_value = self.map.insert(key.clone(), value);
            self.update_access_order(&key);
            return Ok(old_value);
        }
        
        // 容量チェック
        if self.is_full() {
            self.evict_lru();
        }
        
        let old_value = self.map.insert(key.clone(), value);
        self.order.push(key);
        Ok(old_value)
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.map.remove(key)
    }
    
    fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }
    
    fn len(&self) -> usize {
        self.map.len()
    }
    
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<K, V> CacheStats for LRUCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    fn hit_count(&self) -> u64 {
        self.hits
    }
    
    fn miss_count(&self) -> u64 {
        self.misses
    }
    
    fn reset_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

// FIFOキャッシュの実装
struct FIFOCache<K, V> {
    map: HashMap<K, V>,
    order: Vec<K>,
    capacity: usize,
    hits: u64,
    misses: u64,
}

impl<K, V> FIFOCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    fn new(capacity: usize) -> Self {
        FIFOCache {
            map: HashMap::new(),
            order: Vec::new(),
            capacity,
            hits: 0,
            misses: 0,
        }
    }
}

impl<K, V> Cache<K, V> for FIFOCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    type Error = CacheError;
    
    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    
    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        if self.map.contains_key(&key) {
            return Ok(self.map.insert(key, value));
        }
        
        if self.is_full() {
            if let Some(oldest_key) = self.order.first().cloned() {
                self.order.remove(0);
                self.map.remove(&oldest_key);
            }
        }
        
        let old_value = self.map.insert(key.clone(), value);
        self.order.push(key);
        Ok(old_value)
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.map.remove(key)
    }
    
    fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }
    
    fn len(&self) -> usize {
        self.map.len()
    }
    
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<K, V> CacheStats for FIFOCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    fn hit_count(&self) -> u64 {
        self.hits
    }
    
    fn miss_count(&self) -> u64 {
        self.misses
    }
    
    fn reset_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

// 汎用キャッシュファクトリ
struct CacheFactory;

impl CacheFactory {
    fn create_cache<K, V>(
        strategy: EvictionStrategy,
        capacity: usize,
    ) -> Box<dyn Cache<K, V, Error = CacheError>>
    where
        K: Clone + Hash + Eq + Debug + 'static,
        V: Clone + Debug + 'static,
    {
        match strategy {
            EvictionStrategy::LRU => Box::new(LRUCache::new(capacity)),
            EvictionStrategy::FIFO => Box::new(FIFOCache::new(capacity)),
            EvictionStrategy::LFU => {
                // LFU実装は省略、実際にはより複雑
                Box::new(LRUCache::new(capacity))
            }
        }
    }
}

// 使用例とテスト
fn main() -> Result<(), CacheError> {
    println!("=== LRU Cache Demo ===");
    let mut lru_cache = LRUCache::new(3);
    
    // データの挿入
    lru_cache.set("a".to_string(), 1)?;
    lru_cache.set("b".to_string(), 2)?;
    lru_cache.set("c".to_string(), 3)?;
    
    println!("Cache size: {}", lru_cache.len());
    
    // アクセス
    if let Some(value) = lru_cache.get(&"a".to_string()) {
        println!("Found a: {}", value);
    }
    
    // 容量超過でLRU削除
    lru_cache.set("d".to_string(), 4)?;
    
    println!("After adding 'd', cache size: {}", lru_cache.len());
    
    // 統計情報
    println!("Hit rate: {:.2}%", lru_cache.hit_rate() * 100.0);
    
    println!("\n=== FIFO Cache Demo ===");
    let mut fifo_cache = FIFOCache::new(2);
    
    fifo_cache.set(1, "first")?;
    fifo_cache.set(2, "second")?;
    fifo_cache.set(3, "third")?; // "first" が削除される
    
    println!("FIFO cache contains key 1: {}", fifo_cache.get(&1).is_some());
    println!("FIFO cache contains key 2: {}", fifo_cache.get(&2).is_some());
    println!("FIFO cache contains key 3: {}", fifo_cache.get(&3).is_some());
    
    println!("\n=== Factory Pattern Demo ===");
    let mut cache = CacheFactory::create_cache::<String, i32>(
        EvictionStrategy::LRU,
        5,
    );
    
    for i in 0..7 {
        cache.set(format!("key{}", i), i * 10)?;
    }
    
    println!("Factory cache size: {}", cache.len());
    
    Ok(())
}
```

**設計の特徴:**

1. **柔軟なトレイト設計**: 基本機能と統計機能を分離
2. **複数の戦略**: LRU、FIFO等の異なるアルゴリズム
3. **型安全性**: ジェネリクスとトレイト境界による型安全
4. **エラーハンドリング**: 関連型でエラー処理を統一
5. **ファクトリパターン**: 戦略に応じたキャッシュ生成
6. **統計機能**: ヒット率等のメトリクス提供

この設計により、実用的で拡張可能なキャッシュシステムを構築できます。

## まとめ

トレイトシステムは、Rustの型システムの中核を成す強力な機能です。オブジェクト安全性、関連型、トレイト境界、コヒーレンスルールを理解することで、柔軟で安全なAPIを設計できます。実践では、パフォーマンスと抽象化のバランスを考慮しながら、適切なパターンを選択することが重要です。
