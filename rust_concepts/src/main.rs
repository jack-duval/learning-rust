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
    //  var names can be used & referenced in local scope

    // DATA TYPES
    //  Integers:
    let int_unsigned_8bit:u8 = u8::MAX;
    let int_signed_8bit:i8 = i8::MIN;
    let int_unsigned_16bit:u16 = u16::MAX;
    let int_signed_16bit:i16 = i16::MIN;
    let int_unsigned_32bit:u32 = u32::MAX;
    let int_signed_32bit:i32 = i32::MIN;
    let int_unsigned_64bit:u64 = u64::MAX;
    let int_signed_64bit:i64 = i64::MIN;
    let int_unsigned_128bit:u128 = u128::MAX;
    let int_signed_128bit:i128 = i128::MIN;
    let int_unsigned_arch:usize = usize::MAX;
    let int_signed_arch:isize = isize::MIN;

    //  Number Representation
    let decimal_num = 1_000;
    let hex_num = 0xff;
    let octal_num = 0o77;
    let binary_num = 0b1111;
    let byte_u8_only = b'A';

    //  Floating Point types
    let float_64 = 1.0; // Default to float64 (double precision)
    let float_32: f32 = 1.0; // f32, single precision

    //  Numeric Operations
    let sum = 10 + 2;
    let difference = 20.5 - 20.25;
    let product = 20 * 10;
    let quotient = 20.4 / 10.6;
    let floored = 2 / 3; // results in 0
    let remainder = 20 % 3;

    //  Booleans
    let t = true;
    let f: bool = false; // optional explicit annotation

    //  Characters
    let a = 'a';
    let b: char = 'A'; // explicit
    let smiley_emoji = '😀'; // Unicode


    // COMPOUND_TYPES
    //  Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (tupx, tupy, tupz) = tup; // assign individual params to vars
    let tupa = tup.0; // assign using index of tuple
    let tupb = tup.1;
    let tupc = tup.2;

    //  Arrays
    let arr = [1, 2, 3, 4, 5];
    //


    println!("Hello, world!");
}
