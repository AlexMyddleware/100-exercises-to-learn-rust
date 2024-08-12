so in exirsite 01 syntax, i need to pu the type u32 in the function signature


in exercise 02_varibales,
i learn that let and const are mutually exclusive, so i can't use let and const for the same variable declaration

and i can just declare a const variable if one of the variables to calculate it is not a const
correction for this line : A `const` variable must be initialized with a constant expression and is immutable.

so i use let mut to declare a mutable variable

exercises/02_basic_calculator/04_panics/src/lib.rs

there i learned that one has to use no () in the if statement, and for the panic you just put the panic with parentheses and then the message inside

important, there is a ! in the panic, don't forget it !

---------------------------------------------------------

exercises\02_basic_calculator\07_for\src\lib.rs
need to not put space in the for loop for i in (2..=n).rev() {
because else it will not work