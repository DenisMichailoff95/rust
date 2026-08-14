fn next_gen(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut next = vec![vec![false; cols]; rows];
    for y in 0..rows {
        for x in 0..cols {
            let mut live = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if ny >= 0 && nx >= 0 && ny < rows as i32 && nx < cols as i32 {
                        if grid[ny as usize][nx as usize] { live += 1; }
                    }
                }
            }
            next[y][x] = match (grid[y][x], live) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    next
}

fn main() {
    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for _ in 0..5 {
        for row in &grid {
            println!("{}", row.iter().map(|&b| if b { 'X' } else { '.' }).collect::<String>());
        }
        println!("---");
        grid = next_gen(&grid);
    }
}
