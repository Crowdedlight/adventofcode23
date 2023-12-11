use itertools::{Itertools};

#[derive(Debug, Default)]
pub struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, row: i32, col:i32) -> Option<char> {
        Some(*self.rows.get(row as usize)?.get(col as usize)?)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }
    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    // parse input
    let mut m: Matrix = Matrix::default();

    // load in data structure
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        m.add_row(row);
    }

    // print matrix
    // m.print();

    // find empty cols and rows
    let mut empty_row_index: Vec<usize> = vec![];
    let mut empty_col_index: Vec<usize> = vec![];

    let mut x_len = m.rows.first().unwrap().len();
    let mut y_len = m.rows.len();

    for (i, row) in m.rows.iter().enumerate() {
        if !row.contains(&'#') {
            empty_row_index.push(i);
        }
    }
    for x in 0..x_len {
        let mut galaxy = false;
        for y in 0..y_len {
            if m.rows[y][x] == '#' {
                galaxy = true;
                break;
            }
        }
        if !galaxy {
            empty_col_index.push(x);
        }
    }

    // instead of enlarging, we could just check how many enlarge indexes that x and y respectially go through, and add enlarge_value to those
    let enlarge_val = 1000000 -1;

    // find all galaxies
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for y in 0..y_len {
        for x in 0..x_len {
            if m.rows[y][x] == '#' {
                galaxies.push((x,y));
            }
        }
    }
    // label galaxies
    for (i, (x,y)) in galaxies.iter().enumerate() {
        m.rows[*y][*x] = i.to_string().chars().next().unwrap();
    }

    // println!("----- Id'ed all galaxies -----");
    // m.print();

    // make pairs of all galaxies, (1, 9) is same as (9, 1)
    let it = (0..galaxies.len()).combinations_with_replacement(2);
    let mut galaxy_pair: Vec<((i32,i32), (i32, i32))> = vec![];
    for pair in it {
        // don't add ourselfs
        if pair[0] == pair[1] {
            continue;
        }

        let g1 = (galaxies[pair[0]].0 as i32, galaxies[pair[0]].1 as i32);
        let g2 = (galaxies[pair[1]].0 as i32, galaxies[pair[1]].1 as i32);

        galaxy_pair.push((g1, g2));
    }

    let mut dist_result: Vec<u64> = vec![];
    // find manhatten distance between each pair of galaxies
    for (g1, g2) in galaxy_pair.iter() {
        let mut dist: u64 = (g2.0 - g1.0).abs() as u64 + (g2.1 - g1.1).abs() as u64;

        // check how many enlarged indexes that are between g1 and g2 x, and add enlarge distance to dist
        for index in empty_col_index.iter() {
            let range = g1.0.min(g2.0) as usize..g1.0.max(g2.0) as usize;
            if range.contains(&index) {
                dist += enlarge_val;
            }
        }
        for index in empty_row_index.iter() {
            let range = g1.1.min(g2.1) as usize..g1.1.max(g2.1) as usize;
            if range.contains(&index) {
                dist += enlarge_val;
            }
        }

        dist_result.push(dist);
    }

    Ok(dist_result.iter().sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
                assert_eq!("8410", process(input)?);
                Ok(())
    }
}