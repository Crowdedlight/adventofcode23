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
    let options: Vec<(i32, i32, &str, char)> = vec![(1,0,"-J7", 'E'), (-1,0, "-LF", 'W'), (0,-1,"|7F", 'N'), (0,1,"|LJ", 'S')];
    for (x,y,option, dir) in options {
        // get position and check if string contains element
        let x_new = start_pos.0 + x;
        let y_new = start_pos.1 + y;

        if let Some(char) = m.get(x_new, y_new) {
            // it exists, but is it valid symbol
            if option.contains(char) {
                let new_pos = (x_new, y_new, 1, dir);
                way.push_back(new_pos);
                break;
            }
        }
    }

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

    // we now have the big path, lets try and print it
    let mut m_loop = m.clone();
    // draw path
    for (x,y) in big_path.clone() {
        // set character
        m_loop.set(x,y,'*');
    }

    // print map
    for row in m_loop.rows.iter() {
        println!("{:?}", row);
    }
    println!("------");

    // todo, what if we go through all symbols on a row, saving pos that is between * symbols
    //  The chars between a start and stop is the ones we need to count, for visual reference, we can set all of those to I.
    let mut not_nests: Vec<(i32, i32)> = vec![];

    let mut all_possibilities: Vec<(i32, i32)> = vec![];

    // connected search
    let mut queue: VecDeque<(i32, i32)> = VecDeque::with_capacity(100);
    let x_len = m_loop.rows.first().unwrap().len();
    let y_len = m_loop.rows.len();

    // add start positions, which is all edge positions that is not in path
    for x in 0..x_len {
        let new_pos = (x as i32, 0);
        if !big_path.contains(&new_pos) && !queue.contains(&new_pos) {
            queue.push_back(new_pos);
        }
    }
    for x in 0..x_len {
        let new_pos = (x as i32, (y_len-1) as i32);
        if !big_path.contains(&new_pos) && !queue.contains(&new_pos) {
            queue.push_back(new_pos);
        }
    }
    for y in 0..y_len {
        let new_pos = (0, y as i32);
        if !big_path.contains(&new_pos) && !queue.contains(&new_pos) {
            queue.push_back(new_pos);
        }
    }
    for y in 0..y_len {
        let new_pos = ((x_len-1) as i32, y as i32);
        if !big_path.contains(&new_pos) && !queue.contains(&new_pos) {
            queue.push_back(new_pos);
        }
    }

    // go through queue
    while !queue.is_empty() {
        // get element
        let (x, y) = queue.pop_front().unwrap();
        not_nests.push((x, y));

        // check each neighbour, if not part of big_loop, then its not_nest
        let options: Vec<(i32, i32)> = vec![(0,1),(0,-1),(1,0),(-1,0)];
        for option in options {
            let new_pos = (option.0 + x, option.1 + y);

            // check bounds
            if new_pos.0 < 0 || new_pos.0 >= x_len as i32 || new_pos.1 < 0 || new_pos.1 >= y_len as i32 {
                continue;
            }

            // neither loop, nor already visited nodes should contain it before its new
            if !big_path.contains(&new_pos) && !not_nests.contains(&new_pos) {
                queue.push_back(new_pos);
            }
        }
    }

    // remove all not_nests from all_possibilities
    // let filtered_nests: Vec<(i32, i32)> = all_possibilities.into_iter().filter(|x| !not_nests.contains(x)).collect();

    // set Os for not-nest
    for (x, y) in not_nests.iter() {
        m_loop.set(*x,*y,'O');
    }

    // println!("{:?}", filtered_nests);

    for row in m_loop.rows {
        println!("{:?}", row);
    }
    println!("----");

    not_nests.sort();
    not_nests.dedup();
    // sum of enclosed tiles must be all_tiles - path.len() - not_nest.len();
    let sum: u32 = (x_len * y_len) as u32 - big_path.len() as u32 - not_nests.len() as u32;


    // todo find sub-loops. Smaller loops of the big loop where the loop is adjecant to eachother. So save big loop in vector with x,y from one end to another, not double start as before
    //  end condition for big loop is to find S
    //  then find ranges of x,y in vector that is not next to eachother in index, but where x,y is neighbours. That should denote a subloop.
    //  And any positions that is inside a subloop, should be a position that can be a nest.

    // TODO consider cloning matrix and using functions to mark the new subloops with * or something similar. Easier to debug visually for errors?

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