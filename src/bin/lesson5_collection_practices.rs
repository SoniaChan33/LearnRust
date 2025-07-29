/**
 * 通过vec实现stack
 */
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 初始化空栈
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 入栈：向尾部添加元素
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 出栈：移除并返回尾部元素（空栈时返回 None）
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 查看栈顶：返回尾部元素的引用（空栈时返回 None）
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

// 测试用例
fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("栈顶元素（peek）: {:?}", stack.peek()); // 输出: Some(3)
    println!("出栈元素（pop）: {:?}", stack.pop()); // 输出: Some(3)
    println!("出栈后栈顶: {:?}", stack.peek()); // 输出: Some(2)
}

/**
 * 通过hashmap计算单词频率
 */

fn count_frequency(s: &str) -> HashMap<&str, i32> {
    let mut letters: HashMap<&str, i32> = HashMap::new();
    for ch in s.split_whitespace() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    return letters;
}

#[test]
fn test_count_frequency() {
    let s = "hello world hello rust";
    let letters = count_frequency(s);
    println!("{:?}", letters);
}
use std::collections::HashMap;

/**
 * 使用Vec和HashMap实现一个简单的书籍库存管理系统
 */
use std::fmt;

// 书籍结构体：包含核心信息
#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    quantity: u32,
}

// 为Book实现Display trait，方便打印
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, 书名: {}, 作者: {}, 库存: {}",
            self.id, self.title, self.author, self.quantity
        )
    }
}

// 库存管理系统
struct InventorySystem {
    books: Vec<Book>,                 // 存储所有书籍
    id_to_index: HashMap<u32, usize>, // ID -> Vec索引的映射
    next_id: u32,                     // 下一个可用ID（自增）
}

impl InventorySystem {
    // 初始化空系统
    fn new() -> Self {
        InventorySystem {
            books: Vec::new(),
            id_to_index: HashMap::new(),
            next_id: 1, // ID从1开始
        }
    }

    // 添加书籍：自动分配ID，返回新书籍ID
    fn add_book(&mut self, title: String, author: String, quantity: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let book = Book {
            id,
            title,
            author,
            quantity,
        };
        let index = self.books.len();
        self.books.push(book);
        self.id_to_index.insert(id, index); // 记录ID与索引的映射

        id
    }

    // 按ID查询书籍：返回Option<&Book>（不存在则返回None）
    fn get_book(&self, id: u32) -> Option<&Book> {
        self.id_to_index.get(&id).map(|&index| &self.books[index]) // 通过索引取书籍引用
    }

    // 按标题模糊查询：返回所有包含该标题的书籍
    fn search_by_title(&self, title: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title))
            .collect()
    }

    // 更新库存：按ID修改数量，返回是否成功
    fn update_quantity(&mut self, id: u32, new_quantity: u32) -> bool {
        if let Some(&index) = self.id_to_index.get(&id) {
            self.books[index].quantity = new_quantity;
            true
        } else {
            false
        }
    }

    // 删除书籍：按ID删除，返回是否成功
    // （注：删除时将最后一本书移到被删位置，保证Vec索引一致性）
    fn remove_book(&mut self, id: u32) -> bool {
        if let Some(index) = self.id_to_index.remove(&id) {
            // 若删除的不是最后一本书，将最后一本书移到删除位置
            if index < self.books.len() - 1 {
                let last_book = self.books.pop().unwrap(); // 取出最后一本书
                self.books[index] = last_book.clone(); // 覆盖到删除位置
                self.id_to_index.insert(last_book.id, index); // 更新最后一本书的索引映射
            } else {
                self.books.pop(); // 删除最后一本书，无需调整映射
            }
            true
        } else {
            false
        }
    }
}

// 测试用例
#[test]
fn test_inventory_system() {
    let mut inventory = InventorySystem::new();

    // 添加书籍
    let id1 = inventory.add_book("Rust编程入门".to_string(), "张三".to_string(), 10);
    let id2 = inventory.add_book("Effective Rust".to_string(), "李四".to_string(), 5);

    // 查询书籍
    println!("书籍1: {}", inventory.get_book(id1).unwrap());
    println!("书籍2: {}", inventory.get_book(id2).unwrap());

    // 更新库存
    inventory.update_quantity(id1, 20);
    println!("更新后书籍1: {}", inventory.get_book(id1).unwrap());

    // 模糊查询
    let results = inventory.search_by_title("Rust");
    println!("含'Rust'的书籍:");
    for book in results {
        println!("{}", book);
    }

    // 删除书籍
    inventory.remove_book(id2);
    println!("删除书籍2后，查询是否存在: {:?}", inventory.get_book(id2));
}
