#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hour: i64
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hour: 40
    };

    println!("{:?}", me.position);
    println!("{:?}", me);
    println!("{:?}", me);

    //match me.position {
    //    Position::Manager => println!("manager"),
    //    Position::Supervisor => println!("supervisor"),
    //    Position::Worker => println!("worker"),
    //}
}