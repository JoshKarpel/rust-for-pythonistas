fn ownership() {
    let foo = 1;
    let mut bar = 2;

    bar += 5;

    println!("foo -> {}", foo);
    println!("bar -> {}", bar);
}

fn main() {
    ownership();
}

//def ownership():
//    foo = 'foo'
//    bar = foo
//
//    bar += 'bar'
//
//    print(foo)
//    print(bar)
