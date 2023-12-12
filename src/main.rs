/// https://adventofcode.com/2023/day/1
mod day1;
/// https://adventofcode.com/2023/day/2
mod day2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("---------- Day 1 ----------");
    {
        let input = tokio::fs::File::open("data/day1.txt").await.expect("Error opening day1 input");
        println!("Result: {}", day1::day1(input).await);
    }
    println!("---------------------------");

    println!("---------- Day 2 ----------");
    {
        let input = std::fs::File::open("data/day2.txt").expect("Error opening day2 input");
        println!("Result: {}", day2::day2(input, (12, 13, 14)));
    }
    println!("---------- Pt. 2 ----------");
    {
        let input = std::fs::File::open("data/day2.txt").expect("Error opening day2 input");
        println!("Result: {}", day2::day2_pt2(input));
    }
    println!("---------------------------");
    Ok(())
}
