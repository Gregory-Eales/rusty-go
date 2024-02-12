use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
// use std::collections::VecDequeue;
/*

ok so how do we want this code to work?

1. load all the files paths that we want to run through
2. loop through each of the files:
    - initialize an empty board to play on
    - load up the file as a string
    - pass through the string and make moves on the board, w / black
    - if a group is captured then we should remove it and continue playing pieces
    - save each unique board state / the winner of the game
*/

//static MOVE_MAP: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k'];

// actually we want a hashmap that maps a : 0, b : 1, c : 2, etc


fn create_map() -> HashMap<char, usize> {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k'];
    let mut move_map = HashMap::with_capacity(chars.len());

    for (index, &char) in chars.iter().enumerate() {
        move_map.insert(char, index);
    }

    move_map
}

fn process_file(path: String) {
    // takes in a string
    let mut contents = fs::read_to_string(path);

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
                    //println!("{} moved to location: {} {}", chars[idx], p1, p2);

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
    }

    // print the board
    for i in 0..9 {
        for j in 0..9 {
            // if -1 add one space, else then add two spaces
            if grid[i][j] == -1 {
                print!("{} ", grid[i][j]);
            } else {
                print!(" {} ", grid[i][j]);
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
    for (x, y) in positions {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        if !visit_stones(color, x, y, &mut visited, board) {
            for &(a, b) in visited.iter() {
                board[a][b] = 0;
            }
        }
    }
}

// takes in a visited HashSet refrence that we fill
// with board positions. returns bool
fn visit_stones(color : i32, i: usize, j: usize, visited: &mut HashSet<(usize, usize)> , board: &[[i32; 9]; 9]) -> bool {
    // if we are in an out of bounds position then we
    // should exit out

    if i >= 9 || j >= 9 {
        return true;
    }

    // if already visited then return;
    if visited.contains(&(i, j)) {
        return true;
    }

    // if the board position is not the same color then return
    if board[i][j] != color {
        return true;
    }

    // if the board position is empty then return
    if board[i][j] == 0 {
        return false;
    }

    // add board position to visited
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


fn play_moves() {
    // takes in an array of ints corresponding to each move and then plays it out on a board
    // board is an N x N nested array that stores the states
    // need an hashset to track positions to make sure we don't duplicate the board (not allowed)
}

fn main() {
    println!("Getting filenames");

    let paths = fs::read_dir("data").unwrap();

    let mut num_files = 0;

    let start = Instant::now();

    for path in paths {

        let path_str = path.unwrap().path().display().to_string();

        println!("Name: {}", path_str);
        num_files += 1;
        process_file(path_str);
        //break;
    }

    println!("Files Loaded: {}", num_files);

    print();

    let mut duration = start.elapsed(); // Calculate the elapsed time

    println!("Time taken: {:?}", duration); // Print the duration
    // If you want a specific unit, like milliseconds, you can do:
    println!("Time taken (milliseconds): {:?}", duration.as_millis());
    println!("Time taken per game (ms): {:?}", duration.as_millis() / num_files);


}


fn print() {
    println!("hello, world!");
}
