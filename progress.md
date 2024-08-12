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

-------------------- 07 for loop ---------------------
for <element> in <iterator> {
    // code to execute
}

basic example
example of inclusive range

let mut sum = 0;
for i in 1..=5 {
    sum += i;
}


that means that the for loop will iterate over the values 1, 2, 3, 4, and 5.

if we imagine that being a tauren picking up herbs, he will pick up 5 herbs, the 5th herb being the last one he picks up

_________________________________________________________

1..5: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.

let mut sum = 0;
for i in 1..5 {
    sum += i;
}

that means that the for loop will iterate over the values 1, 2, 3, and 4.

if we imagine a crucible knight using crucible incantations, he will use 4 incantations, the 4th incantation being the last one he uses

_________________________________________________________

1..: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).

let mut sum = 0;
for i in 1.. {
    sum += i;
}

that means that the for loop will iterate over the values 1, 2, 3, 4, 5, 6, 7, 8, 9, and so on.

if we imagine angelo from dragon quest 8 trying to seduce women, then he will try to seduce an infinite amount, starting at 1 because women can't start at 0, and it 's open ended

_________________________________________________________


..5: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.

let mut sum = 0;
for i in ..5 {
    sum += i;
}

that means that the for loop will iterate over the values 0, 1, 2, 3, and 4.

if we imagine harry potter picking up berties botts every flavor beans, he will pick up 4 beans, the 4th bean being the last one he picks up, but at first he will pick up 0 beans because he misses and it's a bad flavor and the last one is a bad flavor 2, we don't want 5 because that is the order of the phoenix

_________________________________________________________

..=5: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.

let end = 5;
let mut sum = 0;

for i in ..=end {
    sum += i;
}

that means that the for loop will iterate over the values 0, 1, 2, 3, 4, and 5.
if we imagie the animal well character trying to get to the end of the game, he will get to the end of the game, but then there is a hidden level + 1 that he has to get to
