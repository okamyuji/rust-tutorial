// src/bin/problem4_smart_pointers.rs
// 復習問題4: スマートポインタの選択の解答

use std::collections::HashMap;

fn main() {
    println!("=== 復習問題4: スマートポインタの選択 ===\n");

    println!("【要件1】グラフ構造で、ノードが複数の他のノードから参照される");
    graph_structure_example();

    println!("\n【要件2】設定データを複数のスレッドで共有したい");
    shared_config_example();

    println!("\n【要件3】親子関係を持つツリー構造を実装したい");
    tree_structure_example();

    println!("\n【スマートポインタ選択ガイド】");
    selection_guide();
}

// 要件1: グラフ構造の実装
fn graph_structure_example() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    // グラフのノード定義
    #[derive(Debug)]
    struct GraphNode {
        id: u32,
        data: String,
        // 子ノードへの強い参照
        children: RefCell<Vec<Rc<GraphNode>>>,
        // 親ノードへの弱い参照（循環参照を防ぐ）
        parent: RefCell<Weak<GraphNode>>,
    }

    impl GraphNode {
        fn new(id: u32, data: String) -> Rc<Self> {
            Rc::new(GraphNode {
                id,
                data,
                children: RefCell::new(Vec::new()),
                parent: RefCell::new(Weak::new()),
            })
        }

        fn add_child(parent: &Rc<GraphNode>, child: &Rc<GraphNode>) {
            parent.children.borrow_mut().push(Rc::clone(child));
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }

        fn get_children(&self) -> Vec<Rc<GraphNode>> {
            self.children.borrow().clone()
        }

        fn get_parent(&self) -> Option<Rc<GraphNode>> {
            self.parent.borrow().upgrade()
        }

        fn print_tree(&self, indent: usize) {
            println!("{}{}: {}", " ".repeat(indent), self.id, self.data);
            for child in self.children.borrow().iter() {
                child.print_tree(indent + 2);
            }
        }
    }

    println!("✅ 選択: Rc<T> + RefCell<T> + Weak<T>");
    println!("理由:");
    println!("・Rc<T>: 複数の所有権を可能にする");
    println!("・RefCell<T>: 内部可変性で動的な構造変更を可能にする");
    println!("・Weak<T>: 循環参照を防ぐ");

    // 使用例
    let root = GraphNode::new(1, "Root".to_string());
    let child1 = GraphNode::new(2, "Child1".to_string());
    let child2 = GraphNode::new(3, "Child2".to_string());
    let grandchild = GraphNode::new(4, "Grandchild".to_string());

    GraphNode::add_child(&root, &child1);
    GraphNode::add_child(&root, &child2);
    GraphNode::add_child(&child1, &grandchild);

    println!("\nグラフ構造:");
    root.print_tree(0);

    // 参照カウント確認
    println!("\n参照カウント:");
    println!("root: {}", Rc::strong_count(&root));
    println!("child1: {}", Rc::strong_count(&child1));
    println!("grandchild: {}", Rc::strong_count(&grandchild));

    // 親への参照確認
    if let Some(parent) = grandchild.get_parent() {
        println!("grandchildの親: {}", parent.data);
    }

    // 子ノードの確認
    let root_children = root.get_children();
    println!("rootの子ノード数: {}", root_children.len());
}

