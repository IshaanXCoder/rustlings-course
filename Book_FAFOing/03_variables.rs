// by default, variables are immutable

fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");

    //so to make it mutable we have to do thissss, add a lmut before the naem
    let mut x = 6;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");


    ///BUT KUCH KAAM KI CHEEZ IS KI we’re not allowed to mutate a variable’s type:

    let mut x = 3;
    println!("Number {x}"); //not ,x

    ///CONSTANTS
    /// Constants are valid for the entire time a program runs, within the scope in which they were declared.
    /// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const NUMBER: i32 = 3;
    println!("Number: {NUMBER}");

    newShit();
    
}


///Shadowing
fn newShit(){

        let x = 5;
    
        let x = x + 1;
    
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
    
        println!("The value of x is: {x}");
    
}