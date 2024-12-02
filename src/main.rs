fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = fibonacci(10);
    println!("{x}");

    let x = 6.0;

    {
        let x = x * 2.0;
        println!("The value of x in here is: {x}");
    }

    println!("The value of x is: {x}");

    let x = fahrenheit_and_celsius(x, "K");
    println!("{x}")
}

fn fahrenheit_and_celsius (num: f32, unit: &str) -> f32 {
    if unit == "C" {
        num * (9.0 / 5.0) + 32.0
    }

    else if unit == "F" {
        (num - 32.0) * (5.0 / 9.0)
    }

    else {
        num
    }
}

fn fibonacci (n: i32) -> i32 {
    println!("{n}");

    if n > 1 {
        fibonacci(n - 1) + fibonacci(n - 2)
    }

    else if n == 1 {
        1
    }

    else {
        0
    }
}
