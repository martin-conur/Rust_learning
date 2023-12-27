fn main() {
    //// LOOP
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }

    // println!("End count = {count}")

    //// WHILE

    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1
    // }
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    //// FOR
    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}")
    // }

    // for number in (1..4).rev() {
    //     println!("{number}")
    // }
    // println!("LIFTOFF!!!")

    // TASKS
    // *Convert temperatures between Fahrenheit and Celsius.
    // *Generate the nth Fibonacci number.
    // *Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
    //   taking advantage of the repetition in the song.

    // // 1. temperature 
    // fn to_fahrenheit(celsius: f32) -> f32 {
    //     celsius * (9.0/5.0) + 32.0
    // }

    // println!("{}", to_fahrenheit(20.0));

    // 2. fibonacci
    fn fibonacci(n: i32) -> i32{
        if n == 0 {
            return 0
        } else if n == 1 {
           return 1
        } else {
            return fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
    println!("{}", fibonacci(30));
}
