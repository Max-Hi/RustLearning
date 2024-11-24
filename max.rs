fn main() {
    println!("Hello, World!");

    borrowing_test();
}

fn borrowing_test() {
    //any number of reads or one read/write
    let mut s1 = String::from("hello");
    println!("{}", s1);
    let s2 = &s1;
    s1 = String::from("world");
    println!("{}", s1);
    // This line now produces an error: 
    // println!("{}", s2);

    let mut s3 = String::from("hello");
    println!("{}", s3);
    let s4 = &s3;
    // alternatively, this line can be dropped and the last line is not a problem any more: 
    // s1 = String::from("world");
    println!("{}", s3);
    println!("{}", s4);
}