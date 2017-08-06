fn main() {
    let foo = vec![1, 2, 3];
    let bar = foo;
    // Variable foo tidak akan bisa di panggil kembali
    // karen foo sudah dipidahkan ke bar
    println!("First value is {}", bar[0]);
}
