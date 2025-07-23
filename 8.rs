fn main() {
    let array:[i32;10]=[3,5,8,11,22,2,4,6,9,10];
    println!("the given array is arr={:?}",array);
    let a=&array[1..3];
    let p=array.len();
    println!("a={:?}",a);
    
    let b=&array[1..p];
    println!("b={:?}",b);
    
    let c=&array[0..p-1];
    println!("c={:?}",c);
    
    let d=&array[1..p-1];
    println!("d={:?}",d);

}                 
