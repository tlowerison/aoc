use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    println!("{}", part_one()?);
    println!("{}", part_two()?);
    Ok(())
}

fn part_one() -> Result<u32, Error> { 
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut score = 0u32;
        
    for line in lines {
        let line = line?;
        if line.len() < 3 {
            continue;
        }
        let choice_score = match &line[2..3] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("unexpected choice pattern: {line}"),
        };
        let outcome_score = match &line[0..3] {
            "A X" => 3,
            "A Y" => 6,
            "A Z" => 0,
            "B X" => 0,
            "B Y" => 3,
            "B Z" => 6,
            "C X" => 6,
            "C Y" => 0,
            "C Z" => 3,
            _ => panic!("unexpected outcome pattern: {line}"),
        };
        score += choice_score + outcome_score;
    }

    Ok(score)
}

fn part_two() -> Result<u32, Error> { 
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut score = 0u32;
        
    for line in lines {
        let line = line?;
        if line.len() < 3 {
            continue;
        }
        let outcome_score = match &line[2..3] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("unexpected outocme pattern: {line}"),
        };
        let choice_score = match &line[0..3] {
            "A X" => 3, // lose to rock -> scissors
            "A Y" => 1, // tie against rock -> rock
            "A Z" => 2, // win against rock -> paper
            "B X" => 1, // lose to paper -> rock
            "B Y" => 2, // tie against paper -> paper
            "B Z" => 3, // win against paper -> scissors
            "C X" => 2, // lose against scissors -> paper
            "C Y" => 3, // tie against scissors -> scissors
            "C Z" => 1, // win against scissors -> rock
            _ => panic!("unexpected outcome pattern: {line}"),
        };
        score += choice_score + outcome_score;
    }

    Ok(score)
}

