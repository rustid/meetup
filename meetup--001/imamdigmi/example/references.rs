fn main() {
    let mut foo = 5;
    {
        let bar = &mut foo;
        *bar += 1;
    }
    println!("{}", foo);
}