// 要件2: マルチスレッド環境での設定共有
fn shared_config_example() {
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    // 設定構造体
    #[derive(Debug, Clone)]
    struct AppConfig {
        settings: HashMap<String, String>,
        feature_flags: HashMap<String, bool>,
    }

    impl AppConfig {
        fn new() -> Self {
            let mut settings = HashMap::new();
            settings.insert("api_url".to_string(), "https://api.example.com".to_string());
            settings.insert("timeout".to_string(), "30".to_string());

            let mut feature_flags = HashMap::new();
            feature_flags.insert("experimental_feature".to_string(), false);
            feature_flags.insert("debug_mode".to_string(), true);

            AppConfig {
                settings,
                feature_flags,
            }
        }

        fn get_setting(&self, key: &str) -> Option<String> {
            self.settings.get(key).cloned()
        }

        fn set_setting(&mut self, key: String, value: String) {
            self.settings.insert(key, value);
        }

        fn is_feature_enabled(&self, feature: &str) -> bool {
            self.feature_flags.get(feature).copied().unwrap_or(false)
        }

        fn set_feature(&mut self, feature: String, enabled: bool) {
            self.feature_flags.insert(feature, enabled);
        }
    }

    println!("✅ 選択: Arc<RwLock<T>>");
    println!("理由:");
    println!("・Arc<T>: スレッド間での安全な共有");
    println!("・RwLock<T>: 読み取り多数の場合に効率的");
    println!("・複数の読み取り操作を同時実行可能");

    let config = Arc::new(RwLock::new(AppConfig::new()));

    // 読み取り専用スレッド
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let config_clone = Arc::clone(&config);
            thread::spawn(move || {
                for j in 0..3 {
                    let config_read = config_clone.read().unwrap();
                    let api_url = config_read.get_setting("api_url").unwrap_or_default();
                    let debug_enabled = config_read.is_feature_enabled("debug_mode");

                    println!(
                        "Reader {} ({}): API URL = {}, Debug = {}",
                        i, j, api_url, debug_enabled
                    );

                    drop(config_read); // 明示的に読み取りロックを解放
                    thread::sleep(Duration::from_millis(50));
                }
            })
        })
        .collect();

    // 書き込みスレッド
    let writer_handle = {
        let config_clone = Arc::clone(&config);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));

            {
                let mut config_write = config_clone.write().unwrap();
                config_write.set_setting("timeout".to_string(), "60".to_string());
                config_write.set_feature("experimental_feature".to_string(), true);
                println!("Writer: 設定を更新しました");
            } // 書き込みロックを自動解放

            thread::sleep(Duration::from_millis(100));

            {
                let config_read = config_clone.read().unwrap();
                println!(
                    "Writer確認: timeout = {}",
                    config_read.get_setting("timeout").unwrap_or_default()
                );
            }
        })
    };

    // 全スレッドの完了を待機
    for handle in handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();

    // 最終状態確認
    let final_config = config.read().unwrap();
    println!("\n最終設定:");
    println!(
        "  timeout: {}",
        final_config.get_setting("timeout").unwrap_or_default()
    );
    println!(
        "  experimental_feature: {}",
        final_config.is_feature_enabled("experimental_feature")
    );
}

// 要件3: 親子関係のツリー構造
fn tree_structure_example() {
    // シンプルなツリーノード
    #[derive(Debug)]
    struct TreeNode {
        data: i32,
        // Box<T> の使い方を示すためにあえて包む。Vec は要素を既にヒープに置くので、実務では Vec<TreeNode> で足りる
        #[allow(clippy::vec_box)]
        children: Vec<Box<TreeNode>>,
    }

    impl TreeNode {
        fn new(data: i32) -> Self {
            TreeNode {
                data,
                children: Vec::new(),
            }
        }

        fn add_child(&mut self, child: TreeNode) {
            self.children.push(Box::new(child));
        }

        fn print_tree(&self, indent: usize) {
            println!("{}{}", " ".repeat(indent), self.data);
            for child in &self.children {
                child.print_tree(indent + 2);
            }
        }

        // 深さ優先探索
        fn find(&self, target: i32) -> Option<&TreeNode> {
            if self.data == target {
                return Some(self);
            }

            for child in &self.children {
                if let Some(found) = child.find(target) {
                    return Some(found);
                }
            }

            None
        }

        // ノード数をカウント
        fn count_nodes(&self) -> usize {
            1 + self
                .children
                .iter()
                .map(|child| child.count_nodes())
                .sum::<usize>()
        }

        // 最大深度を計算
        fn max_depth(&self) -> usize {
            if self.children.is_empty() {
                1
            } else {
                1 + self
                    .children
                    .iter()
                    .map(|child| child.max_depth())
                    .max()
                    .unwrap_or(0)
            }
        }
    }

    println!("✅ 選択: Box<T>");
    println!("理由:");
    println!("・再帰的データ構造を実現");
    println!("・単純な所有権関係（親が子を所有）");
    println!("・メモリオーバーヘッドなし");
    println!("・コンパイル時にサイズが決定できない型を格納");

    // 使用例
    let mut root = TreeNode::new(1);

    let mut child1 = TreeNode::new(2);
    child1.add_child(TreeNode::new(4));
    child1.add_child(TreeNode::new(5));

    let mut child2 = TreeNode::new(3);
    child2.add_child(TreeNode::new(6));

    root.add_child(child1);
    root.add_child(child2);

    println!("\nツリー構造:");
    root.print_tree(0);

    println!("\n統計:");
    println!("ノード数: {}", root.count_nodes());
    println!("最大深度: {}", root.max_depth());

    // 検索例
    if let Some(node) = root.find(5) {
        println!("ノード5を発見: {}", node.data);
    }

    // より複雑なツリー（ファイルシステム風）
    #[derive(Debug)]
    struct FileNode {
        name: String,
        is_file: bool,
        size: Option<u64>,
        // Box<T> の使い方を示すためにあえて包む。Vec は要素を既にヒープに置くので、実務では Vec<FileNode> で足りる
        #[allow(clippy::vec_box)]
        children: Vec<Box<FileNode>>,
    }

    impl FileNode {
        fn new_file(name: String, size: u64) -> Self {
            FileNode {
                name,
                is_file: true,
                size: Some(size),
                children: Vec::new(),
            }
        }

        fn new_directory(name: String) -> Self {
            FileNode {
                name,
                is_file: false,
                size: None,
                children: Vec::new(),
            }
        }

        fn add_child(&mut self, child: FileNode) {
            if !self.is_file {
                self.children.push(Box::new(child));
            }
        }

        fn total_size(&self) -> u64 {
            if self.is_file {
                self.size.unwrap_or(0)
            } else {
                self.children.iter().map(|child| child.total_size()).sum()
            }
        }

        fn print_filesystem(&self, indent: usize) {
            let prefix = " ".repeat(indent);
            if self.is_file {
                println!(
                    "{}📄 {} ({} bytes)",
                    prefix,
                    self.name,
                    self.size.unwrap_or(0)
                );
            } else {
                println!("{}📁 {}/", prefix, self.name);
                for child in &self.children {
                    child.print_filesystem(indent + 2);
                }
            }
        }
    }

    println!("\n【ファイルシステムツリーの例】");
    let mut root_dir = FileNode::new_directory("project".to_string());

    let mut src_dir = FileNode::new_directory("src".to_string());
    src_dir.add_child(FileNode::new_file("main.rs".to_string(), 1024));
    src_dir.add_child(FileNode::new_file("lib.rs".to_string(), 2048));

    let mut docs_dir = FileNode::new_directory("docs".to_string());
    docs_dir.add_child(FileNode::new_file("README.md".to_string(), 512));

    root_dir.add_child(src_dir);
    root_dir.add_child(docs_dir);
    root_dir.add_child(FileNode::new_file("Cargo.toml".to_string(), 256));

    root_dir.print_filesystem(0);
    println!("総サイズ: {} bytes", root_dir.total_size());
}

