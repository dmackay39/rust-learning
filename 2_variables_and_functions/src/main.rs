fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    // Constants are always immutable.
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // x can be reassigned so long as it has the mut keyword.
    // You cannot change the type of a variable when you use mut.

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    // You can shadow a variable by using let again.

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
    // You can also change the type of a variable when you shadow it.
    
    // Rust is a statically typed language. Though it can infer type from how you use it.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {guess}");
    // Eg here you have to specify u32 otherwise rust won't know what type to parse the string into.
    // u refers to it being "unsigned" ie not negative. The 32 refers to it being a 32 bit number.

    let _floating_point_1 = 2.0; // f64

    let _floating_point_2: f32 = 3.0; // f32

    // Compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // A tuple is a fixed length collection of values of different types.
    let _five_hundred = tup.0;
    let (_a, b, _c) = tup;
    // Destructuring a tuple.
    println!("The value of b is: {b}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // An array is a fixed length collection of values of the same type.
    let _first = arr[0];

    let _number = another_function(5);

    looped_function();
    countdown();
    for_loop();

}

fn another_function(x: i32) -> i32 {
    // Must declare the type of each parameter.
    println!("The value of x is: {x}");
    x + 1
    // No semicolon here as this is an expression that returns a value rather than a statement.
}

fn looped_function() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn countdown() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}