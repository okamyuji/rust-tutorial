//! 復習問題3：型安全な単位系
//! 
//! コンパイル時に単位の整合性をチェックする型システムをマクロで実装します。

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

// 単位の次元を表すトレイト
pub trait Dimension {
    const NAME: &'static str;
}

// 基本的な単位を定義するマクロ
macro_rules! define_unit {
    ($name:ident, $display:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
        
        impl Dimension for $name {
            const NAME: &'static str = $display;
        }
    };
}

// 複合単位を定義するマクロ
macro_rules! define_composite_unit {
    ($name:ident = $unit1:ident * $unit2:ident, $display:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
        
        impl Dimension for $name {
            const NAME: &'static str = $display;
        }
        
        impl std::ops::Mul<Quantity<$unit2>> for Quantity<$unit1> {
            type Output = Quantity<$name>;
            
            fn mul(self, rhs: Quantity<$unit2>) -> Self::Output {
                Quantity {
                    value: self.value * rhs.value,
                    _unit: PhantomData,
                }
            }
        }
    };
    
    ($name:ident = $unit1:ident / $unit2:ident, $display:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
        
        impl Dimension for $name {
            const NAME: &'static str = $display;
        }
        
        impl std::ops::Div<Quantity<$unit2>> for Quantity<$unit1> {
            type Output = Quantity<$name>;
            
            fn div(self, rhs: Quantity<$unit2>) -> Self::Output {
                Quantity {
                    value: self.value / rhs.value,
                    _unit: PhantomData,
                }
            }
        }
    };
}

// 量を表す構造体
#[derive(Debug, Clone, Copy)]
pub struct Quantity<U: Dimension> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: Dimension> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Quantity {
            value,
            _unit: PhantomData,
        }
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
}

// 同じ単位同士の演算
impl<U: Dimension> Add for Quantity<U> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

impl<U: Dimension> Sub for Quantity<U> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value - rhs.value)
    }
}

// スカラーとの演算
impl<U: Dimension> Mul<f64> for Quantity<U> {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Quantity::new(self.value * rhs)
    }
}

impl<U: Dimension> Div<f64> for Quantity<U> {
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self::Output {
        Quantity::new(self.value / rhs)
    }
}

// 表示の実装
impl<U: Dimension> std::fmt::Display for Quantity<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, U::NAME)
    }
}

// 単位定義マクロを使った便利な関数生成
macro_rules! quantity_functions {
    ($($unit:ident => $fn_name:ident),* $(,)?) => {
        $(
            pub fn $fn_name(value: f64) -> Quantity<$unit> {
                Quantity::new(value)
            }
        )*
    };
}

// 基本単位の定義
define_unit!(Meter, "m");
define_unit!(Second, "s");
define_unit!(Kilogram, "kg");
define_unit!(Ampere, "A");
define_unit!(Kelvin, "K");

// 複合単位の定義
define_composite_unit!(MeterPerSecond = Meter / Second, "m/s");
define_composite_unit!(MeterPerSecondSquared = MeterPerSecond / Second, "m/s²");
define_composite_unit!(Newton = Kilogram * MeterPerSecondSquared, "N");
define_composite_unit!(Joule = Newton * Meter, "J");
define_composite_unit!(Watt = Joule / Second, "W");

// 既存の単位同士の積（define_composite_unit! は新しい単位型を定義するため、既存の Meter などには使えない）
macro_rules! impl_unit_product {
    ($unit1:ident * $unit2:ident => $out:ident) => {
        impl std::ops::Mul<Quantity<$unit2>> for Quantity<$unit1> {
            type Output = Quantity<$out>;

            fn mul(self, rhs: Quantity<$unit2>) -> Self::Output {
                Quantity {
                    value: self.value * rhs.value,
                    _unit: PhantomData,
                }
            }
        }
    };
}

impl_unit_product!(MeterPerSecond * Second => Meter);
impl_unit_product!(MeterPerSecondSquared * Second => MeterPerSecond);

// 便利な関数の生成
quantity_functions! {
    Meter => meters,
    Second => seconds,
    Kilogram => kilograms,
    Ampere => amperes,
    Kelvin => kelvins,
}

// より高度な単位変換マクロ
macro_rules! define_conversion {
    ($from:ident -> $to:ident: $factor:expr, $method_name:ident) => {
        impl Quantity<$from> {
            pub fn $method_name(self) -> Quantity<$to> {
                Quantity::new(self.value * $factor)
            }
        }
    };
}

// 派生単位の定義
define_unit!(Centimeter, "cm");
define_unit!(Kilometer, "km");
define_unit!(Hour, "h");
define_unit!(Minute, "min");

// 単位変換の定義
define_conversion!(Kilometer -> Meter: 1000.0, to_meter);
define_conversion!(Meter -> Kilometer: 0.001, to_kilometer);
define_conversion!(Centimeter -> Meter: 0.01, to_meter);
define_conversion!(Meter -> Centimeter: 100.0, to_centimeter);
define_conversion!(Hour -> Second: 3600.0, to_second);
define_conversion!(Minute -> Second: 60.0, to_second);

// 物理計算の例
fn calculate_kinetic_energy(mass: Quantity<Kilogram>, velocity: Quantity<MeterPerSecond>) -> Quantity<Joule> {
    // E = 1/2 * m * v²
    // 注：v² の単位 m²/s² は型として定義していないため、数値で計算する
    let speed = velocity.value();
    let energy_value = 0.5 * mass.value() * speed * speed;
    Quantity::new(energy_value)
}

