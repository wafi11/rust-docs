mod human;
mod op;
mod ticket;

use human::{Human, print_user};
use op::{Op, calc};

fn parity(n: i32) -> &'static str {
    if n % 2 == 0 { "genap" } else { "ganjil" }
}


fn string_not_str(data : &str){
    println!("{}",data)
}

fn main() {
    let mut data = Human {
        name: "wafiuddin".into(),
        age: 10,
    };


    string_not_str("string");

    data.birthday();

    for op in [Op::Plus, Op::Minus, Op::Mul, Op::Div] {
        match calc(op, 1, 2) {
            Ok(v) => println!("{op:?}: {v}"),
            Err(e) => println!("{op:?} error: {e}"),
        }
    }

    println!("{} {}", parity(1), parity(2));

    print_user(&data);
}
