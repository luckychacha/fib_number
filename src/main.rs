use fib_number;

fn main() {
    let number: usize = 1_000_000;
    // todo: support bigger number.
    // let number: usize = 1_000_000;
    // println!("fib_number {number} is : {:?}", fib_number::fib(number));
    println!(
        "fib_number {number} is : {:?}",
        // fib_number::fib(number)
        fib_number::fib_matrix(number as u64)
    );
}
