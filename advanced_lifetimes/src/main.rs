/*
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
*/

/*
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
*/

struct Context<'s>(&'s str);

/*
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}
*/

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

fn main() {
    println!("Hello, world!");

    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}

//struct Ref<'a, T>(&'a T);
struct Ref<'a, T: 'a>(&'a T);

struct StaticRef<T: 'static>(&'static T);

trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }
