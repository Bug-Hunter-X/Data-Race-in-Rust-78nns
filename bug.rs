fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; 
    *y = 10; 
    *z = 12; //Data race
    println!("x = {}", x);
}