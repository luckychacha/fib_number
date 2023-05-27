use fib_number;

fn main() {
    let number: usize = 1_000;
    // todo: support bigger number.
    // let number: usize = 1_000_000;
    println!("fib_number {number} is : {:?}", fib_number::fib(number));
}
