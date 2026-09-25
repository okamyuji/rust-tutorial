// src/bin/lifetime_generics.rs

use std::fmt::Display;
use std::marker::PhantomData;

// スライスのラッパー
struct SliceWrapper<'a, T> {
    data: &'a [T],
    start: usize,
    end: usize,
}

impl<'a, T> SliceWrapper<'a, T> {
    fn new(data: &'a [T]) -> Self {
        let len = data.len();
        SliceWrapper {
            data,
            start: 0,
            end: len,
        }
    }

    fn slice(&self) -> &'a [T] {
        &self.data[self.start..self.end]
    }

    fn narrow(&mut self, new_start: usize, new_end: usize) {
        self.start += new_start;
        self.end = self.start + new_end.min(self.end - self.start);
    }
}

fn main() {
    println!("=== ライフタイムとジェネリクスの組み合わせ ===\n");

    generic_functions_with_lifetimes();
    generic_structs_with_lifetimes();
    trait_bounds_and_lifetimes();
    associated_types_and_lifetimes();
    complex_generic_patterns();
}

fn generic_functions_with_lifetimes() {
    println!("--- ジェネリック関数とライフタイム ---");

    // 基本的な組み合わせ
    basic_generic_lifetime();

    // 複数の型パラメータとライフタイム
    multiple_generics_and_lifetimes();

    // トレイト境界付き
    generic_with_trait_bounds();
}

fn basic_generic_lifetime() {
    println!("\n[基本的なジェネリックとライフタイム]");

    // ジェネリックな参照を返す関数
    fn first_element<'a, T>(slice: &'a [T]) -> Option<&'a T> {
        slice.first()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(first) = first_element(&numbers) {
        println!("最初の要素: {}", first);
    }

    let strings = vec!["apple", "banana", "cherry"];
    if let Some(first) = first_element(&strings) {
        println!("最初の文字列: {}", first);
    }

    // ジェネリックな最大値検索
    fn find_max<'a, T: PartialOrd>(slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            return None;
        }

        let mut max = &slice[0];
        for item in slice {
            if item > max {
                max = item;
            }
        }
        Some(max)
    }

    if let Some(max) = find_max(&numbers) {
        println!("最大値: {}", max);
    }
}

fn multiple_generics_and_lifetimes() {
    println!("\n[複数のジェネリックとライフタイム]");

    // 複数の型とライフタイムを扱う関数
    fn combine<'a, 'b, T, U>(first: &'a T, second: &'b U) -> String
    where
        T: Display,
        U: Display,
    {
        format!("{} + {}", first, second)
    }

    let num = 42;
    let text = "hello";
    let result = combine(&num, &text);
    println!("結合結果: {}", result);

    // タプルを返す関数
    fn split_at_index<'a, T>(slice: &'a [T], index: usize) -> (&'a [T], &'a [T]) {
        slice.split_at(index.min(slice.len()))
    }

    let data = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_index(&data, 3);
    println!("左側: {:?}, 右側: {:?}", left, right);
}

fn generic_with_trait_bounds() {
    println!("\n[トレイト境界付きジェネリック]");

    // 複数のトレイト境界
    fn process_and_display<'a, T>(value: &'a T) -> String
    where
        T: Display + std::fmt::Debug + Clone,
    {
        let cloned = value.clone();
        format!("Display: {}, Debug: {:?}", value, cloned)
    }

    let number = 42;
    println!("{}", process_and_display(&number));

    // ライフタイム境界付き
    fn longest_with_constraint<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: PartialOrd + Display,
    {
        if x > y {
            println!("{}の方が大きい", x);
            x
        } else {
            println!("{}の方が大きいか等しい", y);
            y
        }
    }

    let result = longest_with_constraint(&10, &20);
    println!("結果: {}", result);
}

fn generic_structs_with_lifetimes() {
    println!("\n--- ジェネリック構造体とライフタイム ---");

    // 基本的なジェネリック構造体
    basic_generic_struct();

    // 複雑なジェネリック構造体
    complex_generic_struct();

    // ライフタイムを持つコンテナ
    lifetime_container_examples();
}

