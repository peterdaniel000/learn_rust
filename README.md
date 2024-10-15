rust programming for blockchain development 
1. concept of rust programming 
   variable writing 
let x = 5
println!("The value of x is: {}, x");
  We use the mut keyword to make immutable variable mutable 
  const is created using the const key word and they are always mutable and the conventional way of wrting a const variable is to make all the name to be uppercase and to must be annotated.
        SHADOWING 
    let x = 5;
    let x = x + 1;
    let x = x + 2;

    DATA TYPE 
Scalar data type 
conpound data type

Scalar data type: It represent a single variable and there are four primary scalar types; integers,floating-point number, booleans and characters
    integer types 
        unsigned and sign
        u32 and i32 respectively
    floating-point types
        f32 and f64 


you prefix variable that you are not use e.g 
let _x = 5;