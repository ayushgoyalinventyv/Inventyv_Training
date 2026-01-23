fn main() {
    let n:[i32;5]=[1,2,3,4,5];
    let mut n2=[0;6];

    n2[0..5].clone_from_slice(&n);
    n2[5]=6;

    println!("{:?}",n);
    println!("{:?}",n2);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
