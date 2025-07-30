use serde_json::de;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    // let a = Box::new(list::Cons(5, Box::new(list::Nil)));
    // RefCell可实现 Interior Mutability内部可变性
    let a = 5;
    let b: Rc<RefCell<i32>> = Rc::new(RefCell::new(a));
    *b.borrow_mut() = 10;
    println!("{:?}", b.borrow());

    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let data1 = Rc::clone(&shared_data);
}

/**
 * Cons变体：表示链表节点
第一个参数i32：存储当前节点的数值
第二个参数Box<list>：指向下一个节点的堆内存指针
Nil变体：表示链表终止节点（空节点
 */
#[derive(Debug)]
enum list {
    Cons(i32, Box<list>),
    Nil,
}

#[derive(Debug)]
enum ListNode {
    Cons(i32, Rc<ListNode>),
    Nil,
}

/**
 * 双向链表
 */
#[derive(Debug)]
struct Node {
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[test]
fn weak_reference() {
    let a: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
        value: 1,
        prev: None,
        next: None,
    }));
    let b: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
        value: 2,
        prev: Some(a.clone()),
        next: None,
    }));
    a.borrow_mut().next = Some(b.clone());

    // 循环引用：a.next -> b，b.prev -> a
    // 无法释放内存！
}

use std::rc::Weak;
/**
 * 实现一个简单的社交网络
 */
struct User {
    username: String,
    friends: RefCell<Vec<Weak<RefCell<User>>>>,
}

impl User {
    fn new(name: String) -> Rc<RefCell<User>> {
        Rc::new(RefCell::new(User {
            username: name,
            friends: RefCell::new(vec![]),
        }))
    }

    fn show_friends(&self) {
        print!("{}的朋友：", self.username);
        for weak_friend in self.friends.borrow().iter() {
            if let Some(friend) = weak_friend.upgrade() {
                print!("{} ", friend.borrow().username);
            }
        }
        println!();
    }
}
fn add_friend(this: Rc<RefCell<User>>, other: Rc<RefCell<User>>) {
    this.borrow_mut()
        .friends
        .borrow_mut()
        .push(Rc::downgrade(&other));
    other
        .borrow_mut()
        .friends
        .borrow_mut()
        .push(Rc::downgrade(&this));
}
#[test]
fn test_social_network() {
    let alice: Rc<RefCell<User>> = User::new("Alice".to_string());
    let bob = User::new("Bob".to_string());
    add_friend(Rc::clone(&alice), Rc::clone(&bob));
    alice.borrow().show_friends();
    bob.borrow().show_friends();
}
// 老师的答案
// fn add_friend(this: &Rc<RefCell<User>>, other: &Rc<RefCell<User>>) {
//     this.borrow_mut()
//         .friends
//         .borrow_mut()
//         .push(Rc::downgrade(&other));
//     other
//         .borrow_mut()
//         .friends
//         .borrow_mut()
//         .push(Rc::downgrade(&this));
// }

// fn main(){
//     let alice: Rc<RefCell<User>> = User::new("Alice".to_string());
//     let bob = User::new("Bob".to_string());
//     add_friend(&alice, &bob);
//     alice.borrow().show_friends();
//     bob.borrow().show_friends();
// }
