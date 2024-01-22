use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


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

    // try and load the file contents

    // need to parse the file to get the game parameters

    // need to parse the file to get the turn and moves of the users
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

}
