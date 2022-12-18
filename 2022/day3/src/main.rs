use anyhow::Error;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

const GROUP_SIZE: usize = 3;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|x| match x {
            Ok(x) => match x.is_empty() {
                false => Some(Ok(x)),
                true => None,
            },
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part one: {}", part_one(&lines)?);
    println!("Part two: {}", part_two(&lines, GROUP_SIZE)?);

    Ok(())
}

fn item_priority(item: &char) -> Result<u32, Error> {
    let ascii_code = item.to_digit(36).ok_or_else(|| Error::msg(format!("found non ascii duplicate: '{item}'")))?;
    let is_uppercase = item.is_ascii_uppercase();
    
    match ascii_code {
        10..=35 => Ok(ascii_code - 10 + 1 + if is_uppercase { 26 } else { 0 }),
        _ =>Err(Error::msg(format!("unsupported duplicate ascii code: {ascii_code}, char: '{item}'"))),
    }
}

fn part_one(lines: &[String]) -> Result<u32, Error> {
    let mut first_rucksack_items = HashSet::new();
    let mut duplicate_priority_total = 0;
    for line in lines {
        first_rucksack_items.drain();

        first_rucksack_items.extend(line[0..line.len()/2].chars());

        let mut duplicate = None;

        for item in line[line.len()/2..].chars() {
            if first_rucksack_items.contains(&item) {
                duplicate = Some(item);
                break;
            }
        }
        let duplicate = duplicate.ok_or_else(|| Error::msg(format!("no duplicate found for line: '{line}'")))?;
        duplicate_priority_total += item_priority(&duplicate)?;
    }

    Ok(duplicate_priority_total)
}

fn part_two(lines: &[String], group_size: usize) -> Result<u32, Error> {
    let chunks = lines.chunks(GROUP_SIZE).collect::<Vec<_>>();
    let allocations: Arc<Mutex<Vec<Vec<HashSet<char>>>>> = Arc::new(Mutex::new(Vec::default()));

    Ok(chunks
        .par_iter()
        .map(|chunk| {
            let mut all_items = {
                let mut allocations = allocations.lock().map_err(|err| Error::msg(format!("unable to lock allocations mutex: {err}")))?;
                allocations
                    .pop()
                    .unwrap_or_else(|| {
                        let mut all_items = Vec::with_capacity(group_size);
                        for _ in 0..group_size {
                            all_items.push(HashSet::default());
                        }
                        all_items
                    })
            };

            for (line, items) in chunk.iter().zip(all_items.iter_mut()) {
                items.extend(line.chars());
            }

            // sort item groups by size in ascending order to make intersection more efficient
            all_items.sort_by(|a, b| a.len().cmp(&b.len()));

            let mut intersection = all_items[0]
                .iter()
                .filter(|k| all_items.iter().all(|items| items.contains(k)))
                .collect::<Vec<_>>();

            let group_badge = match intersection.len() {
                0 => return Err(Error::msg("no intersection found between group priorities")),
                1 => intersection.pop().unwrap(),
                _ => return Err(Error::msg("more than one intersection found between group priorities")),
            };
            let group_priority = item_priority(group_badge)?;

            for items in all_items.iter_mut() {
                items.drain();
            }

            let mut allocations = allocations.lock().map_err(|err| Error::msg(format!("unable to lock allocations mutex: {err}")))?;
            allocations.push(all_items);

            Ok(group_priority)

        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .sum())
}

