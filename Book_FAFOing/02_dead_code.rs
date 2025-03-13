 fn main(){
    // let a: i32 = 5;
    //this will give us an error ki a is an unused varibale so to get rid of it we will ad a _ before a.

    let _a: i32 = 5;
    // this means a is a 32 bit integer with value 5

    println!("Hello, world!");

 }

#[allow(dead_code)]

 fn newFun(){
//this is again a unused functino so error again. hence we'll have to add allow(dead_code) to get rid of it.

        println!("This is a new function");

        //and agar newFun ko pehele and allowdeadcode baadme kare toh error hoga!
 }