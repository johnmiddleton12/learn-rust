fn main() {

    let x = 5;

    let x = x + 1;

    {
        // shadows x
        let x = x * 2;

        println!("inner x is {}", x);
    }

    println!("x is: {}", x);

}
