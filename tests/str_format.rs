use std::fmt::{self, Formatter, Display};

/**
打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：
    format!：将格式化文本写到字符串（String）。（译注：字符串是返回值不是参数。）
    print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
    println!: 与 print! 类似，但输出结果追加一个换行符。
    eprint!：与 format! 类似，但将文本输出到标准错误（io::stderr）。
    eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
*/

#[test]
fn test_str_format() {
    let text = format!("{} days", 31);
    println!("{}", text);
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}


#[test]
fn test_struct_debug_format() {
    // 为结构添加 #[derive(Debug)] 属性，使其能有默认输出
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    // 打印
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}


#[test]
fn test_struct_custom_format() {
    struct City {
        name: &'static str,
        // 纬度
        lat: f32,
        // 经度
        lon: f32,
    }
    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
    impl Display for City { // 自定义结构体的输出
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。

            // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
            // 一个缓冲区（即第一个参数f）中。
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    // 打印
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
}


#[test]
fn test_pretty_print() {
    // 美化打印
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
