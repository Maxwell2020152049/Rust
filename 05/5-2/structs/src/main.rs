struct Rectangle {
    a: i32, // 长
    b: i32, // 宽
}

fn main() {
    let t: Rectangle = Rectangle {
        a: 20,
        b: 10,
    };

    let area: i32 = t.a * t.b;

    println!("area of triangle is {}", area);
}
