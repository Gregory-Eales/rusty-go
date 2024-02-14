use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::fs::File;
use std::fs;

use indicatif::ProgressBar;
use csv::Writer;


fn create_map() -> HashMap<char, usize> {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let mut move_map = HashMap::with_capacity(chars.len());

    for (index, &char) in chars.iter().enumerate() {
        move_map.insert(char, index);
    }

    move_map
}


fn process_file(path: String) {
    // takes in a string
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => panic!("Could not read file"),
    };

    let mut idx = 0;
    let chars: Vec<char> = contents.chars().collect();
    let str_len = chars.len();

    let m = create_map();

    let mut grid: [[i32; 9]; 9] = [[0; 9]; 9];

    while idx < str_len - 1 {
        // if line contains B[ or W[ then it is a move.
        // check from idx to idx + 2 using slice contents[idx..idx+2]
        if (chars[idx] == 'B' || chars[idx] == 'W') && chars[idx+1] == '[' {
            let p1_option = m.get(&chars[idx+2]);
            let p2_option = m.get(&chars[idx+3]);
    
            match (p1_option, p2_option) {
                (Some(&p1), Some(&p2)) => {
                    // Both p1 and p2 are found, proceed with the move
                    // if black then set to -1, if white set to 1
                    if chars[idx] == 'B' && p1 < 9 && p2 < 9{
                        grid[p1][p2] = 1;
                        remove_dead_stones(&mut grid, 1, p1, p2);
                    }
                    
                    if chars[idx] == 'W' && p1 < 9 && p2 < 9 {
                        grid[p1][p2] = -1;
                        remove_dead_stones(&mut grid, -1, p1, p2);
                    }
                },
                _ => {
                }
            }
        }
        idx += 1;

        if false {
            print_board(&grid);
        }
    }
}

fn print_board(board: &[[i32; 9]; 9]) {
    // print the board
    for i in 0..9 {
        for j in 0..9 {
            // if -1 add one space, else then add two spaces
            if board[i][j] == -1 {
                print!("{} ", board[i][j]);
            } else {
                print!(" {} ", board[i][j]);
            }
        }
        println!();
    }
    println!();
    println!();
}


fn remove_dead_stones(board: &mut [[i32; 9]; 9], color: i32, i: usize, j: usize) {
    // check the adjacent positions to see if any stones were captured
    let mut positions = HashSet::from([(i+1, j), (i, j+1)]);
    if i > 0 {
        positions.insert((i-1, j));
    }
    if j > 0 {
        positions.insert((i, j-1));
    }
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in positions {
        if visit_stones(color, x, y, &mut visited, board) {
            for &(a, b) in visited.iter() {
                board[a][b] = 0;
            }
        }
        visited.clear();
    }
}

// takes in a visited HashSet refrence that we fill
// with board positions. returns bool
fn visit_stones(color : i32, i: usize, j: usize, visited: &mut HashSet<(usize, usize)> , board: &[[i32; 9]; 9]) -> bool {

    if i >= 9 || j >= 9 {
        return true;
    }

    if visited.contains(&(i, j)) {
        return true;
    }

    if board[i][j] == color {
        return true;
    }

    if board[i][j] == 0 {
        return false;
    }

    visited.insert((i, j));

    let right = visit_stones(color, i+1, j, visited, board);
    let down = visit_stones(color, i, j+1, visited, board);
    let mut left = true;
    let mut up = true;

    if j > 0 {
        up = visit_stones(color, i, j-1, visited, board);
    }
    if i > 0 {
        left = visit_stones(color, i-1, j, visited, board);
    }

    if left && right && up && down {
        return true;
    }

    false
}


fn main() {
    println!("Getting filenames");

    let paths: Vec<_> = fs::read_dir("ogs_games").unwrap().collect();
    let mut num_files = 0;
    let start = Instant::now();
    let num_items = paths.len() as u64;

    println!("Processing {} files:", num_items);
    let bar = ProgressBar::new(num_items);

    for path in paths {

        bar.inc(1);
        num_files += 1;

        let path_str = path.unwrap().path().display().to_string();
        process_file(path_str);
    }
    bar.finish();
    let duration = start.elapsed(); 

    println!("Files Loaded: {}", num_files);
    println!("Time taken: {:?}", duration);
    println!("Time taken (milliseconds): {:?}", duration.as_millis());
    println!("Time taken per game (ms): {:?}", duration.as_millis() / num_files);

}
