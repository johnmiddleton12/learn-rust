fn main() {

    let x = 5;

    let x = x + 1;

    {
        // shadows x
        let x = x * 2;

        println!("inner x is {}", x);
    }

    println!("x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces is {}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {}", a[0]);
    {
        let a: [i32; 5] = [2, 2, 3, 4, 5];
        println!("The value of a is: {}", a[0]);
    }
    println!("The value of a is: {}", a[0]);

    let index = 5;

    // doesn't run, out of bounds - runtime error, panic!
    //println!("The value of a is: {}", a[index]);

}
