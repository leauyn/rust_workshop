struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test {score: 90},
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 66 },
    ];

    for score_item in &my_scores {
        println!("score: {:?}", score_item.score);
    }

    println!("number of elements = {:?}", my_scores.len());

}