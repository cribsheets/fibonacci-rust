use std::env;

fn fib(number: i32) -> i32 {
    // remember the rules:
    // read a single argument n, and compute F(n) where:

    // - F(0) = 0
    // - F(1) = 1
    // - F(n) = F(n-1) + F(n-2)

    // we can shortcut in the case of 0 or 1
    // these also serve as our base cases in the recursive version
    if 0 == number { return 0 }
    if 1 == number { return 1 }

    // otherwise, we just implement the formula literally
    return fib(number - 1) + fib(number - 2);
}

fn main() {
    // in rust, the 0th argument is the string used to invoke the program.
    // we want the 1st argument, and we then need to convert it to an i32, as
    // rust is strongly typed
    let args: Vec<String> = env::args().collect();
    let number: i32 = args[1].parse().expect("argument should be an integer");
    println!("F({}) = {}", number, fib(number));
}