fn basic_generic_struct() {
    println!("\n[基本的なジェネリック構造体]");

    // 単一の型パラメータとライフタイム
    struct Wrapper<'a, T> {
        value: &'a T,
    }

    impl<'a, T> Wrapper<'a, T> {
        fn new(value: &'a T) -> Self {
            Wrapper { value }
        }

        fn get(&self) -> &'a T {
            self.value
        }
    }

    impl<'a, T: Display> Wrapper<'a, T> {
        fn display(&self) {
            println!("Wrapped value: {}", self.value);
        }
    }

    let data = 42;
    let wrapper = Wrapper::new(&data);
    wrapper.display();

    // getメソッドを使用
    let _value = wrapper.get();
    println!("値を取得しました");

    // ペア構造体
    struct Pair<'a, 'b, T, U> {
        first: &'a T,
        second: &'b U,
    }

    impl<'a, 'b, T, U> Pair<'a, 'b, T, U> {
        fn new(first: &'a T, second: &'b U) -> Self {
            Pair { first, second }
        }

        fn swap(&self) -> Pair<'b, 'a, U, T> {
            Pair {
                first: self.second,
                second: self.first,
            }
        }
    }

    let x = 10;
    let y = "text";
    let pair = Pair::new(&x, &y);
    let _swapped = pair.swap();
    println!("元: ({}, {})", pair.first, pair.second);
}

fn complex_generic_struct() {
    println!("\n[複雑なジェネリック構造体]");

    // 制約付きジェネリック構造体
    struct Container<'a, T>
    where
        T: 'a + Clone,
    {
        items: Vec<&'a T>,
        owned: Vec<T>,
    }

    impl<'a, T> Container<'a, T>
    where
        T: 'a + Clone,
    {
        fn new() -> Self {
            Container {
                items: Vec::new(),
                owned: Vec::new(),
            }
        }

        fn add_ref(&mut self, item: &'a T) {
            self.items.push(item);
            self.owned.push(item.clone());
        }

        fn get_refs(&self) -> &[&'a T] {
            &self.items
        }

        fn get_owned(&self) -> &[T] {
            &self.owned
        }
    }

    let data1 = String::from("item1");
    let data2 = String::from("item2");

    let mut container = Container::new();
    container.add_ref(&data1);
    container.add_ref(&data2);

    println!("参照数: {}", container.get_refs().len());
    println!("所有数: {}", container.get_owned().len());
}

fn lifetime_container_examples() {
    println!("\n[ライフタイムを持つコンテナ]");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut wrapper = SliceWrapper::new(&data);

    println!("元のスライス: {:?}", wrapper.slice());
    wrapper.narrow(2, 5);
    println!("狭めたスライス: {:?}", wrapper.slice());
}

fn trait_bounds_and_lifetimes() {
    println!("\n--- トレイト境界とライフタイム ---");

    // トレイトの実装とライフタイム
    trait_implementation_examples();

    // 高階トレイト境界との組み合わせ
    higher_ranked_bounds_with_generics();
}

fn trait_implementation_examples() {
    println!("\n[トレイト実装とライフタイム]");

    // ライフタイム付きトレイト
    trait Process<'a> {
        type Input;
        type Output: 'a;

        fn process(&self, input: Self::Input) -> Self::Output;
    }

    struct StringProcessor;

    impl<'a> Process<'a> for StringProcessor {
        type Input = &'a str;
        type Output = String;

        fn process(&self, input: Self::Input) -> Self::Output {
            input.to_uppercase()
        }
    }

    let processor = StringProcessor;
    let result = processor.process("hello");
    println!("処理結果: {}", result);

    // ジェネリックトレイト実装
    struct GenericProcessor<T> {
        _marker: PhantomData<T>,
    }

    impl<'a, T> Process<'a> for GenericProcessor<T>
    where
        T: Display + 'a,
    {
        type Input = &'a T;
        type Output = String;

        fn process(&self, input: Self::Input) -> Self::Output {
            format!("Processed: {}", input)
        }
    }

    let gen_processor = GenericProcessor::<i32> {
        _marker: PhantomData,
    };
    let result = gen_processor.process(&42);
    println!("ジェネリック処理: {}", result);
}

fn higher_ranked_bounds_with_generics() {
    println!("\n[高階トレイト境界とジェネリクス]");

    // for<'a> を使ったジェネリック関数
    fn apply_to_ref<T, F, R>(value: &T, f: F) -> R
    where
        F: for<'a> Fn(&'a T) -> R,
    {
        f(value)
    }

    let number = 42;
    let result = apply_to_ref(&number, |n| n * 2);
    println!("適用結果: {}", result);

    // より複雑な高階境界
    fn complex_hrtb<T, F>(values: &[T], processor: F) -> Vec<String>
    where
        T: Display,
        F: for<'a> Fn(&'a T) -> String,
    {
        values.iter().map(processor).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let results = complex_hrtb(&numbers, |n| format!("Number: {}", n));
    println!("処理済みリスト: {:?}", results);
}

