// Here, we discuss mutability

fn main() {
    //  ---- VARIABLES----
    // are by default immutable.
    let immutable = 1;
    println!("immutable variable: {immutable}");
    // immutable = 2 // will not compile
    let mut mutable = 1;
    println!("mutable variable: {mutable}");
    mutable = 2; // will compile
    println!("mutable variable now: {mutable}");

    // ---- CONSTANTS ----
    // similar to immutable variables
    // but mut and const are incompatible
    // constants MUST be type annotated and CANNOT be computed at runtime
    const PI: f64 = 3.14159265;
    // let PI: f32 = 3.14;   // will not compile
    // const E = 2.718       // will not compile
    println!("constant PI is {PI}");

    // ---- SHADOWING ----
    // Shadowing also allows for change in types
    // This is possible due to type inference
    let x = 5;
    let x = x + 1; // OK 
    {
        let x = 10; // shadowed
        println!("inner x: {x}");
    }
    println!("outer x: {x}");
    // type inferred
    let str = "foobar";
    let str = str.len();
    println!("str is {str}");

    // ---- TYPES ----
    // Rust is statically typed.
    // At compile time, types of all variables must be known
    // Compiler can infer, but when it cannot, type annotating will help
    let ux: u8 = 0b1000_0001;
    let sx: i8 = -10;
    // Sizes: 8, 16, 32, 64, 128, isize, usize (arch dependent)
    println!("{ux}, {sx}");
    let _fx = 2.0; // f64
    let _fy: f32 = 3.0; // f32

    // ---- MATHEMATICAL OPERATIONS ----
    // + - * / % 
    // integer division truncates to 0 like C
    let x = 10;
    let y = 3;
    let x_div_y = 10/3; // 3
    println!("{x},{y},{x_div_y}");  

    // ---- CHARACTERS ----
    // char types are 4 bytes (not 1!)
    // this is a unicode scalar type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c},{z},{heart_eyed_cat}");

    // ---- TUPLES ----
    // Similar to pythin tuples.
    // Fixed length! Type annotations OPTIONAL
    let tup: (i8, char, bool) = (1, 'b', true);
    let (a, b, c) = tup; // destructure
    println!("Destructured tuple: {a},{b},{c}");

    // ---- ARRAYS ----
    // Similar to C, all elements have the same types
    // Arrays have FIXED LENGTH
    let _arr = [1,2,3,4,5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // ---- FUNCTIONS ----
    // refer to below
    some_void_function(10);
    let bar: char = 'a';
    let bar = some_function(10, bar);
    println!("{bar}");
    control_flow();

}


// ---- FUNCTIONS ----

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. 
// Function definitions are statements.
// Calling a function is an expression.

// Return signature not needed
fn some_void_function(foo: i32) -> () {
    println!("some_void_function called with argument {foo}");
}

fn some_function(foo: i32, bar: char) -> char {
    let _y = {
        let x = 1;
        foo + x 
    }; // the statement in {} is an expression

    // adding ; causes expressions to become statements
    // we can force a statement by using return
    // return bar
    // is equivalent to
    bar
}

// Demo control flow 
fn control_flow() {
    let foo = 3;
    // IF ELSE 
    if foo < 3 {
        // ...
    } else if foo > 3 {
        // ...
    }else {
        // foo == 3
    }
    // TERNARY
    let _bar = if foo % 3 == 0 {10} else {-10};

    // LOOPING RETURN
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // WHILE LOOP 
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // FOREACH
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // FOR ITER LOOP
    // n..m -> [n,m)
    // n..=m -> [n,m]
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}