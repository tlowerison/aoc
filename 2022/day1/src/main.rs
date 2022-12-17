use anyhow::Error;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two(3)?);
    Ok(())
}

fn part_one() -> Result<u32, Error> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut max_calories = 0u32;
    let mut calories = 0u32;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
        } else {
            let calory: u32 = line.parse()?;
            calories += calory;
        }
    }

    if calories > max_calories {
        max_calories = calories;
    }

    Ok(max_calories)
}

fn part_two(num_top_elves: usize) -> Result<i32, Error> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    // note: values are negated in order to implement a min-heap
    let mut top_calories = BinaryHeap::with_capacity(num_top_elves);
    let mut calories = 0i32;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            if calories > 0 {
                if top_calories.len() < num_top_elves {
                    top_calories.push(-calories);
                } else {
                    match top_calories.peek() {
                        Some(peeked) => if calories > -peeked {
                            top_calories.pop();
                            top_calories.push(-calories);
                        },
                        None => top_calories.push(-calories)
                    }
                }
                calories = 0;
            }
        } else {
            let calory: i32 = line.parse()?;
            calories += calory;
        }
    }
 
    if calories > 0 {
        if top_calories.len() < num_top_elves {
            top_calories.push(-calories);
        } else {
            match top_calories.peek() {
                Some(peeked) => if calories > -peeked {
                    top_calories.pop();
                    top_calories.push(-calories);
                },
                None => top_calories.push(-calories)
            }
        }
    }

    Ok(-top_calories.into_iter().sum::<i32>())
}
