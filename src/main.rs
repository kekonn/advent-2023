/// https://adventofcode.com/2023/day/1
mod day1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("---------- Day 1 ----------");
    {
        let input = tokio::fs::File::open("data/day1.txt").await.expect("Error opening day1 input");
        println!("Result: {}", day1::day1(input).await);
    }
    println!("---------------------------");

    Ok(())
}
