fn ownership2() {
    let foo = vec![1, 2, 3];
    let mut bar = foo;

    bar.push(4);

    println!("{:?}", foo);
    println!("{:?}", bar);
}


fn main() {
    ownership2();
}

//def ownership2():
//    foo = [1, 2, 3]
//    bar = foo
//
//    bar.append(4)
//
//    print(foo)
//    print(bar)
