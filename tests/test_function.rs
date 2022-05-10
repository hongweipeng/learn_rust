


#[test]
fn test_function_declare() {
    // 函数的声明通过 fn 关键字，函数参数的类型必须写
    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

    another_function(1, 2);

    // 函数的返回值类型通过 -> 声明
    fn five() -> i32 {
        return 5;
    }
    assert_eq!(5, five());

    // 函数也是 {} 包裹的，因此也是 scope，最有一个语句作为表达式返回值，
    // 可以省略 return 关键字，注意尾部不加分号
    fn six() -> i32 {
        let num = 5;
        num + 1
    }

    assert_eq!(6, six());
}


