

/*
Rust 自定义数据类型主要是通过下面这两个关键字来创建：

    struct： 定义一个结构体（structure）
    enum： 定义一个枚举类型（enumeration）

 */


/*
结构体（structure，缩写成 struct）有 3 种类型，使用 struct 关键字来创建：

    元组结构体（tuple struct），事实上就是具名元组而已。
    经典的 C 语言风格结构体（C struct）。
    单元结构体（unit struct），不带字段，在泛型中很有用。

 */

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[test]
fn test_struct_use() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    assert_eq!(name, peter.name);
    assert_eq!(age, peter.age);
    // 以 Debug 方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };
    // 访问 point 的字段
    assert_eq!(0.3, point.x);
    assert_eq!(0.4, point.y);
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };

    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    assert_eq!(0.1, new_point.x);
    assert_eq!(point.y, new_point.y);

    // 使用 `let` 绑定来解构 point, 将成员赋值给 my_x, my_y
    let Point { x: my_x, y: my_y } = point;
    assert_eq!(my_x, point.x);
    assert_eq!(my_y, point.y);

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    assert_eq!(1, pair.0);
    assert_eq!(0.1, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    assert_eq!(integer, pair.0);
    assert_eq!(decimal, pair.1);
}


/*
    枚举

    enum 关键字允许创建一个从数个不同取值中选其一的枚举类型（enumeration）。任何一个在 struct 中合法的取值在 enum 中也合法。
 */

// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

#[test]
fn test_enum_use() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);


}

#[test]
fn test_enum_alias() {
    // 类型别名
    type WebOperations = WebEvent;
    let pressed = WebOperations::KeyPress('x');
}

#[test]
fn test_enum_use2() {
    // 使用 use 声明的话，就可以不写出名称的完整路径了
    use WebEvent::{PageUnload, PageLoad};
    let pressed = PageLoad;
}

#[test]
fn test_enum_c_style() {
    // enum 的 C 风格用法
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    enum Number {
        Zero,
        One,
        Two,
    }

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    assert_eq!(0, Number::Zero as i32);
    assert_eq!(1, Number::One as i32);

    assert_eq!(0xff0000, Color::Red as i32);
    assert_eq!(0x00ff00, Color::Green as i32);

}

