fn main() {
    let mut x = 5;
    println!("The value of x {}", x);
    x = 6;
    println!("The value of x {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS {}", THREE_HOURS_IN_SECONDS);

    {
        let x = x * 2;
        println!("The value of x (inner scope) {}", x);
    }
    println!("The value of x {}", x);

}
