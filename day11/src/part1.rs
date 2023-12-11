use itertools::{Itertools, unfold};

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

    // enlarge each empty col and row
    for (n, index) in empty_row_index.iter().enumerate() {
        let y_index = index + n;
        m.rows.insert(y_index, vec!['.'; x_len])
    }
    // insert col, which means insert into all rows for col index
    for (n, index) in empty_col_index.iter().enumerate() {
        // insert single character in each row on col index
        // every time we insert a new column, we need to increase index by 1, as the matrix grows
        let x_index = index + n;
        for row_index in 0..m.rows.len() {
            m.rows[row_index].insert(x_index, '.');
        }
    }

    x_len = m.rows.first().unwrap().len();
    y_len = m.rows.len();

    // println!("----- Enlarged the system ----- ");
    // m.print();

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

    let mut dist_result: Vec<u32> = vec![];
    // find manhatten distance between each pair of galaxies
    for (g1, g2) in galaxy_pair.iter() {
        let dist: i32 = (g2.0 - g1.0).abs() + (g2.1 - g1.1).abs();
        dist_result.push(dist as u32);
    }

    Ok(dist_result.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
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
                assert_eq!("374", process(input)?);
                Ok(())
    }
}