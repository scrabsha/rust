// This test corresponds to issue #88065 which points out that there is no
// recovery when parsing a block closure body when there is no braces around
// its statements.

fn main() {
    let num = 5;
    (1..num).reduce(|a, b| 
        println!("{}", a); //~ ERROR: Missing braces around block
        a * b
    ).unwrap();
}
