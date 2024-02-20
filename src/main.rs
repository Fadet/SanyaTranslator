use std::io::Read;
use std::io;
use sanyabot::translator;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf);

    let res = translator::translate(buf);

    print!("{res}");
}