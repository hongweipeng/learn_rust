

#[test]
fn test_simple_if() {
    // if 条件里不需要 () 包裹
    let num = 3;
    if num < 5 {
        assert!(true);
    } else {
        assert!(false);
    }

    // if 的条件返回必须是 bool
    //|     if num {
    //|        ^^^ expected `bool`, found integer
}

#[test]
fn test_if_else_if() {
    // rust 中没有 elif 作为缩写，因此还是得写 else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[test]
fn test_if_assign() {
    // if 语句可以作为三目运算符
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    assert_eq!(5, number);
}


#[test]
fn test_loop() {
    // Rust 提供了一个loop关键字来指示无限循环。
    fn function_loop() -> i32{
        let mut count = 0;
        loop {
            count += 1;
            if count == 100 {
                break;
            }
        }
        return count;
    }
    assert_eq!(100, function_loop());
}

#[test]
fn test_nest_loop() {
    // 嵌套 loop
    fn nest_loop() -> i32 {
        let mut count = 0;
        'outer: loop {
            count = 2;
            'inner: loop {
                count = 3;
                break 'outer;   // 退出 'outer 的循环
            }
            count = 4; // 这行不会执行
        }
        return count;
    }

    assert_eq!(3, nest_loop());
}

#[test]
fn test_return_from_loop() {
    // break 后面可以接表达式作为 loop 语句的返回值
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    assert_eq!(20, result);
}

#[test]
fn test_while() {
    let mut count = 10;
    while count > 0 {
        count -= 1;
    }
    assert_eq!(0, count);
}

#[test]
fn test_for() {
    // rust 的for
    let nums = [10, 20, 30, 40, 50];
    let mut i = 0;
    for element in nums.iter() {
        assert_eq!(*element, nums[i]);
        i += 1;
    }
}

#[test]
fn test_range() {
    // range 可以快速控制循环次数
    let mut num = 0;

    // n 将取 1, 2, 3, 4, ..., 99 注意，没有取到100
    for n in 1..100 {
        num = n;
    }
    assert_eq!(99, num);

    // 若要取到可以 a..-b 的形式
    for n in 1..=100 {
        num = n;
    }
    assert_eq!(100, num);
}

