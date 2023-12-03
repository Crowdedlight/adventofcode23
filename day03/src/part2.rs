use std::collections::HashMap;
use crate::part1::Matrix;

impl Matrix {
    fn get_this_symbol_pos_if_adjacent(&self, row:i32, col:i32, symbol:char) -> (i32, i32) {
        // check if any symbol exists adjecent. So that will be 8 options:
        let options: Vec<(i32,i32)> = vec![(0,1),(0,-1),(1, 0),(-1, 0),(1,1),(-1,-1),(1,-1),(-1,1)];
        for (r, c) in options {
            if let Some(char) = self.get(row+r, col+c) {
                // it exists, but is it a symbol?
                if char == symbol {
                    return (row+r, col+c)
                }
            }
        }
        (-1, -1)
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut m: Matrix = Matrix::default();

    // load in data structure
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        m.add_row(row);
    }

    // let mut sum:u32 = 0;
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();

    // run through each element of each row until we hit a digit
    let col_num = m.rows.first().unwrap().len();
    let row_num = m.rows.len();

    for row in 0..row_num as i32 {
        let mut digits: Vec<u32> = vec![];
        let mut offset = 1;
        for col in 0..col_num as i32 {

            if let Some(d) = m.get_if_digit(row, col) {
                // exists, add to digits
                digits.push(d);

                // if not at end of column and we have valid digit, we continue the loop to get all digits
                if col != (col_num-1) as i32 {
                    continue;
                }
                // else we are at the end, thus set offset to 0
                offset = 0;
            }
            // we only reach here if we no longer have a valid digit, or if at end of string
            for x in 0..digits.len() as i32 {
                let (r, c) = m.get_this_symbol_pos_if_adjacent(row, col - offset - x, '*');

                if r != -1 && c != -1 {
                    // make key
                    let key = format!("{},{}", r, c);

                    // parse digits for val
                    let val:u32 = digits.iter().map(|i| i.to_string()).collect::<String>().parse::<u32>()?;

                    // push to hashmap
                    match gears.get_mut(&key) {
                        Some(value) => {
                            if !value.contains(&val) {
                                value.push(val);
                            }
                        },
                        None => {
                            gears.insert(key, vec![val]);
                        }
                    }
                }
            }
            // in all cases we clear digits vec
            digits.clear();
        }
    }

    // now parsed all input, we should have a neat hashmap and we can throw everything away that doesn't have a vector of size 2.
    gears.retain(|_, v| v.len() == 2);

    let sum:u32 = gears.values().map(|x| x.iter().product::<u32>()).sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}