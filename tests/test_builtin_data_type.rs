use std::ptr::eq;

/**
 * 原生数据类型
 *
 * 标量类型（scalar type）：
 *  - 有符号整型（signed integers）：i8、i16、i32、i64 和 isize（指针宽度）
 *  - 无符号整型（unsigned integers）： u8、u16、u32、u64 和 usize（指针宽 度）
 *  - 浮点类型（floating point）： f32、f64
 *  - char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
 *  - bool（布尔型）：只能是 true 或 false
 *  - 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组，尽管单元类型的值是个元组，它却并不被认为是复合类型，因为并不包含多个值。
 *
 * 复合类型（compound type）：
 *  - 数组（array）：如 [1, 2, 3]
 *  - 元组（tuple）：如 (1, true)
 */

// 字面量和运算符
/*
 整数 1
 浮点数 1.2
 字符 'a'
 字符串 "abc"
 布尔值 true
 单元类型 ()
 可以用数字、文字或符号之类的 “字面量”（literal）来表示。
 */


// 整型类型
#[test]
fn test_integer_type() {
    /*
    无符号整数类型以 u 开头
    有符号整数类型以 i 开头

    整数类型列表如图
    | Length  | Signed | Unsigned |
    | ------- | ------ | -------- |
    | 8-bit   | i8     | u8       |
    | 16-bit  | i16    | u16      |
    | 32-bit  | i32    | u32      |
    | 64-bit  | i64    | u64      |
    | 128-bit | i128   | u128     |
    | arch    | isize  | usize    |

    isize 和 usize 的位数有计算机架构决定，如果是 64 位计算机，那就是 64 位的。
    另外，通过加前缀 0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示。
     */


    // 为了改善可读性，可以在数值字面量中插入下划线
    let large = 1_000;
    let rate = 0.000_001;
    assert_eq!(1000, large);
    assert_eq!(0.000001, rate);

    let num = 18;   // 会默认使用 i32 类型
    let num : i64 = 18;  // 指定具体类型
    let num = 18i64;    // 除了 byte 类型外，所有字面值都允许使用类型后缀
    let num = 0xff; // 0x 前缀十六进制
    let num = 0o77; // 0o 前缀八进制
    let num = 0b1111_0000;  // 0b 二进制
    let byte = b'A'; // 字节，只能声明为 u8 类型

}

#[test]
fn test_integer_compute() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);


    // 溢出检查
    // 调试模式下的编译阶段会检查出溢出从不能通过
    // 发布模式下的编译则不会检查溢出
    // let num = 1u32 - 2;
    // let num = 4294967296i64 * 4294967296i64 * 4294967296i64;

    // 整数相除, 得到是整数去余数处理
    let num = 64 / 5;
    assert_eq!(12, num);

    // 通过类型转换为浮点类型才能得到浮点数(除数与被除数都要是浮点类型)
    let num =  (64 as f64) / (5 as f64);
    assert_eq!(12.8, num);

    // 除法取余
    let num = 54 % 5;
    assert_eq!(4, num);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

}

#[test]
fn test_float_type(){
    /*
    Rust 有两种基础的浮点类型
    f32, 32位, 单精度
    f64, 64位, 双精度

    f64 是默认类型,在现代 CPU 中,单精度和双精度速度差不多
     */
    let money = 2.0;    // 默认使用 f64
    let money : f32 = 2.0;

    assert_eq!(2.0, money);
}

#[test]
fn test_bool_type() {
    /*
    布尔类型只有两个值：true 和 false
    内存中只占 1 个字节大小
     */
    let t = true;
    let f : bool = false;

    // 布尔逻辑判断使用短路求值的方式
    assert!(t);
    assert!(!f);
    assert!(t && t);
    assert!(f || t);
}

#[test]
fn test_char_type() {
    /*
    char 类型是描述里最基础的单个字符
    字符的字面值使用单引号
    内存占用 4 字节代销
    是 Unicode 标量值，可以是 ascii，拼音，表情等
    范围：U+0000 ~ U+D7FF
         U+E000 ~ U+10FFFF
     */
    let ch = 'z';
    let ch : char = '中';
    let ch : char = '\0';
    let ch : char = '\n';
    let ch : char = '😊';

}

#[test]
fn test_tuple_type() {
    /*
    tuple 可以将多个类型的多个值放在一个类型里
    tuple 的长度是固定的，一旦声明就无法改变
     */
    let empty = (); // 空元组
    let tup = (500, 6.4, true);
    let tup : (i32, f64, bool) = (500, 6.4, true);  // 可以显示声明元素类型

    // 元组里取值
    assert_eq!(500, tup.0);
    assert_eq!(6.4, tup.1);
    assert_eq!(true, tup.2);

    // 元组没有获得其长度的方法
    //tup.len();

    // 元组的元素不能重新赋值
    //tup.0 = 1;

    // 元组可用于拆包
    let (x, y) = (1, 2);
    assert_eq!(1, x);
    assert_eq!(2, y);
    // 拆包左右数量不匹配时，编译报错
    // let (x, y) = (1, 2, 3, 4, 5);
    // 可以使用 _ 作为占位
    let (x, y, _, _, _) = (1, 2, 3, 4, 5);
    assert_eq!(1, x);
    assert_eq!(2, y);

    // 但很长的元组无法打印
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

}

#[test]
fn test_list_type() {
    /*
    数组里的每个元素的类型都是相同的
    数组的长度是固定的,若需要边长,推荐使用 Vector
    数组的索引元素可以重新赋值
     */

    // 数组的声明：在中括号中，各值用逗号分开
    let mut arr = [1, 2, 3, 4, 5];
    // 如果数组里每个元素都相同,可以用 ; + 长度 的方式声明
    let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];

    // 数组取值
    assert_eq!(1, arr[0]);
    assert_eq!(2, arr[1]);

    // 数组元素重新赋值, 需要声明数组是 mut 的
    arr[0] = 100;
    assert_eq!(100, arr[0]);

    // 数组的长度
    assert_eq!(5, arr.len());

    // 越界的下标时,编译会通过,运行会引发致命错误（panic）
    // println!("{}", arr[100]);

}

#[test]
fn test_str_change_to_int() {
    // 字符串转整型
    let guess: u32 = "42".parse().expect("Not a number!");
    assert_eq!(42, guess);
}

#[test]
fn test_array_and_slice() {
    // 数组中的元素类型必须都是一样的
    // 数组是用来声明定长的，若需要变长数组，应该使用 vector

    // 声明固定大小的数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    assert_eq!(0, ys[400]);
    assert_eq!(500, ys.len());

    // slice
    let pick = &xs[1..4]; // 从索引1取到索引4（不包含4） pick = [2, 3, 4];
    assert_eq!(3, pick.len());
    assert_eq!(2, pick[0]);
    assert_eq!(3, pick[1]);
    assert_eq!(4, pick[2]);

}

