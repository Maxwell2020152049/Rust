fn main() {
    let s1: String = String::from("Hello, World!");

    // 使用了 clone, s1的所有权没有被回收
    let s2: String = s1.clone();

    println!("{}, {}", s1, s2);
}
