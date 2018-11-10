/*
enum List {
    Cons(i32, List),
    Nil,
}
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
