use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, row: i32, col:i32) -> Option<char> {
        Some(*self.rows.get(row as usize)?.get(col as usize)?)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut start_pos = (0i32, 0i32);
    let mut m: Matrix = Matrix::default();
    // fill 2d array
    for (i, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        m.add_row(row.clone());

        // check if row contains S
        let x_pos = row.iter().position(|&x| x == 'S');
        if x_pos.is_some() {
            start_pos = (x_pos.unwrap() as i32, i as i32);
        }
    }

    // dequeue for elements
    let mut way: VecDeque<(i32, i32, u32, char)> = VecDeque::with_capacity(100);

    // determine S pipe => EAST, WEST, NORTH, SOUTH. Only two valid exists
    let options: Vec<(i32, i32, &str, char)> = vec![(1,0,"-J7", 'E'), (-1,0, "-LF", 'W'), (0,-1,"|7F", 'N'), (0,1,"|LJ", 'S')];
    for (x,y,option, dir) in options {
        // get position and check if string contains element
        let x_new = start_pos.0 + x;
        let y_new = start_pos.1 + y;

        if let Some(char) = m.get(y_new, x_new) {
            // it exists, but is it valid symbol
            if option.contains(char) {
                let new_pos = (x_new, y_new, 1, dir);
                way.push_back(new_pos);
            }
        }
    }

    // todo could make it simpler, simply go from one end until completed loop and back at S, and highest cost would be half the path length

    let mut sum = 0;
    // go through queue until we meet
    while !way.is_empty() {
        // get current element
        let (x, y, cost, dir) = way.pop_front().unwrap();
        println!("At ({:?},{:?}, with Dir: {:?} and cost: {:?}", x, y, dir, cost);

        // check finish conditions. If there is another element in queue with same x,y then we have completed the loop
        if way.iter().filter(|&item| item.0 == x && item.1 == y).count() == 1 {
            sum = cost;
            break;
        }

        // get symbol
        if let Some(char) = m.get(y, x) {
            // match symbol
            let (next_x, next_y, next_dir) = match (char, dir) {
                ('|', 'N') => (x, y-1, 'N'),
                ('|', 'S') => (x, y+1, 'S'),
                ('-', 'E') => (x+1, y, 'E'),
                ('-', 'W') => (x-1, y, 'W'),
                ('L', 'W') => (x, y-1, 'N'),
                ('L', 'S') => (x+1, y, 'E'),
                ('J', 'E') => (x, y-1, 'N'),
                ('J', 'S') => (x-1, y, 'W'),
                ('7', 'N') => (x-1, y, 'W'),
                ('7', 'E') => (x, y+1, 'S'),
                ('F', 'N') => (x+1, y, 'E'),
                ('F', 'W') => (x, y+1, 'S'),
                _ => continue,
            };

            // add next pos to queue with cost
            way.push_back((next_x, next_y, cost+1, next_dir));
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
                assert_eq!("4", process(input)?);
                Ok(())
    }

    #[test]
    fn test_process2() -> anyhow::Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
                assert_eq!("8", process(input)?);
                Ok(())
    }
}