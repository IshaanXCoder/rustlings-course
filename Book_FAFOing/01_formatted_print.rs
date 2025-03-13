fn main(){ 
    println!("{} days", 32);
    println!("{} {}days", 31, 32);

    //this numbering starts from 0, even if we leave the spacres blank it ill work proerly but for better udnerstadnign, likhhdete he
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // println!("{1}, this is {2}. {2}, this is {1}", "Alice", "Bob");
    println!("{0} {1}{2}", "a", "b","c");
}