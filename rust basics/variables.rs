fn main(){
    println!("Hello, World!");
}

//initialisation of variables
fn main{
  let x: i32 = 5; // Initialized and used - OK
    let y: i32;     // Uninitialized and unused - Warning only
    assert_eq!(x, 5);
    println!("success");
    println!("x = {}", x);
}

//mutable variables are declared with the mut keyword . they can be changed after they are initialized.
//immutable variables are declared without the mut keyword. they cannot be changed after they are initialized.
fn main(){
    let mut x=1; // added"mut" keyword to make x mutable so x can be changed after initialization
    x+=1; // x is changed to 2
    assert_eq!(x, 2);
    println!("x = {}", x);
    println!("success");
}

//scope is the region of the program where a variable is valid. A variable is valid from the point it is declared until the end of the block in which it is declared. A block is a set of statements enclosed in curly braces {}. A variable can be shadowed by another variable with the same name in an inner block. The inner variable will be valid only within that block and will not affect the outer variable.
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // Only 'x' is accessible here because 'y' is out of scope
    println!("The value of x is {}", x); 
}

//function and return values: Functions are declared using the fn keyword. The return type of a function is specified after the -> symbol. If a function does not return a value, it is called a "unit" function and its return type is ().
fn main(){
    println!("The value of x is {}", add(5, 10));
}                                                
fn add(x: i32, y: i32) -> i32 {
    x + y // The last expression in a function is the return value. No semicolon is used.
}  
//anopther example of function and return values
fn main(){
    let x = define_x();
    println("{},world", x);
}
fn define_x()->&str{
    "hello" // return "hello back to the main function"
}
//shadowing: A variable can be shadowed by another variable with the same name in an inner block. The inner variable will be valid only within that block and will not affect the outer variable.
fn main() {
    let x: i32 = 5;
    {
        let x = 12; 
        assert_eq!(x, 12); // Fixed: inner x is 12
    }
    assert_eq!(x, 5); // Outer x is still 5
}

//unused variables: If a variable is declared but not used, the compiler will issue a warning. To suppress this warning, you can prefix the variable name with an underscore (_). This indicates that the variable is intentionally unused.
fn main() {
    let _x: i32 = 5; // Unused variable with underscore prefix - No warning
    let y: i32 = 10; // Unused variable without underscore prefix - Warning
    println!("The value of y is {}", y);
}
//using of atttributes because ham chahte hai ki kahin bhi unused variable ka warning na aaye to hum uske aage underscore _ laga dete hai.. Ham file ke sabse upar ek special attribute likh dete hai jisse ki file ke andar jitne bhi unused variable hai unka warning na aaye..
#![allow(unused_variables)] // This attribute suppresses warnings for unused variables in the entire file


///destructuring ka matlab hai ki hum ek complex data structure ko uske components me tod sakte hai. Rust me destructuring ka use pattern matching ke sath hota hai. Hum tuples, structs, aur enums ko destructure kar sakte hai. ham bade data structure ko chote chote parts me tod kar unhe alag alag variables me store kar sakte hai. isse code ko samajhna aur maintain karna easy ho jata hai.
fn main(){
    // 1. Pehle x aur y variables ko declare kiya (abhi khali hain)
    let (x,y);
    // 2. Fir ek tuple ko destructure kiya aur uske components ko x aur y me assign kiya
    let (x,..y) = (5, 10); // Tuple destructuring
    // 3. Fir x aur y ko print kiya
    println!("x = {}, y = {}", x, y);
    // 3. Slice destructuring: [1, 2] array mein se '..' matlab pehli sabhi values ignore, 
    // aur aakhri wali value 'y' ko mil jayegi (2).
    [.., y] = [1, 2]; 
    // Abhi yahan y ki value 2 ho chuki hai.

    // 4. Check karna ki x aur y ki values kya hain
    // [x, y] ek array ban gaya [3, 2], aur dusri taraf target array [3, 2] hai.
    assert_eq!([x, y], [3, 2]); 

    println!("Success!");
}