/// https://adventofcode.com/2023/day/1
mod day1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    println!("---------- Day 1 ----------");
    let day1_input = tokio::fs::File::open("data/day1.txt").await.expect("Error opening day1 input");
    let day1_result = day1::day1(day1_input).await;
    println!("Result: {}", day1_result);
    println!("---------------------------");

    Ok(())
}
