use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default, Clone)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, x: i32, y:i32) -> Option<char> {
        Some(*self.rows.get(y as usize)?.get(x as usize)?)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }
    pub fn set(&mut self, x: i32, y:i32, c: char) {
        self.rows[y as usize][x as usize] = c;
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
    let mut big_path: Vec<(i32, i32)> = vec![];

    // determine S pipe => EAST, WEST, NORTH, SOUTH. Only two valid exists
    let options: Vec<(i32, i32, &str, char)> = vec![(1,0,"-J7", 'E'), (-1,0, "-FL", 'W'), (0,-1,"|7F", 'N'), (0,1,"|LJ", 'S')];
    for (x,y,option, dir) in options.iter() {
        // get position and check if string contains element
        let x_new = start_pos.0 + x;
        let y_new = start_pos.1 + y;

        if let Some(char) = m.get(x_new, y_new) {
            // it exists, but is it valid symbol
            if option.contains(char) {
                let new_pos = (x_new, y_new, 1, *dir);
                way.push_back(new_pos);
                continue;
            }
        }
    }
    // have to set start symbol here, while we know what the start pos options are
    let start_symbol = match (way.iter().nth(0).unwrap().3, way.iter().nth(1).unwrap().3) {
        ('N', 'E') => 'L',
        ('E', 'N') => 'L',
        ('N', 'S') => '|',
        ('S', 'N') => '|',
        ('N', 'W') => 'J',
        ('W', 'N') => 'J',
        ('S', 'W') => '7',
        ('W', 'S') => '7',
        ('S', 'E') => 'F',
        ('E', 'S') => 'F',
        ('W', 'E') => '-',
        ('E', 'W') => '-',
        _ => 'S'
    };

    println!("Start symbol S is a: {:?} at location: {:?}", start_symbol, start_pos);

    while !way.is_empty() {
        // get current element
        let (x, y, cost, dir) = way.pop_front().unwrap();

        // add curr position to path
        big_path.push((x,y));

        // println!("At ({:?},{:?}, with Dir: {:?} and cost: {:?}", x, y, dir, cost);

        // get symbol
        if let Some(char) = m.get(x, y) {
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
                ('S', _) => break,
                _ => continue,
            };

            // add next pos to queue with cost
            way.push_back((next_x, next_y, cost+1, next_dir));
        }
    }

    // replace S with its symbol
    m.set(start_pos.0, start_pos.1, start_symbol);

    // we now have the big path, lets try and print it
    let mut m_loop = m.clone();
    // draw path
    for (i, (x,y)) in big_path.clone().iter().enumerate() {
        // set character
        m_loop.set(*x,*y,i.to_string().chars().next().unwrap());
    }

    // print map
    for row in m.rows.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }
    println!("------");

    let mut sum: u32 = 0;

    // go through each row
    for (y, row) in m.rows.iter().enumerate() {
        let mut crossings: u32 = 0;

        for (x, char) in row.iter().enumerate() {
            // check if crossing
            if big_path.contains(&(x as i32, y as i32)) && "|7F".contains(*char) {
                crossings += 1;
            } else if crossings % 2 != 0 && !big_path.contains(&(x as i32, y as i32)) {
                // if divisible with 2, we are inside
                sum += 1;
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> anyhow::Result<()> {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process3() -> anyhow::Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", process(input)?);
        Ok(())
    }
}