//DIFFERENT DAta types ke baich rust direct asssignment allow nahi karega esliye assign karne ke  liye type casting ka use karna padega  ya fir  dataa type hi change karna padega
fn main() {
    let a: i32 = 10;
    let b: f64 = 20.5;

    // Type casting
    let c: f64 = a as f64 + b;

    println!("The sum is: {}", c);
}
fn main() {
    let x: u32 = 5; // i32 se u32 kar diya
    let mut y: u32 = 5; 

    y = x; // Ab dono ka type 'u32' hai, toh error nahi aayega!

    let z = 10; // Type of z yahan default 'i32' ban jayega (Type Inference ki wajah se)

    println!("Success!");
}

//Type Casting, MAX limits, Overflow handling, aur Different Number Literals//
fn main(){
    // 1. Type Casting (`as` keyword)
    // u8 ko u16 mein convert karna
    let v_cast:u16 = 255u8 as u16;
    println!("Value after casting: {}", v_cast);
// 2. Integer Limits (::MAX)
    // i8 aur u8 ki max limits ko check karna
    assert_eq!(i8::MAX,127);
    assert_eq!(u8::MAX,255);
    println!("i8 max: {}, u8 max: {}", i8::MAX, u8::MAX);
// 3. Integer Overflow & Checked Addition (Safe Methods)
    // Badi range wale types (u16, i16) ka use karna ya checked_add use karna taaki panic na ho
let v1:u16 = 251_u16+8;
let v2: i16 = i16::checked_add(251,8).unwrap();
println!("Checked addition result: {}", v2);
// 4. Different Number Formats (Literals)
    // Decimal, Hexadecimal, Octal, aur Binary ka addition
    // 1_024 (Decimal) + 0xff (255) + 0o77 (63) + 0b1111_1111 (255) = 1597
    let v_total = 1_024 + 0xff + 0o77 + 0b1111_1111;
    
    assert!(v_total == 1597);
    println!("4. Number Literals Calculation Successful: v = {}", v_total);

    println!("\n🎉 Success! Sabhi concepts ekdum sahi se run ho gaye.");

}
  topic covered ====Floating-Point Types (f64), Float Precision/Accuracy Problems (0.1 + 0.2 != 0.3), Ranges (.. and ..=), aur Bitwise Operations (AND, OR, XOR, Shifting)
  let x : f64 = 1_000.0; // f64 type ka variable
  let y : f64 = 0.1 + 0.2; // Floating point precision problem ka example`
  let z : f64 = 0.3; // Expected value
  assert_eq!(type_of(&x), "".to_string());
  println!("y: {}, z: {}", y, z); // y aur z ko print karna

 // 2. Float Precision Fix (0.1 + 0.2)
fn main() {
    let result = (0.1 + 0.2 - 0.3).abs();
    
    // Check kar rahe hain ki fark f64::EPSILON se chota hai ya nahi
    assert!(result < f64::EPSILON);
    
    println!("Assertion passed! Floating point difference is: {}", result);
}




 //3. range operations
 let mut sum = 0;
 for i in -3..2{ //-3.-2,-1,0,1
    sum += i;
 }
 assert_eq!(sum, -3); // sum = -3
 println!("Sum of range -3..2 is: {}", sum);

 //4. computations and modulus operations
 assert!(5 % 2 == 1); // Modulus operation
 assert!(5 / 2 == 2); // Integer division
 println!("5 % 2 = {}, 5 / 2 = {}", 5 % 2, 5 / 2);
 //5. Bitwise operations
 let a: u8 = 0b1010_1010; // Binary representation
 let b: u8 = 0b1100_1100; // Binary representation
 println!("Bitwise AND: {:08b}", a & b); // Bitwise AND
 println!("Bitwise OR: {:08b}", a | b); // Bitwise OR
 println!("Bitwise XOR: {:08b}", a ^ b); // Bitwise XOR
 println!("Left Shift: {:08b}", a << 1); // Left Shift
 println!("Right Shift: {:08b}", a >> 1); // Right Shift

 //helper function to get the type of a variable
fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}