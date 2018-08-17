fn ownership2() {
    let foo = vec![1, 2, 3];
    let mut bar = foo;

    bar.push(4);

    println!("foo -> {:?}", foo);
    println!("bar -> {:?}", bar);
}


fn main() {
    ownership2();
}
