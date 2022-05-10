

// {} 的代码块是一个 scope 表达式，表达式返回最后一个值
#[test]
fn test_simple_scope() {
    let y = {
        let x = 3;
        x + 1   // 注意这里没有分号
    };
    assert_eq!(4, y);
}

#[test]
fn test_scope_shadow() {
    let x = 5;
    let y = {
        let x = 3;  // 这里重新声明了 x，由于这是新的 scope ， 因此不会对上面的 x 覆盖
        x + 1   // 注意这里没有分号
    };
    assert_eq!(4, y);
    assert_eq!(5, x);
}
