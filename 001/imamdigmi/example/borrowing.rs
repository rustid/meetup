fn main() {
    fn foo(val: &Vec<&str>) {
        println!("{:?}", val);
    }
    let val = vec!["satu", "dua", "tiga"];
    foo(&val);
}
