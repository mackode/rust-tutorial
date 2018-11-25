use std::io::Error;
use std::fmt;

/*
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
*/

type Kilometers = i32;
type ResultTuple<T> = Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> ResultTuple<usize>;
    fn flush(&mut self) -> ResultTuple<()>;

    fn write_all(&mut self, buf: &[u8]) -> ResultTuple<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> ResultTuple<()>;
}

/*
fn bar() -> ! {
    // --snip--
}
*/

fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));
}

/*
fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // --snip--
}
*/

type Thunk = Box<Fn() + Send + 'static>;

/*
fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
*/

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
