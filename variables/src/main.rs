fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const Y: u32 = 10;
    println!("The value of Y is: {}", Y);

    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let guess: u32 = "42".parse().expect("Not a number!");
    print_type_of(&guess);

    let sum = 5 + 10;
    println!("The sum of 5 + 10 is: {}", sum);

    let difference = 95.5 - 4.3;
    println!("The difference of 95.5 - 4.3 is: {}", difference);

    let product = 4 * 30;
    println!("The product of 4 + 30 is: {}", product);

    let quotient = 56.7 / 32.2;
    println!("The division of 56.7 / 32.2 is: {}", quotient);

    let remainder = 43 % 5;
    println!("The division of 43 % 5 is: {}", remainder);

    let t = true;
    let f = false;
    println!("Variable T is: {}", t);
    println!("Variable F is: {}", f);

    let character = 'z';
    println!("Variable character (char type) is: {}", character);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("Tuple A: {}", a);
    println!("Tuple B: {}", b);
    println!("Tuple C: {}", c);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The 10th Month is: {}", months[9]);

    if x == 6 {
        println!("X is Still Equal to 6!");
    }
    else {
        println!("This Shouldn't Happen!");
    }

    if x + 1 == 6 {
        println!("This Shouldn't Happen!");
    } else if x - 1 == 5 {
        println!("X - 1 is equal to 5!");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // See Solution Below for Better Implementation
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn print_type_of<T>(_: &T) {
    println!("The type of guess is: {}", std::any::type_name::<T>())
}
