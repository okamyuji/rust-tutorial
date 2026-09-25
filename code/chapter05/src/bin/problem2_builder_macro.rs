//! 復習問題2：ビルダーパターンマクロ
//! 
//! 構造体に対してビルダーパターンを自動生成するマクロを作成します。

// ビルダーパターンを生成するマクロ
// macro_rules! は識別子を連結できないため、ビルダー名は呼び出し側で `=> UserBuilder` と渡す
macro_rules! create_builder {
    (
        $(#[$struct_meta:meta])*
        struct $struct_name:ident => $builder_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_name:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        // 元の構造体
        $(#[$struct_meta])*
        pub struct $struct_name {
            $(
                $(#[$field_meta])*
                pub $field_name : $field_type
            ),*
        }

        #[doc = concat!("Builder for ", stringify!($struct_name))]
        #[derive(Default, Debug)]
        pub struct $builder_name {
            $(
                $field_name : Option<$field_type>
            ),*
        }

        // 元の構造体の実装
        impl $struct_name {
            #[doc = concat!("Create a new builder for ", stringify!($struct_name))]
            pub fn builder() -> $builder_name {
                $builder_name::default()
            }
        }

        // ビルダーの実装  
        impl $builder_name {
            $(
                #[doc = concat!("Set the ", stringify!($field_name), " field")]
                pub fn $field_name(mut self, value: $field_type) -> Self {
                    self.$field_name = Some(value);
                    self
                }
            )*

            #[doc = concat!("Build the ", stringify!($struct_name))]
            pub fn build(self) -> Result<$struct_name, BuilderError> {
                Ok($struct_name {
                    $(
                        $field_name: self.$field_name
                            .ok_or_else(|| BuilderError::MissingField(stringify!($field_name)))?
                    ),*
                })
            }
        }
    };
}

// デフォルト値付きビルダーマクロ
macro_rules! builder_with_defaults {
    (
        struct $name:ident => $builder_name:ident {
            $(
                $field:ident : $type:ty = $default:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }

        #[doc = concat!("Builder for ", stringify!($name))]
        pub struct $builder_name {
            $(
                $field: $type
            ),*
        }

        impl Default for $builder_name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default
                    ),*
                }
            }
        }

        impl $name {
            pub fn builder() -> $builder_name {
                $builder_name::default()
            }
        }

        impl $builder_name {
            $(
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = value;
                    self
                }
            )*

            pub fn build(self) -> $name {
                $name {
                    $(
                        $field: self.$field
                    ),*
                }
            }
        }
    };
}

// エラー型
#[derive(Debug, Clone)]
pub enum BuilderError {
    MissingField(&'static str),
    ValidationFailed(&'static str, &'static str),
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::MissingField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            BuilderError::ValidationFailed(field, msg) => {
                write!(f, "Validation failed for field '{}': {}", field, msg)
            }
        }
    }
}

impl std::error::Error for BuilderError {}

// 使用例のための構造体定義
create_builder! {
    #[derive(Debug)]
    struct User => UserBuilder {
        name: String,
        email: String,
        age: u32,
        active: bool,
    }
}

builder_with_defaults! {
    struct Config => ConfigBuilder {
        host: String = "localhost".to_string(),
        port: u16 = 8080,
        debug: bool = false,
        timeout: u64 = 30,
    }
}

// 検証付きビルダー（手動実装）
#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub category: String,
}

#[derive(Default)]
pub struct ProductBuilder {
    name: Option<String>,
    price: Option<f64>,
    quantity: Option<u32>,
    category: Option<String>,
}

impl Product {
    pub fn builder() -> ProductBuilder {
        ProductBuilder::default()
    }
}

impl ProductBuilder {
    pub fn name(mut self, value: String) -> Result<Self, BuilderError> {
        if value.is_empty() {
            return Err(BuilderError::ValidationFailed("name", "Name cannot be empty"));
        }
        self.name = Some(value);
        Ok(self)
    }

    pub fn price(mut self, value: f64) -> Result<Self, BuilderError> {
        if value <= 0.0 {
            return Err(BuilderError::ValidationFailed("price", "Price must be positive"));
        }
        self.price = Some(value);
        Ok(self)
    }

    pub fn quantity(mut self, value: u32) -> Result<Self, BuilderError> {
        if value == 0 {
            return Err(BuilderError::ValidationFailed("quantity", "Quantity must be greater than 0"));
        }
        self.quantity = Some(value);
        Ok(self)
    }

    pub fn category(mut self, value: String) -> Result<Self, BuilderError> {
        self.category = Some(value);
        Ok(self)
    }

