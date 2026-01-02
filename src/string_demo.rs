fn print_it(data: &str) {
    println!("{:?}", data);
}

fn print(data: &str) {
    println!("{:?}", data);
}

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}


fn main() {
    print_it("a string slice");
    let owned_string = "Owned string".to_owned();
    let another_string = String::from("another");
    print_it(&owned_string);
    print_it(&another_string);

    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age:7,
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::from("purple"),
            age:9,
        },
        Person {
            name: String::from("Katie"),
            fav_color: String::from("blue"),
            age:12,
        },
    ];

    for person in people {
        if person.age < 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}