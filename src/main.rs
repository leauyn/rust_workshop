fn first_name(){
    println!("Jason");
}

fn last_name(){
    println!("TOm");
}

fn sub(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
    first_name();
    last_name();

    let res = sub(10, 8);
    println!("{:?}", res);

    let some_bool = true;
    match some_bool {
        true => println!("This is true"),
        false => println!("This is false"),
    }
}
