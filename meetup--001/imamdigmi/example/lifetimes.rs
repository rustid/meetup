fn main() {
    fn life<'a>(bar: &'a str) -> &'a str {
        bar
    }
    let foo;
    {
        let bar = "<3 Rust";
        foo = life(bar);
    }
    println!("{}", foo);
}
