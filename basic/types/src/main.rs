fn main() {
    // ==================== integer ==========================
    let value: i32 = -42;
    println!("value {}", value);

    let value: u32 = 42;
    println!("value {}", value);

    let value = 98_222_000;
    println!("value {}", value);

    let value = b'A';
    println!("value = {}", value);

    // ==================== float ==========================
    let value32_min = std::f32::MIN;
    println!("value f32 min = {}", value32_min);
    let value32_max = std::f32::MAX;
    println!("value f32 max = {}", value32_max);

    let value64_min = std::f64::MIN;
    println!("value f64 min = {}", value64_min);
    let value64_max = std::f64::MAX;
    println!("value f64 max = {}", value64_max);

    println!("std::f64");
    // Approximate number of significant digits in base 10.
    println!("DIGITS = {}", std::f64::DIGITS);

    // Difference between 1.0 and the next largest representable number.
    println!("EPSILON = {}", std::f64::EPSILON);

    // Infinity (∞).
    println!("INFINITY = {}", std::f64::INFINITY);

    // Number of significant digits in base 2.
    println!("MANTISSA_DIGITS = {}", std::f64::MANTISSA_DIGITS);

    // Largest finite f64 value.
    println!("MAX = {}", std::f64::MAX);

    // Maximum possible power of 10 exponent.
    println!("MAX_10_EXP = {}", std::f64::MAX_10_EXP);

    // Maximum possible power of 2 exponent.
    println!("MAX_EXP = {}", std::f64::MAX_EXP);

    // Smallest finite f64 value.
    println!("MIN = {}", std::f64::MIN);

    // Minimum possible normal power of 10 exponent.
    println!("MIN_10_EXP = {}", std::f64::MIN_10_EXP);

    // One greater than the minimum possible normal power of 2 exponent.
    println!("MIN_EXP = {}", std::f64::MIN_EXP);

    // Smallest positive normal f64 value.
    println!("MIN_POSITIVE = {}", std::f64::MIN_POSITIVE);

    // Not a Number (NaN).
    println!("NAN = {}", std::f64::NAN);

    // Negative infinity (-∞).
    println!("NEG_INFINITY = {}", std::f64::NEG_INFINITY);

    // The radix or base of the internal representation of f64.
    println!("RADIX = {}", std::f64::RADIX);


    // ==================== math ==========================
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("remainder = {}", remainder);

    // ==================== boolean ==========================
    let t = true;

    println!("t = {}", t);
    println!("!t = {}", !t);
    println!("t && t= {}", t && t);
    println!("t || t= {}", t || t);

    let f: bool = false; // with explicit type annotation

    println!("f = {}", f);
    println!("!t = {}", !t);
    println!("t && t= {}", t && t);
    println!("t || t= {}", t || t);

    // ==================== symbols ==========================
    let c:char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart_eyed_cat = {}", heart_eyed_cat);


    // ==================== tuples ==========================
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x,y,z) is: ({},{},{})", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let v1 = x.0;
    let v2 = x.1;
    let v3= x.2;

    println!("The value of (x,y,z) is: ({},{},{})", v1,v2,v3);

    // ==================== arrays ==========================
    let a = [1, 2, 3, 4, 5];
    let b: [u8; 5] = [1, 2, 3, 4, 5];
    println!("a is: {:?}", a);
    println!("b is: {:?}", b);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("first = {}, second = {}", first, second);
}