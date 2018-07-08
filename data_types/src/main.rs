
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("double {} float {}", x, y);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("sum {} difference {} product {} quotient {} remainder {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t {} f {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c {} z {} heart_eyed_cat {}", c, z, heart_eyed_cat);

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup {:?} The value of y is: {} {} {}", tup, x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x {:?} {} {} {}", x, five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    println!("a {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months {:?}", months);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("a {:?} first {} second {}", a, first, second);

    let a = [1, 2, 3, 4, 5];
    let index = 3;
    let element = a[index];
    println!("The value of element is: {:?} {} {}", a, index, element);
}
