use std::collections::VecDeque;

type Maze = Vec<Vec<u8>>;

fn solve(
    maze: &mut Maze,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, vec![start]));
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some(((x, y), path)) = queue.pop_front() {
        if (x, y) == end {
            return Some(path);
        }
        for (dx, dy) in &dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if ny < maze.len() && nx < maze[0].len() && maze[ny][nx] == 1 {
                    maze[ny][nx] = 2;
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push_back(((nx, ny), new_path));
                }
            }
        }
    }
    None
}

fn main() {
    let mut maze = vec![vec![1, 0, 1, 1], vec![1, 1, 0, 1], vec![0, 1, 1, 1]];
    if let Some(path) = solve(&mut maze, (0, 0), (3, 2)) {
        println!("Path found: {:?}", path);
    } else {
        println!("No path");
    }
}
