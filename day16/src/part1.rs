
#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    MirrorRightAngle,
    MirrorLeftAngle,
    SplitterVertical,
    SplitterHorizontal
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub symbol: char,
    pub visited: bool,
    // pub location: (i32, i32)
}
impl Tile {
    pub fn new(symbol: char) -> Self {

        let tile_type = match symbol {
            '.' => TileType::Empty,
            '/' => TileType::MirrorRightAngle,
            '\\' => TileType::MirrorLeftAngle,
            '|' => TileType::SplitterVertical,
            '-' => TileType::SplitterHorizontal,
            _ => panic!("Symbol isn't known for parsing")
        };

        let visited = false;

        Self { tile_type, symbol, visited }
    }
}

#[derive(Debug, Default)]
pub struct Matrix {
    pub rows: Vec<Vec<Tile>>
}
impl Matrix {
    pub fn get_ref(&self, x: i32, y:i32) -> Option<&Tile> {
        Some(self.rows.get(y as usize)?.get(x as usize)?)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        let newV = row.iter().map(|x| Tile::new(*x)).collect();
        self.rows.push(newV);
    }
    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{}", row.iter().map(|x| x.symbol).collect::<String>());
        }
    }
    pub fn print_pretty(&self) {
        for row in self.rows.iter() {
            // TODO
            //  make it print where it replaces chars with unicode symbols for path
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

    m.print();

    Ok(0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
                assert_eq!("46", process(input)?);
                Ok(())
    }
}