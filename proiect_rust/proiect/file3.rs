struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Johnnyyyyy"),
        age: 30
    };
    println!("{} is {}", p.name, p.age);
}