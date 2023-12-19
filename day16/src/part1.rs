
enum TileType {
    Empty,
    MirrorRightAngle,
    MirrorLeftAngle,
    SplitterVertical,
    SplitterHorizontal
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    pub tile_type: TileType,
    pub symbol: char,
    pub visited: bool,
    pub location: (i32, i32)
}
impl Tile {
    pub fn new(symbol: char, location: (i32, i32)) -> Self {

        let tile_type = match symbol {
            '.' => TileType::Empty,
            '/' => TileType::MirrorRightAngle,
            '\\' => TileType::MirrorLeftAngle,
            '|' => TileType::SplitterVertical,
            '-' => TileType::SplitterHorizontal,
            _ => panic!("Symbol isn't known for parsing")
        };

        let visited = false;

        Self { tile_type, symbol, visited, location }
    }
}

#[derive(Debug, Default)]
pub struct Matrix {
    pub rows: Vec<Vec<Tile>>
}
impl Matrix {
    pub fn get_ref(&self, row: i32, col:i32) -> Option<&Tile> {
        Some(self.rows.get(row as usize)?.get(col as usize)?)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        // todo 
        let newV = row.iter().map(|x| Tile::new())
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

    Ok(0.to_string())
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