fn calculate_power(energy: Quantity<Joule>, time: Quantity<Second>) -> Quantity<Watt> {
    energy / time
}

fn main() {
    println!("=== 復習問題3：型安全な単位系 ===\n");

    // 基本的な使用例
    println!("--- 基本的な単位 ---");
    let distance = meters(100.0);
    let time = seconds(10.0);
    let mass = kilograms(5.0);
    
    println!("距離: {}", distance);
    println!("時間: {}", time);
    println!("質量: {}", mass);

    // 同じ単位の演算
    println!("\n--- 同じ単位の演算 ---");
    let d1 = meters(50.0);
    let d2 = meters(30.0);
    let total_distance = d1 + d2;
    let difference = d1 - d2;
    
    println!("{} + {} = {}", d1, d2, total_distance);
    println!("{} - {} = {}", d1, d2, difference);

    // 速度の計算
    println!("\n--- 速度の計算 ---");
    let velocity = distance / time;
    println!("速度 = {} / {} = {}", distance, time, velocity);

    // 加速度の計算
    println!("\n--- 加速度の計算 ---");
    let acceleration = velocity / time;
    println!("加速度 = {} / {} = {}", velocity, time, acceleration);

    // 力の計算（F = ma）
    println!("\n--- 力の計算 ---");
    let force = mass * acceleration;
    println!("力 = {} × {} = {}", mass, acceleration, force);

    // エネルギーの計算
    println!("\n--- エネルギーの計算 ---");
    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!("運動エネルギー = {}", kinetic_energy);

    // パワーの計算
    println!("\n--- パワーの計算 ---");
    let power = calculate_power(kinetic_energy, time);
    println!("パワー = {} / {} = {}", kinetic_energy, time, power);

    // 単位変換
    println!("\n--- 単位変換 ---");
    let km = Quantity::<Kilometer>::new(1.5);
    let m = km.to_meter();
    println!("{} = {}", km, m);

    let cm = Quantity::<Centimeter>::new(250.0);
    let m2 = cm.to_meter();
    println!("{} = {}", cm, m2);

    let hours = Quantity::<Hour>::new(2.5);
    let secs = hours.to_second();
    println!("{} = {}", hours, secs);

    // コンパイルエラーの例（コメントアウト）
    println!("\n--- 型安全性の例（コンパイルエラー） ---");
    println!("以下はコンパイルエラーになります：");
    println!("// let invalid = distance + time;  // 異なる単位は加算できない");
    println!("// let invalid = distance + mass;   // 異なる単位は加算できない");
    
    // スカラー演算
    println!("\n--- スカラーとの演算 ---");
    let doubled = distance * 2.0;
    let halved = distance / 2.0;
    println!("{} × 2 = {}", distance, doubled);
    println!("{} ÷ 2 = {}", halved, halved);

    // 複雑な計算の例
    println!("\n--- 複雑な計算 ---");
    let initial_velocity = meters(0.0) / seconds(1.0);
    let final_velocity = meters(20.0) / seconds(1.0);
    let time_elapsed = seconds(5.0);
    
    let avg_acceleration = (final_velocity - initial_velocity) / time_elapsed;
    println!("平均加速度: {}", avg_acceleration);

    let distance_traveled = initial_velocity * time_elapsed + 
                           avg_acceleration * time_elapsed * time_elapsed * 0.5;
    println!("移動距離: {}", distance_traveled);

    println!("\n型安全な単位系の実装完了！");
    println!("この実装により、物理計算で単位の間違いをコンパイル時に検出できます。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_units() {
        let d = meters(10.0);
        assert_eq!(d.value(), 10.0);
        
        let t = seconds(5.0);
        assert_eq!(t.value(), 5.0);
    }

    #[test]
    fn test_addition() {
        let d1 = meters(5.0);
        let d2 = meters(3.0);
        let sum = d1 + d2;
        assert_eq!(sum.value(), 8.0);
    }

    #[test]
    fn test_velocity() {
        let d = meters(100.0);
        let t = seconds(10.0);
        let v = d / t;
        assert_eq!(v.value(), 10.0);
    }

    #[test]
    fn test_conversions() {
        let km = Quantity::<Kilometer>::new(1.0);
        let m = km.to_meter();
        assert_eq!(m.value(), 1000.0);

        let h = Quantity::<Hour>::new(1.0);
        let s = h.to_second();
        assert_eq!(s.value(), 3600.0);
    }

    #[test]
    fn velocity_times_time_is_distance_and_acceleration_times_time_is_velocity() {
        let distance: Quantity<Meter> = (meters(20.0) / seconds(2.0)) * seconds(3.0);
        assert_eq!(distance.value(), 30.0);

        let accel = meters(8.0) / seconds(1.0) / seconds(2.0);
        let velocity: Quantity<MeterPerSecond> = accel * seconds(5.0);
        assert_eq!(velocity.value(), 20.0);
    }

    #[test]
    fn test_kinetic_energy() {
        let m = kilograms(2.0);
        let v = meters(10.0) / seconds(1.0);
        let ke = calculate_kinetic_energy(m, v);
        assert_eq!(ke.value(), 100.0); // 1/2 * 2 * 10^2 = 100
    }
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
