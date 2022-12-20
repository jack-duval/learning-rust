const GLOBAL_NUMBER: u32 = 1;

fn main() {
    // let without mut is immutable assignment
    let x = 5;
    // x += 1 or x = 2 will throw an error at compile time
    println!("X: {}", x);

    let mut y = 6;
    print("Y before inc: {}", y);
    y += 1; // this is fine bc we included mut
    print("Y after inc: {}", y);

    // Constants:
    // - immutable and can't change (not just immut by default)
    // - must be type and value annotated.
    // - constants can be declared in any scope
    // - can only be set to constant expressions, not runtime computed values
    const MINUTES_IN_YEAR: u32 = 60 * 24 * 365;
    // global_number also assigned outside main for ex
    println!("Minutes in a Year (const defined in main): {}", MINUTES_IN_YEAR);
    println!("Global constant: {}", GLOBAL_NUMBER);

    // Shadowing


    println!("Hello, world!");
}
