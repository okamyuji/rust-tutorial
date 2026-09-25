// src/bin/problem5_cache_trait.rs
// 復習問題5: キャッシュ機能を持つトレイトの実装

use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::{Duration, Instant};

// LRUキャッシュの実装
struct LRUCache<K, V> {
    data: HashMap<K, V>,
    order: VecDeque<K>,
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
            data: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
            hits: 0,
            misses: 0,
        }
    }

    fn update_access_order(&mut self, key: &K) {
        // 既存のキーを削除して最後に追加
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key.clone());
    }

    fn evict_lru(&mut self) -> Option<(K, V)> {
        if let Some(oldest_key) = self.order.pop_front() {
            if let Some(value) = self.data.remove(&oldest_key) {
                return Some((oldest_key, value));
            }
        }
        None
    }

    fn get_with_stats(&mut self, key: &K) -> Option<&V> {
        // 参照を保持したまま update_access_order(&mut self) を呼べないため、
        // 先に存在確認と順序更新を済ませてから参照を取り直す
        if !self.data.contains_key(key) {
            self.misses += 1;
            return None;
        }
        self.hits += 1;
        self.update_access_order(key);
        self.data.get(key)
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    type Error = CacheError;

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.data.contains_key(key) {
            self.update_access_order(key);
        }
        self.data.get_mut(key)
    }

    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        // 既存のキーの場合は更新
        if let Some(old_value) = self.data.get(&key) {
            let old = old_value.clone();
            self.data.insert(key.clone(), value);
            self.update_access_order(&key);
            return Ok(Some(old));
        }

        // 容量チェック
        if self.is_full() {
            self.evict_lru();
        }

        self.data.insert(key.clone(), value);
        self.order.push_back(key);
        Ok(None)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.data.remove(key)
    }

    fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<&V, Self::Error>
    where
        K: Clone,
        F: FnOnce() -> V,
    {
        if !self.data.contains_key(&key) {
            let value = f();
            self.set(key.clone(), value)?;
        } else {
            self.update_access_order(&key);
        }
        Ok(self.data.get(&key).unwrap())
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
    data: HashMap<K, V>,
    order: VecDeque<K>,
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
            data: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
            hits: 0,
            misses: 0,
        }
    }

    fn get_with_stats(&mut self, key: &K) -> Option<&V> {
        match self.data.get(key) {
            Some(value) => {
                self.hits += 1;
                Some(value)
            }
            None => {
                self.misses += 1;
                None
            }
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
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        if self.data.contains_key(&key) {
            return Ok(self.data.insert(key, value));
        }

        if self.is_full() {
            if let Some(oldest_key) = self.order.pop_front() {
                self.data.remove(&oldest_key);
            }
        }

        self.order.push_back(key.clone());
        Ok(self.data.insert(key, value))
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.data.remove(key)
    }

    fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<&V, Self::Error>
    where
        K: Clone,
        F: FnOnce() -> V,
    {
        if !self.data.contains_key(&key) {
            let value = f();
            self.set(key.clone(), value)?;
        }
        Ok(self.data.get(&key).unwrap())
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

fn main() {
    println!("=== 復習問題5: キャッシュトレイト設計 ===\n");

    // 基本的なキャッシュ機能
    basic_cache_demo();

    // LRUキャッシュの実装
    lru_cache_demo();

    // FIFOキャッシュの実装
    fifo_cache_demo();

    // 統計機能付きキャッシュ
    stats_cache_demo();

    // タイムベースキャッシュ
    time_based_cache_demo();

    // マルチレベルキャッシュ
    multi_level_cache_demo();
}

// タイムベースキャッシュの実装
struct TimedCache<K, V> {
    data: HashMap<K, (V, Instant)>,
    ttl: Duration,
    capacity: usize,
    hits: u64,
    misses: u64,
    expirations: u64,
}

impl<K, V> TimedCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    fn new(capacity: usize, ttl: Duration) -> Self {
        TimedCache {
            data: HashMap::with_capacity(capacity),
            ttl,
            capacity,
            hits: 0,
            misses: 0,
            expirations: 0,
        }
    }

    fn is_expired(&self, timestamp: &Instant) -> bool {
        timestamp.elapsed() > self.ttl
    }

    fn cleanup_expired(&mut self) {
        let expired_keys: Vec<K> = self
            .data
            .iter()
            .filter(|(_, (_, timestamp))| self.is_expired(timestamp))
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            self.data.remove(&key);
            self.expirations += 1;
        }
    }

    fn get_with_expiry(&mut self, key: &K) -> Option<&V> {
        // 期限判定を先に bool へ落とし、data への借用を切ってから変更する
        let expired = match self.data.get(key) {
            Some((_, timestamp)) => self.is_expired(timestamp),
            None => {
                self.misses += 1;
                return None;
            }
        };
        if expired {
            self.data.remove(key);
            self.expirations += 1;
            self.misses += 1;
            return None;
        }
        self.hits += 1;
        self.data.get(key).map(|(value, _)| value)
    }

    fn expiration_count(&self) -> u64 {
        self.expirations
    }
}

