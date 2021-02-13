// countの値が出力されるように1箇所修正してください
fn main() {
    let mut count = 0u32; // mut をつけてcountを可変にする
    while count < 10 {
        count += 1;
        println!("count={}", count);
    }
}