fn associated_types_and_lifetimes() {
    println!("\n--- 関連型とライフタイム ---");

    // 関連型を持つトレイト
    associated_type_examples();

    // GAT（Generic Associated Types）の概念
    gat_concepts();
}

fn associated_type_examples() {
    println!("\n[関連型の例]");

    // イテレータ風のトレイト
    trait MyIterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct RangeIterator {
        current: i32,
        end: i32,
    }

    impl MyIterator for RangeIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let mut iter = RangeIterator { current: 0, end: 5 };
    while let Some(value) = iter.next() {
        print!("{} ", value);
    }
    println!();
}

fn gat_concepts() {
    println!("\n[GAT概念の実装]");

    // 代替パターンを使用
    trait AlternativeContainer<'a> {
        type Item;

        fn get(&'a self) -> Self::Item;
    }

    struct MyContainer {
        data: String,
    }

    impl<'a> AlternativeContainer<'a> for MyContainer {
        type Item = &'a str;

        fn get(&'a self) -> Self::Item {
            &self.data
        }
    }

    let container = MyContainer {
        data: String::from("コンテナデータ"),
    };
    let item = container.get();
    println!("取得したアイテム: {}", item);
}

fn complex_generic_patterns() {
    println!("\n--- 複雑なジェネリックパターン ---");

    // ファントムデータの活用
    phantom_data_patterns();

    // 型レベルプログラミング
    type_level_programming();
}

fn phantom_data_patterns() {
    println!("\n[PhantomDataパターン]");

    // ライフタイムとジェネリックを追跡
    struct Tagged<'a, T, Tag> {
        value: T,
        _lifetime: PhantomData<&'a ()>,
        _tag: PhantomData<Tag>,
    }

    // タグ型
    struct UserData;
    struct SystemData;

    impl<'a, T> Tagged<'a, T, UserData> {
        fn new_user(value: T) -> Self {
            Tagged {
                value,
                _lifetime: PhantomData,
                _tag: PhantomData,
            }
        }

        fn get_value(&self) -> &T {
            &self.value
        }
    }

    impl<'a, T> Tagged<'a, T, SystemData> {
        fn new_system(value: T) -> Self {
            Tagged {
                value,
                _lifetime: PhantomData,
                _tag: PhantomData,
            }
        }

        fn get_value(&self) -> &T {
            &self.value
        }
    }

    let user_data = Tagged::<'_, _, UserData>::new_user(42);
    let system_data = Tagged::<'_, _, SystemData>::new_system("system");

    println!("ユーザーデータ: {}", user_data.get_value());
    println!("システムデータ: {}", system_data.get_value());
    println!("ユーザーデータとシステムデータを型で区別");
}

fn type_level_programming() {
    println!("\n[型レベルプログラミング]");

    // 型状態パターン
    struct Builder<State> {
        value: String,
        _state: PhantomData<State>,
    }

    struct Empty;
    struct WithData;
    struct Complete;

    impl Builder<Empty> {
        fn new() -> Self {
            Builder {
                value: String::new(),
                _state: PhantomData,
            }
        }

        fn add_data(mut self, data: &str) -> Builder<WithData> {
            self.value.push_str(data);
            Builder {
                value: self.value,
                _state: PhantomData,
            }
        }
    }

    impl Builder<WithData> {
        fn finalize(self) -> Builder<Complete> {
            Builder {
                value: self.value,
                _state: PhantomData,
            }
        }
    }

    impl Builder<Complete> {
        fn get(&self) -> &str {
            &self.value
        }
    }

    let builder = Builder::new().add_data("構築された").finalize();

    println!("ビルダー結果: {}", builder.get());
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}

#[cfg(test)]
mod slice_wrapper_tests {
    use super::SliceWrapper;

    #[test]
    fn narrow_moves_start_and_limits_length_within_current_window() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut wrapper = SliceWrapper::new(&data);

        wrapper.narrow(2, 5);
        assert_eq!(wrapper.slice(), &[3, 4, 5, 6, 7]);

        wrapper.narrow(1, 10);
        assert_eq!(wrapper.slice(), &[4, 5, 6, 7]);
    }
}