impl<K, V> Cache<K, V> for TimedCache<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    type Error = CacheError;

    fn get(&self, key: &K) -> Option<&V> {
        if let Some((value, timestamp)) = self.data.get(key) {
            if !self.is_expired(timestamp) {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let expired = self.data.get(key).map(|(_, ts)| self.is_expired(ts))?;
        if expired {
            None
        } else {
            self.data.get_mut(key).map(|(value, _)| value)
        }
    }

    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        self.cleanup_expired();

        if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
            return Err(CacheError::CapacityExceeded);
        }

        let old_value = self
            .data
            .insert(key, (value, Instant::now()))
            .map(|(v, _)| v);
        Ok(old_value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key).map(|(v, _)| v)
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<&V, Self::Error>
    where
        K: Clone,
        F: FnOnce() -> V,
    {
        if let Some(value) = self.get_with_expiry(&key) {
            // getは変更しないが、統計は更新済み
            return Ok(unsafe { std::mem::transmute(value) });
        }

        let value = f();
        self.set(key.clone(), value)?;
        Ok(&self.data.get(&key).unwrap().0)
    }
}

impl<K, V> CacheStats for TimedCache<K, V>
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
        self.expirations = 0;
    }
}

// 基本的なキャッシュトレイト定義
trait Cache<K, V> {
    type Error;

    // 基本操作
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);

    // メタデータ
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }

    // 高度な操作
    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<&V, Self::Error>
    where
        K: Clone,
        F: FnOnce() -> V;
}

// 統計情報を提供するトレイト
trait CacheStats {
    fn hit_count(&self) -> u64;
    fn miss_count(&self) -> u64;
    fn total_requests(&self) -> u64 {
        self.hit_count() + self.miss_count()
    }
    fn hit_rate(&self) -> f64 {
        let total = self.total_requests();
        if total == 0 {
            0.0
        } else {
            self.hit_count() as f64 / total as f64
        }
    }
    fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }
    fn reset_stats(&mut self);
}

// キャッシュ戦略の列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
enum EvictionStrategy {
    LRU,    // Least Recently Used
    FIFO,   // First In, First Out
    LFU,    // Least Frequently Used
    Random, // Random eviction
}

// エラー型
#[derive(Debug, Clone, PartialEq)]
enum CacheError {
    CapacityExceeded,
    InvalidKey,
    StorageError(String),
    StrategyError(String),
}

