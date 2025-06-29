fn main() {
    // Creating immutable variable;
    let x = 5;
    
    println!("Var x = {}", x);

    // Cause error, because variable is immutable;
    x = 4;

    println!("Var x = {}", x);
}
