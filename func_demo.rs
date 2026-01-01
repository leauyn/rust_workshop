fn add(a: i32, b:i32) -> i32 {
    a + b
}

// let y = add(3, 0);
// let z = add(x, 1);

fn main(){

   let x = add(1, 1);
   println!("{:?}", x);
   println!("{:?} {:?}", x, x);
}