// 基本的なキャッシュ機能のデモ
fn basic_cache_demo() {
    println!("【基本的なキャッシュ機能】");

    // シンプルなHashMapベースキャッシュ
    struct SimpleCache<K, V> {
        data: HashMap<K, V>,
        capacity: usize,
        hits: u64,
        misses: u64,
    }

    impl<K, V> SimpleCache<K, V>
    where
        K: Clone + Hash + Eq + Debug,
        V: Clone + Debug,
    {
        fn new(capacity: usize) -> Self {
            SimpleCache {
                data: HashMap::with_capacity(capacity),
                capacity,
                hits: 0,
                misses: 0,
            }
        }
    }

    impl<K, V> Cache<K, V> for SimpleCache<K, V>
    where
        K: Clone + Hash + Eq + Debug,
        V: Clone + Debug,
    {
        type Error = CacheError;

        fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }

        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.data.get_mut(key)
        }

        fn set(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error> {
            if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
                return Err(CacheError::CapacityExceeded);
            }

            Ok(self.data.insert(key, value))
        }

        fn remove(&mut self, key: &K) -> Option<V> {
            self.data.remove(key)
        }

        fn clear(&mut self) {
            self.data.clear();
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<&V, Self::Error>
        where
            K: Clone,
            F: FnOnce() -> V,
        {
            if !self.data.contains_key(&key) {
                let value = f();
                self.set(key.clone(), value)?;
            }
            Ok(self.data.get(&key).unwrap())
        }
    }

    impl<K, V> CacheStats for SimpleCache<K, V>
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

    // 統計機能付きアクセス
    impl<K, V> SimpleCache<K, V>
    where
        K: Clone + Hash + Eq + Debug,
        V: Clone + Debug,
    {
        fn get_with_stats(&mut self, key: &K) -> Option<&V> {
            match self.data.get(key) {
                Some(value) => {
                    self.hits += 1;
                    Some(value)
                }
                None => {
                    self.misses += 1;
                    None
                }
            }
        }
    }

    // 使用例
    let mut cache = SimpleCache::new(3);

    println!("基本操作のテスト:");
    cache.set("key1".to_string(), "value1".to_string()).unwrap();
    cache.set("key2".to_string(), "value2".to_string()).unwrap();

    if let Some(value) = cache.get_with_stats(&"key1".to_string()) {
        println!("  取得成功: {}", value);
    }

    if cache.get_with_stats(&"key3".to_string()).is_none() {
        println!("  キー3は見つかりません");
    }

    println!("  キャッシュサイズ: {}/{}", cache.len(), cache.capacity());
    println!("  ヒット率: {:.2}%", cache.hit_rate() * 100.0);
}

// LRUキャッシュの実装
fn lru_cache_demo() {
    println!("\n【LRUキャッシュの実装】");

    // LRUキャッシュのテスト
    let mut lru = LRUCache::new(3);

    println!("LRUキャッシュのテスト:");

    lru.set("a".to_string(), 1).unwrap();
    lru.set("b".to_string(), 2).unwrap();
    lru.set("c".to_string(), 3).unwrap();

    println!("  初期状態: a=1, b=2, c=3");

    // aにアクセス（最新にする）
    lru.get_with_stats(&"a".to_string());
    println!("  aにアクセス後のLRU順: a(最新), c, b");

    // 容量超過でLRU削除
    lru.set("d".to_string(), 4).unwrap();
    println!("  d=4追加後、最古のbが削除される");

    println!("  現在のキー: {:?}", lru.order);
    println!("  ヒット率: {:.2}%", lru.hit_rate() * 100.0);
}

// FIFOキャッシュの実装
fn fifo_cache_demo() {
    println!("\n【FIFOキャッシュの実装】");

    // FIFOキャッシュのテスト
    let mut fifo = FIFOCache::new(3);

    println!("FIFOキャッシュのテスト:");

    fifo.set("x".to_string(), 10).unwrap();
    fifo.set("y".to_string(), 20).unwrap();
    fifo.set("z".to_string(), 30).unwrap();

    println!("  初期状態: x=10, y=20, z=30");

    // xにアクセス（FIFOでは順序は変わらない）
    fifo.get_with_stats(&"x".to_string());
    println!("  xにアクセス（FIFO順は変わらず）");

    // 容量超過で最初のx削除
    fifo.set("w".to_string(), 40).unwrap();
    println!("  w=40追加後、最初のxが削除される");

    println!("  現在のキー: {:?}", fifo.order);
    println!("  ヒット率: {:.2}%", fifo.hit_rate() * 100.0);
}

