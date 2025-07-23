fn main() {
    // immutable borrow
    let a = 10;
    let b = &a;
    println!("a = {}", a);
    println!("b = {}", b);
   
    // mutable borrow
    let mut p = 20;
    let q = &mut p;
    println!("p = {}", q);
    *q = 30;
    println!("p = {}", q);
   
}
