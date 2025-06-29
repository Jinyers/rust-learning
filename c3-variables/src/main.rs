fn main() {
    // Creating immutable variable;
    let immutable_var = 6;
    
    println!("Immutable var = {}", immutable_var);

    // Will cause error:
    // immutable_var = 2;

    // Creating mutable variable;
    let mut mutable_var = 5;
    
    println!("Mutable var = {}", mutable_var);

    // Now it's okey, because variable mutable;
    mutable_var = 4;

    println!("Mutable var = {}", mutable_var);

    // Creating constant. Make sure to specify type of var.
    const MAX_VALUE: u32 = 1_000_000;
    
    println!("MAX_VALUE is {}", MAX_VALUE);
}