// 統計機能付きキャッシュ
fn stats_cache_demo() {
    println!("\n【統計機能付きキャッシュ】");

    struct StatsCache<K, V> {
        cache: LRUCache<K, V>,
        access_frequency: HashMap<K, u64>,
        last_access: HashMap<K, Instant>,
    }

    impl<K, V> StatsCache<K, V>
    where
        K: Clone + Hash + Eq + Debug,
        V: Clone + Debug,
    {
        fn new(capacity: usize) -> Self {
            StatsCache {
                cache: LRUCache::new(capacity),
                access_frequency: HashMap::new(),
                last_access: HashMap::new(),
            }
        }

        fn get_detailed_stats(&mut self, key: &K) -> Option<&V> {
            let result = self.cache.get_with_stats(key);

            if result.is_some() {
                *self.access_frequency.entry(key.clone()).or_insert(0) += 1;
                self.last_access.insert(key.clone(), Instant::now());
            }

            result
        }

        fn get_frequency(&self, key: &K) -> u64 {
            self.access_frequency.get(key).copied().unwrap_or(0)
        }

        fn get_last_access(&self, key: &K) -> Option<&Instant> {
            self.last_access.get(key)
        }

        fn most_accessed_keys(&self, limit: usize) -> Vec<(K, u64)> {
            let mut frequencies: Vec<_> = self
                .access_frequency
                .iter()
                .map(|(k, &v)| (k.clone(), v))
                .collect();
            frequencies.sort_by(|a, b| b.1.cmp(&a.1));
            frequencies.into_iter().take(limit).collect()
        }

        fn least_accessed_keys(&self, limit: usize) -> Vec<(K, u64)> {
            let mut frequencies: Vec<_> = self
                .access_frequency
                .iter()
                .map(|(k, &v)| (k.clone(), v))
                .collect();
            frequencies.sort_by(|a, b| a.1.cmp(&b.1));
            frequencies.into_iter().take(limit).collect()
        }
    }

    // StatsCache のテスト
    let mut stats_cache = StatsCache::new(4);

    // データの追加と アクセス
    stats_cache
        .cache
        .set("page1".to_string(), "content1".to_string())
        .unwrap();
    stats_cache
        .cache
        .set("page2".to_string(), "content2".to_string())
        .unwrap();
    stats_cache
        .cache
        .set("page3".to_string(), "content3".to_string())
        .unwrap();

    // 様々な頻度でアクセス
    for _ in 0..5 {
        stats_cache.get_detailed_stats(&"page1".to_string());
    }
    for _ in 0..3 {
        stats_cache.get_detailed_stats(&"page2".to_string());
    }
    for _ in 0..1 {
        stats_cache.get_detailed_stats(&"page3".to_string());
    }

    println!("詳細統計の例:");
    println!("  ヒット率: {:.2}%", stats_cache.cache.hit_rate() * 100.0);

    let most_accessed = stats_cache.most_accessed_keys(3);
    println!("  最頻アクセス: {:?}", most_accessed);

    let least_accessed = stats_cache.least_accessed_keys(3);
    println!("  最少アクセス: {:?}", least_accessed);

    for key in ["page1", "page2", "page3"] {
        if let Some(last_access) = stats_cache.get_last_access(&key.to_string()) {
            println!(
                "  {}: 頻度={}, 最終アクセス={:.2}秒前",
                key,
                stats_cache.get_frequency(&key.to_string()),
                last_access.elapsed().as_secs_f64()
            );
        }
    }
}

