fn main() {

    let mut count = 0;

    let result = 'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up 5;
            }

            remaining -= 1;
        }

        count += 1;
    };

    println!("End count - {}", count);

    println!("End result - {}", result);

    while count < 10 {
        println!("count = {}", count);
        count += 1;
    }

    for i in (0..10).rev() {
        println!("i = {}", i);
    }

}
