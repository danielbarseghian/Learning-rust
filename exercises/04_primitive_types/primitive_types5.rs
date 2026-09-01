fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let name = &cat.0[1..5];
    let age = &cat.0[7..];

    println!("{name} is {age} years old");
}
