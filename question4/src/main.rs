// 7行目のscoresが表示されるように修正してください
fn main() {
    let scores = vec![100, 92, 84, 75, 98, 81];
    for score in scores.iter() { // into_iter() -> iter()
        println!("{}", score);
    }
    println!("{:?}", scores);
}
