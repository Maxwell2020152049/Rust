struct Rectangle {
    a: i32, // 长
    b: i32, // 宽
}

impl Rectangle {
    fn area(&self) -> i32{
        self.a * self.b
    }
}

fn main() {
    let t: Rectangle = Rectangle {
        a: 20,
        b: 10,
    };

    println!("area of triangle is {}", t.area());
}
