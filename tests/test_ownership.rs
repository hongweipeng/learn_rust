/*
本文件将通过字符串来学习展示所有权

所有权规则:
- Rust 中的每个值都有一个名为owner的变量。
- 一次只能有一个所有者。
- 当所有者超出范围时，该值将被删除。

 */

use std::ops::Index;

#[test]
fn test_ownership_over_scope() {
    // 所有权的第一个示例
    // 当变量超出范围后，变量就不可用了
    {
        let s = "hello";
        assert_eq!("hello", s); // 在范围内，变量可用
    }
    // assert_eq!("hello", s); // 不在范围内，变量不可用
}

#[test]
fn test_ownership_move() {
    let x = 5;  // 将值绑定到 x 上
    let y = x;  // x并将其绑定到y
                     // 因为整数是具有已知、固定大小的简单值，并且这两个5值被压入堆栈。
                     // 内存中会有两个5，而不是把两个变量绑到一个 5 上
    assert_eq!(x, y);

    let s1 = String::from("hello");
    let s2 = s1;    // String 的所有权发生转移， s1 后续不可用

    // println!("{}, world!", s1);
    //                        ^^ value borrowed here after move
    assert_eq!("hello", s2);

    // clone 可以将堆数据复制，而不产生转移问题，可以理解为这是一个深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone();

    assert_eq!(s1, s2);
}

#[test]
fn test_ownership_copy_trait() {
    // 如果一个类型实现了Copytrait，一个变量在赋值给另一个变量后仍然有效。
    // 比如上个例子的整型
    /*
    以下是一些实现的类型Copy：

    - 所有整数类型，例如u32.
    - 布尔类型 ,bool具有值true和false。
    - 所有浮点类型，例如f64.
    - 字符类型，char.
    - 元组，如果它们只包含也实现Copy. 例如， (i32, i32)实现Copy，但(i32, String)没有。
     */
    let x: i32 = 5;
    let y = x;
    assert_eq!(x, y);

    let x = 'T';
    let y = x;
    assert_eq!(x, y);

    let x = (500, 6.4, true);
    let y = x;
    assert_eq!(x, y);
}


fn takes_ownership(_some_string: String) { // some_string comes into scope

}

fn makes_copy(_some_integer: i32) { // some_integer comes into scope

} // Here, some_integer goes out of scope. Nothing special happens.

#[test]
fn test_ownership_into_function() {
    // 将值传递给函数的语义类似于将值分配给变量
    let s = String::from("hello");
    takes_ownership(s);
    // assert_eq!("hello", s);  // s 变量无法使用
    // ^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move

    let x :i32 = 5;
    makes_copy(x); // 类型若实现 copy trait ，变量则可以后续继续使用
    assert_eq!(5, x);
}

fn get_hello_string() -> String {
    let s = String::from("hello");
    return s;   // 作为函数的返回值，所有权发生了转移，s 不会在函数调用结束后被回收
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    return a_string;  // a_string is returned and moves out to the calling function
}

#[test]
fn test_ownership_function_return() {
    let hello = get_hello_string();
    assert_eq!("hello", hello);

    let hello = takes_and_gives_back(hello);
    assert_eq!("hello", hello);
}

// ==== References and Borrowing ====
// ==== 引用与借用 ====

/*
    引用就像一个指针，因为它是一个地址，我们可以按照它来访问存储在该地址上的数据，该地址由其他变量拥有
 */

fn calculate_length(s: &String) -> usize {
    // 参数 s 是个引用，它没有所有权
    s.len()
}

fn string_append_world(s: &mut String) {
    s.push_str(" world");
}

#[test]
fn test_ownership_ref() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    assert_eq!("hello", s1); // 变量还是可以使用
    assert_eq!(5, len);

    let mut s2 = String::from("hello");
    string_append_world(&mut s2);   // 传递可变引用，则函数内可以对其进行修改
    assert_eq!("hello world", s2);
}

fn get_string_ref(s: &String) -> &String {
    return s;
}

#[test]
fn test_ownership_string_ref() {
    let s1 = String::from("hello");
    let s2 = get_string_ref(&s1);
    // 此时，s2 是 s1 的引用，等价于 let s2 = &s1; 两个变量后续代码均可使用

    assert_eq!("hello", s1);
    assert_eq!("hello", s2);
}

#[test]
fn test_ownership_ref_many() {
    // 允许多个可变引用
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &s;
    assert_eq!("hello", r1);
    assert_eq!("hello", r2);
    assert_eq!("hello", r3);
}

#[test]
fn test_ownership_mut_ref() {
    // 可变引用同一时刻只能有一个
    let mut s = String::from("hello");

    let r1 = &mut s;
    assert_eq!("hello", r1);

    let r2 = &mut s;
    assert_eq!("hello", r2);
    // assert_eq!("hello", r1);   // 此行报错，因为可变引用是r2，因此 r1不可用

}

#[test]
fn test_ownership_mut_one_many_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    assert_eq!("hello", r1);
    assert_eq!("hello", r2);

    // 当声明可变引用后，之前的不可变引用变量后续将不可用
    let r3 = &mut s; // BIG PROBLEM
    //assert_eq!("hello", r1);
    //----------------------- immutable borrow later used here
    // assert_eq!("hello", r2);
    //----------------------- immutable borrow later used here
    assert_eq!("hello", r3);
}

#[test]
fn test_ownership_string_slice() {
    // 字符串是可变的
    let mut s = String::from("hello world");
    let hello = &s[0..5];   // 变量指向了s的某部分内容，并没有新创建字符串
    assert_eq!("hello", hello);
    s.clear();  // 当 s 发生修改时，它之前声明的引用变量将在后续失效
    //assert_eq!("hello", hello);
    //-------------------------- immutable borrow later used here
}