    pub fn build(self) -> Result<Product, BuilderError> {
        Ok(Product {
            name: self.name.ok_or_else(|| BuilderError::MissingField("name"))?,
            price: self.price.ok_or_else(|| BuilderError::MissingField("price"))?,
            quantity: self.quantity.ok_or_else(|| BuilderError::MissingField("quantity"))?,
            category: self.category.ok_or_else(|| BuilderError::MissingField("category"))?,
        })
    }
}

// 実際の使用例を示す関数
fn demonstrate_builders() -> Result<(), Box<dyn std::error::Error>> {
    // 基本的なビルダー
    let user = User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .active(true)
        .build()?;
    
    println!("ユーザー: {:?}", user);

    // デフォルト値付きビルダー
    let config = Config::builder()
        .host("example.com".to_string())
        .debug(true)
        .build();
    
    println!("設定: {:?}", config);

    // 検証付きビルダー
    let product = Product::builder()
        .name("ノートPC".to_string())?
        .price(1299.99)?
        .quantity(10)?
        .category("電子機器".to_string())?
        .build()?;
    
    println!("商品: {:?}", product);

    Ok(())
}

fn main() {
    println!("=== 復習問題2：ビルダーパターンマクロ ===\n");

    // 基本的なビルダーの使用
    println!("--- 基本的なビルダー ---");
    match User::builder()
        .name("Bob".to_string())
        .email("bob@example.com".to_string())
        .age(25)
        .active(false)
        .build() 
    {
        Ok(user) => println!("✓ ユーザー作成成功: {:?}", user),
        Err(e) => println!("✗ エラー: {}", e),
    }

    // 必須フィールドが欠けている場合
    println!("\n--- エラーケース：必須フィールド欠落 ---");
    match User::builder()
        .name("Charlie".to_string())
        .email("charlie@example.com".to_string())
        // ageとactiveが欠落
        .build() 
    {
        Ok(_) => println!("✗ 予期しない成功"),
        Err(e) => println!("✓ 期待通りのエラー: {}", e),
    }

    // デフォルト値の使用
    println!("\n--- デフォルト値付きビルダー ---");
    let config = Config::builder()
        .host("api.example.com".to_string())
        .build();
    
    println!("✓ 設定作成成功:");
    println!("  Host: {}", config.host);
    println!("  Port: {} (デフォルト)", config.port);
    println!("  Debug: {} (デフォルト)", config.debug);
    println!("  Timeout: {} (デフォルト)", config.timeout);

    // 検証付きビルダーの成功例
    println!("\n--- 検証付きビルダー：成功 ---");
    match Product::builder()
        .name("スマートフォン".to_string())
        .and_then(|b| b.price(799.99))
        .and_then(|b| b.quantity(50))
        .and_then(|b| b.category("電子機器".to_string()))
        .and_then(|b| b.build())
    {
        Ok(product) => println!("✓ 商品作成成功: {:?}", product),
        Err(e) => println!("✗ エラー: {}", e),
    }

    // 検証失敗の例
    println!("\n--- 検証付きビルダー：検証失敗 ---");
    match Product::builder()
        .name("".to_string()) // 空の名前は無効
    {
        Ok(_) => println!("✗ 予期しない成功"),
        Err(e) => println!("✓ 期待通りの検証エラー: {}", e),
    }

    match Product::builder()
        .name("無効な商品".to_string())
        .and_then(|b| b.price(-10.0)) // 負の価格は無効
    {
        Ok(_) => println!("✗ 予期しない成功"),
        Err(e) => println!("✓ 期待通りの検証エラー: {}", e),
    }

    // 総合的なデモ
    println!("\n--- 総合デモ ---");
    match demonstrate_builders() {
        Ok(()) => println!("✓ すべてのビルダーが正常に動作しました"),
        Err(e) => println!("✗ エラー: {}", e),
    }

    println!("\nビルダーパターンマクロの実装完了！");
    println!("このマクロにより、ボイラープレートコードを大幅に削減できます。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_builder() {
        let user = User::builder()
            .name("Test User".to_string())
            .email("test@example.com".to_string())
            .age(20)
            .active(true)
            .build()
            .unwrap();

        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, 20);
    }

    #[test]
    fn test_missing_field() {
        let result = User::builder()
            .name("Test".to_string())
            .build();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_default_values() {
        let config = Config::builder().build();
        
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.debug, false);
    }

    #[test]
    fn builder_types_use_names_given_by_caller() {
        let user_builder: UserBuilder = User::builder();
        let config: Config = ConfigBuilder::default().port(9090).timeout(5).build();

        assert!(user_builder.email("a@example.com".to_string()).build().is_err());
        assert_eq!((config.port, config.timeout), (9090, 5));
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_validation() {
        // 無効な名前
        let result = Product::builder().name("".to_string());
        assert!(result.is_err());

        // 無効な価格
        let result = Product::builder()
            .name("Test".to_string())
            .and_then(|b| b.price(-1.0));
        assert!(result.is_err());
    }
}
