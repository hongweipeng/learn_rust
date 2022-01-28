
// 类型声明
#[test]
fn test_var_declare() {
    // 默认方式来声明类型
    let default_var_int = 32; // 整型默认为 `i32` 类型，浮点型默认为 `f64` 类型
    let default_var_bool = true;
    assert_eq!(32, default_var_int);
    assert_eq!(true, default_var_bool);

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 变量都能够显式地给出类型说明（type annotation）
    let logical: bool = true;

    // 变量可以通过后缀 （suffix）方式给出类型声明。
    let an_integer   = 5i32; // 后缀说明

    // 可变的（mutable）变量需要加 `mut` 表明其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。ps: 变量 `mutable` 上文已经定义
    let mutable = true;
    assert!(mutable);

}

// 常量声明通过 const 或 static 关键字
// 常量声明的类型不能省略

#[test]
fn test_const_declare() {
    /*
    常量的赋值只能是字面常量，就是在编译阶段就能确定的值
     */

    const NUM : i32 = 5;

    // 常量不能重新赋值
    // NUM = 9;    // cannot assign to this expression

    // 常量类型不能省略
    // const AGE = 5;  // provide a type for the constant:

    // 常量不能被遮蔽
    // const NUM:f64=200.0;//error[E0428]: the name `NUM` is defined multiple times

    let arr = [3, 3, 3, 3];
    // const COUNT : i32 = arr[1]; // arr[1] 是运行阶段才能确定的值，non-constant value

}

#[test]
fn test_static_declare() {
    /*
    全局变量(static),在整个程序中，全局变量只有一个实例，也就是说所有的引用都会指向一个相同的地址。
     */
    static NUM: i32 = 100;

    // 全局变量可以定义为可变的(mut)
    static mut COUNT : i32 = 0;

    // 多个线程同时访问的情况，因而引发内存不安全的问题
    // 变量的访问和修改代码就必须在unsafe块中进行定义
    unsafe {
        COUNT += 1;
        println!("{}", COUNT);
    }

    unsafe {
        assert_ne!(0, COUNT);
    }

}

