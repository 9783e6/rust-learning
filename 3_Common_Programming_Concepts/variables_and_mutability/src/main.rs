fn main() {
    // immutable
    let x = 5;


    // mutable
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");


    // Const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // Shadowing
    let z = 5;
    let z = z + 1;
    
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");
    

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is {spaces}");
}
