// println!でstr1が表示できるように4行目を修正してください
fn main() {
    let str1 = String::from("hoge");
    let _str2 = str1.to_string(); // もしくはコメントアウトでstr1の借用を消す

    println!("Hello, world! {}", str1);
}
