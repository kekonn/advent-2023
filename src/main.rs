/// https://adventofcode.com/2023/day/1
mod day1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("---------- Day 1 ----------");
    {
        let pt2_input = tokio::fs::File::open("data/day1_pt2.txt").await.expect("Error opening day1 pt. 2 input");
        println!("Result: {}", day1::day1(pt2_input).await);
    }
    println!("---------------------------");

    Ok(())
}
