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

    /* *
        Types - Integer
    * */
    let int_var: u32 = 12555;
    println!("{}", int_var);
    let int_var: i32 = -12555 + 555;
    println!("{}", int_var);

    /* *
        Types - Float
    * */
    let float_var: f64 = 3.666457547547547547547547547;
    println!("{}", float_var);

    println!("{}", float_var - 2.64555);

    /* *
        Types - Tuples
    * */
    let tuple = (1, 2, 3, 4, 5, "Text");
    println!("Tuple position 0: {}", tuple.0);
    println!("Tuple position 1: {}", tuple.1);
    println!("Tuple position 2: {}", tuple.2);
    println!("Tuple position 3: {}", tuple.3);
    println!("Tuple position 4: {}", tuple.4);
    println!("Tuple position 5: {}", tuple.5);

    /* *
        Types - Arrays (must be the same type)
    * */
    let array_i32: [i32; 5] = [0, 1, 2, 3, 4];
    println!("Array position 0: {}", array_i32[0]);


}
