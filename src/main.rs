use fib_number;

fn main() {
    println!("fib_number 1_000 is : {:?}", fib_number::fib(1_000));
    println!(
        "fib_number 1_000_000 is : {:?}",
        fib_number::fib_matrix(1_000_000)
    );
}
