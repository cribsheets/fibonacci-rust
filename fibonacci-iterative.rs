use std::env;

// this function generates the next fibonacci number,
// by summing the last two entries in the fibs vector
fn next_fib(fibs: &Vec<i32>) -> i32 {
    // 
    let (_, last_two) = fibs.split_at(fibs.len() - 2);
    let next = last_two.iter().sum::<i32>();
    return next;
}

// this function contains the fib generation off-camera so the main()
// function can stay neat
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

    // otherwise, we'll iteratively construct a fibonnaci sequence
    let mut fib_vec = vec![0, 1];

    while (fib_vec.len() as i32) < (number + 1) {
        let n = next_fib(&fib_vec); // grab the next number in the sequence
        fib_vec.push(n);
    }

    // last() returns a reference to the element, rather than
    // the element value, so we use * to dereference it
    return *fib_vec.last().expect("was empty?!");
}

fn main() {
    // in rust, the 0th argument is the string used to invoke the program.
    // we want the 1st argument, and we then need to convert it to an i32, as
    // rust is strongly typed
    let args: Vec<String> = env::args().collect();
    let number: i32 = args[1].parse().expect("argument should be an integer");

    println!("F({}) = {}", number, fib(number));
}