// タイムベースキャッシュ
fn time_based_cache_demo() {
    println!("\n【タイムベースキャッシュ】");

    // タイムベースキャッシュのテスト
    let mut timed_cache = TimedCache::new(5, Duration::from_millis(100));

    println!("タイムベースキャッシュのテスト:");

    timed_cache
        .set("temp1".to_string(), "data1".to_string())
        .unwrap();
    timed_cache
        .set("temp2".to_string(), "data2".to_string())
        .unwrap();

    println!("  初期データ設定完了");

    // 即座のアクセス
    if let Some(value) = timed_cache.get_with_expiry(&"temp1".to_string()) {
        println!("  即座のアクセス成功: {}", value);
    }

    // 少し待つ（有効期限内）
    std::thread::sleep(Duration::from_millis(50));
    if let Some(value) = timed_cache.get_with_expiry(&"temp1".to_string()) {
        println!("  50ms後のアクセス成功: {}", value);
    }

    // 有効期限切れまで待つ
    std::thread::sleep(Duration::from_millis(60));
    if timed_cache.get_with_expiry(&"temp1".to_string()).is_none() {
        println!("  110ms後のアクセス失敗（期限切れ）");
    }

    println!("  ヒット率: {:.2}%", timed_cache.hit_rate() * 100.0);
    println!("  期限切れ数: {}", timed_cache.expiration_count());
}

