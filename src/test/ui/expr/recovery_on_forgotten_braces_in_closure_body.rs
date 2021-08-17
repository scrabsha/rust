fn main() {
    let num = 5;
    (1..num).reduce(|a, b| 
        println!("{}", a);
        a * b
    ).unwrap();
}