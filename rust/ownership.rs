fn ownership() {
    let foo = 1;
    let mut bar = foo;

    bar += 5;

    println!("foo -> {}", foo);
    println!("bar -> {}", bar);
}

fn main() {
    ownership();
}
