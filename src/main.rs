use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
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


// define a const BLACK = -1, WHITE = 1
// define a const EMPTY = 0
// we can do this using: enum Color { BLACK = -1, WHITE = 1, EMPTY = 0 }
enum Color {
    BLACK = -1,
    WHITE = 1,
    EMPTY = 0
}



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
                    println!("{} moved to location: {} {}", chars[idx], p1, p2);

                    // if black then set to -1, if white set to 1
                    if chars[idx] == 'B' && p1 < 9 && p2 < 9{
                        grid[p1][p2] = 1;
                    }
                    
                    if chars[idx] == 'W' && p1 < 9 && p2 < 9 {
                        grid[p1][p2] = -1;
                    }
                },
                _ => {

                }
            }

            let board_copy = &mut grid;
            remove_dead_stones(board_copy);
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
}


fn remove_dead_stones(board: &mut [[i32; 9]; 9]) {
    println!("we have the board");

    // init a set, to store visited locations

    for i in 0..9 {
        for j in 0..9{
            // initialize a hashset
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            visit_stones(i, j, visited, board);

        }
    }

    // need a queue 
}

// takes in a visited HashSet refrence that we fill
// with board positions
fn visit_stones(i: usize, j: usize, visited: HashSet<(usize, usize)>, board: &mut [[i32; 9]; 9]) {
    // if we are in an out of bounds position then we
    // should exit out

    if i < 0 || i >= 9 || j < 0 || j >= 9 {
        return False;
    }

    // if already visited then return;
    if visited.contains(&(i, j)) {
        return False;
    }

    // if the board position is empty then return
    if board[i][j] == 0 {
        return True;
    }

    // add board position to visited
    visited.insert((i, j));

    let left = visit_stones(i-1, j, visited, board);
    let right = visit_stones(i+1, j, visited, board);
    let up = visit_stones(i, j-1, visited, board);
    let down = visit_stones(i, j+1, visited, board);
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

    for path in paths {

        let path_str = path.unwrap().path().display().to_string();

        //println!("Name: {}", path_str);
        num_files += 1;

        process_file(path_str);
    }

    println!("Files Loaded: {}", num_files);

    print();

}


fn print() {
    println!("hello, world!");
}
