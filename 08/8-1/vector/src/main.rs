fn main() {
    // 使用 new() 声明Vector
    // 下文如果使用了 push() 方法, Rust可以自行推断出类型
    let mut v1 = Vec::new();

    v1.push(20);
    v1.push(20);
    v1.push(15);
    v1.push(20);
    v1.push(49);

    for i in v1 {
        println!("value is {}", i);
    }
}