// マルチレベルキャッシュ
fn multi_level_cache_demo() {
    println!("\n【マルチレベルキャッシュ】");

    struct MultiLevelCache<K, V> {
        l1_cache: LRUCache<K, V>,  // 高速・小容量
        l2_cache: FIFOCache<K, V>, // 中速・中容量
        total_hits: u64,
        total_misses: u64,
        l1_hits: u64,
        l2_hits: u64,
    }

    impl<K, V> MultiLevelCache<K, V>
    where
        K: Clone + Hash + Eq + Debug,
        V: Clone + Debug,
    {
        fn new(l1_capacity: usize, l2_capacity: usize) -> Self {
            MultiLevelCache {
                l1_cache: LRUCache::new(l1_capacity),
                l2_cache: FIFOCache::new(l2_capacity),
                total_hits: 0,
                total_misses: 0,
                l1_hits: 0,
                l2_hits: 0,
            }
        }

        fn get_multilevel(&mut self, key: &K) -> Option<V> {
            // L1キャッシュをチェック
            if let Some(value) = self.l1_cache.get_with_stats(key) {
                self.l1_hits += 1;
                self.total_hits += 1;
                return Some(value.clone());
            }

            // L2キャッシュをチェック
            if let Some(value) = self.l2_cache.get_with_stats(key) {
                self.l2_hits += 1;
                self.total_hits += 1;

                // L2からL1にプロモート
                let value_clone = value.clone();
                let _ = self.l1_cache.set(key.clone(), value_clone.clone());
                return Some(value_clone);
            }

            self.total_misses += 1;
            None
        }

        fn set_multilevel(&mut self, key: K, value: V) -> Result<(), CacheError> {
            // L1に設定を試行
            if let Err(_) = self.l1_cache.set(key.clone(), value.clone()) {
                // L1が満杯の場合、L2に設定
                if let Err(_) = self.l2_cache.set(key, value) {
                    return Err(CacheError::CapacityExceeded);
                }
            }
            Ok(())
        }

        fn l1_hit_rate(&self) -> f64 {
            if self.total_hits + self.total_misses == 0 {
                0.0
            } else {
                self.l1_hits as f64 / (self.total_hits + self.total_misses) as f64
            }
        }

        fn l2_hit_rate(&self) -> f64 {
            if self.total_hits + self.total_misses == 0 {
                0.0
            } else {
                self.l2_hits as f64 / (self.total_hits + self.total_misses) as f64
            }
        }

        fn total_hit_rate(&self) -> f64 {
            if self.total_hits + self.total_misses == 0 {
                0.0
            } else {
                self.total_hits as f64 / (self.total_hits + self.total_misses) as f64
            }
        }
    }

    // マルチレベルキャッシュのテスト
    let mut multi_cache = MultiLevelCache::new(2, 4);

    println!("マルチレベルキャッシュのテスト:");

    // データ設定
    multi_cache
        .set_multilevel("item1".to_string(), "content1".to_string())
        .unwrap();
    multi_cache
        .set_multilevel("item2".to_string(), "content2".to_string())
        .unwrap();
    multi_cache
        .set_multilevel("item3".to_string(), "content3".to_string())
        .unwrap();
    multi_cache
        .set_multilevel("item4".to_string(), "content4".to_string())
        .unwrap();

    println!("  4つのアイテムを設定");

    // アクセスパターンのテスト
    for _ in 0..3 {
        multi_cache.get_multilevel(&"item1".to_string());
    }

    for _ in 0..2 {
        multi_cache.get_multilevel(&"item3".to_string());
    }

    multi_cache.get_multilevel(&"item5".to_string()); // 存在しない

    println!("  L1キャッシュサイズ: {}", multi_cache.l1_cache.len());
    println!("  L2キャッシュサイズ: {}", multi_cache.l2_cache.len());
    println!("  L1ヒット率: {:.2}%", multi_cache.l1_hit_rate() * 100.0);
    println!("  L2ヒット率: {:.2}%", multi_cache.l2_hit_rate() * 100.0);
    println!(
        "  総合ヒット率: {:.2}%",
        multi_cache.total_hit_rate() * 100.0
    );

    println!("\n【キャッシュ設計のポイント】");
    println!("✓ 適切な削除戦略の選択（LRU、FIFO、LFU）");
    println!("✓ 統計情報による性能監視");
    println!("✓ TTL（Time To Live）による自動期限切れ");
    println!("✓ マルチレベル構成による階層化");
    println!("✓ トレイト境界による型安全性の確保");
    println!("✓ エラーハンドリングの適切な実装");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn lru_get_with_stats_counts_hit_and_moves_key_to_back() {
        let mut lru = LRUCache::new(3);
        lru.set(s("a"), 1).unwrap();
        lru.set(s("b"), 2).unwrap();

        assert_eq!(lru.get_with_stats(&s("a")), Some(&1));

        assert_eq!(lru.hit_count(), 1);
        assert_eq!(lru.miss_count(), 0);
        assert_eq!(lru.order, VecDeque::from([s("b"), s("a")]));
    }

    #[test]
    fn lru_get_with_stats_counts_miss_for_absent_key() {
        let mut lru: LRUCache<String, i32> = LRUCache::new(2);

        assert_eq!(lru.get_with_stats(&s("x")), None);

        assert_eq!((lru.hit_count(), lru.miss_count()), (0, 1));
        assert!(lru.order.is_empty());
    }

    #[test]
    fn lru_set_evicts_least_recently_used_when_full() {
        let mut lru = LRUCache::new(2);
        lru.set(s("a"), 1).unwrap();
        lru.set(s("b"), 2).unwrap();
        lru.get_with_stats(&s("a"));

        assert_eq!(lru.set(s("c"), 3), Ok(None));

        assert_eq!(lru.get(&s("b")), None);
        assert_eq!(lru.get(&s("a")), Some(&1));
        assert_eq!(lru.len(), 2);
        assert_eq!(lru.capacity(), 2);
    }

    #[test]
    fn lru_set_existing_key_returns_old_value_and_refreshes_order() {
        let mut lru = LRUCache::new(2);
        lru.set(s("a"), 1).unwrap();
        lru.set(s("b"), 2).unwrap();

        assert_eq!(lru.set(s("a"), 10), Ok(Some(1)));

        assert_eq!(lru.order, VecDeque::from([s("b"), s("a")]));
        assert_eq!(lru.len(), 2);
    }

    #[test]
    fn lru_evict_lru_returns_oldest_pair_or_none_when_empty() {
        let mut lru = LRUCache::new(2);
        assert_eq!(lru.evict_lru(), None::<(String, i32)>);
        lru.set(s("a"), 1).unwrap();
        lru.set(s("b"), 2).unwrap();

        assert_eq!(lru.evict_lru(), Some((s("a"), 1)));
        assert_eq!(lru.len(), 1);
    }

    #[test]
    fn lru_get_mut_remove_clear_and_get_or_insert_with() {
        let mut lru = LRUCache::new(3);
        lru.set(s("a"), 1).unwrap();
        lru.set(s("b"), 2).unwrap();

        *lru.get_mut(&s("a")).unwrap() += 5;
        assert_eq!(lru.order, VecDeque::from([s("b"), s("a")]));
        assert_eq!(lru.get(&s("a")), Some(&6));
        assert_eq!(lru.get_mut(&s("zz")), None);

        assert_eq!(lru.get_or_insert_with(s("b"), || 99), Ok(&2));
        assert_eq!(lru.order.back(), Some(&s("b")));
        assert_eq!(lru.get_or_insert_with(s("c"), || 3), Ok(&3));

        assert_eq!(lru.remove(&s("a")), Some(6));
        assert!(!lru.order.contains(&s("a")));
        lru.clear();
        assert!(lru.is_empty());
        assert!(lru.order.is_empty());
    }

    #[test]
    fn lru_reset_stats_zeroes_counters() {
        let mut lru: LRUCache<String, i32> = LRUCache::new(1);
        lru.get_with_stats(&s("x"));
        lru.reset_stats();
        assert_eq!((lru.hit_count(), lru.miss_count()), (0, 0));
    }

    #[test]
    fn fifo_evicts_first_inserted_even_after_access() {
        let mut fifo = FIFOCache::new(2);
        fifo.set(s("x"), 1).unwrap();
        fifo.set(s("y"), 2).unwrap();
        assert_eq!(fifo.get_with_stats(&s("x")), Some(&1));
        assert_eq!(fifo.get_with_stats(&s("q")), None);

        fifo.set(s("z"), 3).unwrap();

        assert_eq!(fifo.get(&s("x")), None);
        assert_eq!(fifo.order, VecDeque::from([s("y"), s("z")]));
        assert_eq!((fifo.hit_count(), fifo.miss_count()), (1, 1));
    }

    #[test]
    fn fifo_update_existing_key_keeps_order_and_returns_old() {
        let mut fifo = FIFOCache::new(2);
        fifo.set(s("x"), 1).unwrap();
        fifo.set(s("y"), 2).unwrap();

        assert_eq!(fifo.set(s("x"), 10), Ok(Some(1)));

        assert_eq!(fifo.order, VecDeque::from([s("x"), s("y")]));
        assert_eq!(fifo.len(), 2);
        assert_eq!(fifo.capacity(), 2);
    }

    #[test]
    fn fifo_get_mut_remove_clear_get_or_insert_with_and_reset() {
        let mut fifo = FIFOCache::new(3);
        fifo.set(s("x"), 1).unwrap();
        *fifo.get_mut(&s("x")).unwrap() = 7;
        assert_eq!(fifo.get(&s("x")), Some(&7));

        assert_eq!(fifo.get_or_insert_with(s("x"), || 99), Ok(&7));
        assert_eq!(fifo.get_or_insert_with(s("y"), || 2), Ok(&2));

        assert_eq!(fifo.remove(&s("x")), Some(7));
        assert_eq!(fifo.order, VecDeque::from([s("y")]));
        fifo.clear();
        assert!(fifo.is_empty());
        assert!(fifo.order.is_empty());

        fifo.get_with_stats(&s("none"));
        fifo.reset_stats();
        assert_eq!((fifo.hit_count(), fifo.miss_count()), (0, 0));
    }

    fn timed_with_entry(age: Duration, ttl: Duration) -> TimedCache<String, i32> {
        let mut cache = TimedCache::new(3, ttl);
        cache.data.insert(s("k"), (1, Instant::now() - age));
        cache
    }

    const TTL: Duration = Duration::from_secs(60);
    const OLD: Duration = Duration::from_secs(120);

    #[test]
    fn timed_get_with_expiry_hits_fresh_entry() {
        let mut cache = timed_with_entry(Duration::ZERO, TTL);

        assert_eq!(cache.get_with_expiry(&s("k")), Some(&1));

        assert_eq!((cache.hit_count(), cache.miss_count()), (1, 0));
        assert_eq!(cache.expiration_count(), 0);
    }

    #[test]
    fn timed_get_with_expiry_removes_expired_entry() {
        let mut cache = timed_with_entry(OLD, TTL);

        assert_eq!(cache.get_with_expiry(&s("k")), None);

        assert_eq!((cache.hit_count(), cache.miss_count()), (0, 1));
        assert_eq!(cache.expiration_count(), 1);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn timed_get_with_expiry_counts_miss_for_absent_key() {
        let mut cache = timed_with_entry(Duration::ZERO, TTL);

        assert_eq!(cache.get_with_expiry(&s("none")), None);

        assert_eq!((cache.hit_count(), cache.miss_count()), (0, 1));
        assert_eq!(cache.expiration_count(), 0);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn timed_get_mut_returns_fresh_entry_only() {
        let mut fresh = timed_with_entry(Duration::ZERO, TTL);
        *fresh.get_mut(&s("k")).unwrap() = 5;
        assert_eq!(fresh.get(&s("k")), Some(&5));
        assert_eq!(fresh.get_mut(&s("none")), None);

        let mut stale = timed_with_entry(OLD, TTL);
        assert_eq!(stale.get_mut(&s("k")), None);
        assert_eq!(stale.get(&s("k")), None);
        assert_eq!(stale.get(&s("none")), None);
    }

    #[test]
    fn timed_is_expired_boundary() {
        let cache: TimedCache<String, i32> = TimedCache::new(1, TTL);
        assert!(!cache.is_expired(&Instant::now()));
        assert!(cache.is_expired(&(Instant::now() - OLD)));

        // 未来の時刻は elapsed() が 0 に飽和するため、TTL 0 でちょうど境界になる
        let zero_ttl: TimedCache<String, i32> = TimedCache::new(1, Duration::ZERO);
        assert!(!zero_ttl.is_expired(&(Instant::now() + TTL)));
    }

    #[test]
    fn timed_set_cleans_expired_then_enforces_capacity() {
        let mut cache = timed_with_entry(OLD, TTL);
        cache.capacity = 2;
        cache.set(s("x"), 0).unwrap();

        assert_eq!(cache.set(s("a"), 2), Ok(None));
        assert_eq!(cache.expiration_count(), 1);
        assert_eq!(cache.set(s("a"), 3), Ok(Some(2)));
        assert_eq!(cache.set(s("b"), 4), Err(CacheError::CapacityExceeded));
        assert_eq!(cache.capacity(), 2);
    }

    #[test]
    fn timed_get_or_insert_with_remove_clear_and_reset() {
        let mut cache = timed_with_entry(Duration::ZERO, TTL);

        assert_eq!(cache.get_or_insert_with(s("k"), || 99), Ok(&1));
        assert_eq!(cache.get_or_insert_with(s("n"), || 2), Ok(&2));
        assert_eq!(cache.remove(&s("k")), Some(1));
        assert_eq!(cache.remove(&s("k")), None);
        cache.clear();
        assert!(cache.is_empty());

        cache.reset_stats();
        assert_eq!(
            (cache.hit_count(), cache.miss_count(), cache.expiration_count()),
            (0, 0, 0)
        );
    }

    #[test]
    fn cache_stats_hit_rate_handles_zero_and_mixed() {
        let mut lru: LRUCache<String, i32> = LRUCache::new(1);
        assert_eq!(lru.hit_rate(), 0.0);
        assert!(!lru.is_full());
        lru.set(s("a"), 1).unwrap();
        lru.get_with_stats(&s("a"));
        lru.get_with_stats(&s("b"));
        assert_eq!(lru.hit_rate(), 0.5);
        assert!(lru.is_full());
        assert!(lru.contains_key(&s("a")));
    }
}
