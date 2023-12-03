
#[derive(Debug, Default)]
pub struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, row: i32, col:i32) -> Option<char> {
        Some(*self.rows.get(row as usize)?.get(col as usize)?)
    }
    pub fn get_if_digit(&self, row: i32, col:i32) -> Option<u32> {
        let val = Some(*self.rows.get(row as usize)?.get(col as usize)?);

        val.unwrap_or('n').to_digit(10)
    }
    fn is_symbol_adjacent(&self, row:i32, col:i32) -> bool {
        // check if any symbol exists adjecent. So that will be 8 options:
        let options: Vec<(i32,i32)> = vec![(0,1),(0,-1),(1, 0),(-1, 0),(1,1),(-1,-1),(1,-1),(-1,1)];
        for (r, c) in options {
            if let Some(char) = self.get(row+r, col+c) {
                // it exists, but is it a symbol?
                if !char.is_numeric() && char != '.' {
                    return true
                }
            }
        }
        false
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn check_for_adjecent_symbols(&self, digits:Vec<u32>, row:i32, col:i32, offset:i32) -> bool {
        for x in 0..digits.len() as i32 {
            if self.is_symbol_adjacent(row, col - offset - x) {
                return true
            }
        }
        false
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    // parse input into 2D matrix with a char per position. Makes it easier to test of adjecent symbols?
    // have to ensure to check bounds or use Some/None to handle checking index pos that doesn't exist in array?
    let mut m: Matrix = Matrix::default();

    // load in data structure
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        m.add_row(row);
    }

    let mut sum:u32 = 0;

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
            // As function here checks based on digits vector, an empty vector will return false too
            if m.check_for_adjecent_symbols(digits.clone(), row, col, offset) {
                // if digits are not empty and has symbols we parse and add to sum
                let val:u32 = digits.iter().map(|i| i.to_string()).collect::<String>().parse::<u32>()?;
                sum += val;
            }
            // in all cases we clear digits vec
            digits.clear();
        }
    }
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}