// スマートポインタ選択ガイド
fn selection_guide() {
    println!("┌────────────────────┬──────────────┬────────────────────┐");
    println!("│      要件          │ 推奨ポインタ │      理由          │");
    println!("├────────────────────┼──────────────┼────────────────────┤");
    println!("│ 単一所有権         │ Box<T>       │ シンプル、高速     │");
    println!("│ 再帰構造           │ Box<T>       │ サイズ確定         │");
    println!("│ 複数所有権         │ Rc<T>        │ 参照カウント       │");
    println!("│ 循環参照回避       │ Weak<T>      │ 弱い参照           │");
    println!("│ スレッド間共有     │ Arc<T>       │ アトミック操作     │");
    println!("│ 内部可変性         │ RefCell<T>   │ 実行時借用チェック │");
    println!("│ スレッドセーフ可変 │ Mutex<T>     │ 排他制御           │");
    println!("│ 読み取り多数       │ RwLock<T>    │ 読み取り並行       │");
    println!("└────────────────────┴──────────────┴────────────────────┘");

    println!("\n【決定フローチャート】");
    println!("1. 単一の所有者で十分？");
    println!("   ├─ Yes → 再帰的データ構造？");
    println!("   │        ├─ Yes → Box<T>");
    println!("   │        └─ No → 通常の所有権");
    println!("   └─ No → 複数の所有者が必要");
    println!("            ├─ スレッド間共有？");
    println!("            │   ├─ Yes → Arc<T> (+ Mutex/RwLock)");
    println!("            │   └─ No → Rc<T>");
    println!("            └─ 循環参照の可能性？");
    println!("                ├─ Yes → Weak<T>も併用");
    println!("                └─ No → 上記で十分");

    println!("\n【パフォーマンス比較】");
    println!("作成コスト: Box<T> < Rc<T> < Arc<T>");
    println!("クローンコスト: Box<T>(不可) < Rc<T> < Arc<T>");
    println!("メモリオーバーヘッド: Box<T>(0) = Rc<T>(8B) = Arc<T>(8B)");
    println!("スレッドセーフティ: Box<T>(条件次第) < Rc<T>(×) < Arc<T>(○)");
}

#[cfg(test)]
mod main_smoke_tests {
    #[test]
    fn main_runs_without_panicking() {
        super::main();
    }
}
