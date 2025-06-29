fn main() {
    // Creating immutable variable;
    let immutable_var = 6;
    
    println!("Immutable var = {}", immutable_var);

    // Will cause error:
    // immutable_var = 2;
    
    // Creating mutable variable;
    let mut mutable_var = 5;
    
    println!("Var x = {}", mutable_var);

    // Now it's okey, because variable mutable;
    mutable_var = 4;

    println!("Var x = {}", mutable_var);
}
