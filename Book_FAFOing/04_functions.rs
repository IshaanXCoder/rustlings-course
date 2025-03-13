fn main() {
    another_function(500);
    myFun();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn myFun(){
    let x = {
        let y = 50;
        y + 1
    };
    println!("The value of x is: {x}"); 
}