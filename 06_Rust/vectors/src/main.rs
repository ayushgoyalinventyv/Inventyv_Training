fn main() {
    let mut n = vec![10];
    n.push(20);
    for x in &n{
        println!("{} {}",x,n[0]);
    }
    println!("{:?}",n);
}
