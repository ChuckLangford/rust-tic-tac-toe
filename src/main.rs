use std::io::{self, Write};
use std::process::Command;

fn clear_the_screen() {
    /*
     * The Command struct configures and spawns new processes. In this case, we're using the
     * clear command to clear the screen.
     * new() constructs the new Command struct
     * output() executes the command as a child process and collects it's output
     * unwrap_or_else() returns the Ok value or computes it from a closure
     */
    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    /*
     * from_utf8_lossy() converts a slice of bytes to a string safe for printing
     * output.stdout the data that the Command wrote to stdout
     */
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn ask_for_input() {
    print!("Enter your selection: ");
    io::stdout().flush().unwrap();
}

fn get_board_character(pos: u8, refer: u8) -> String {
    if pos == 1 {
        String::from("X")
    } else if pos == 2 {
        String::from("O")
    } else {
        refer.to_string()
    }
}

#[test]
fn test_get_board_character() {
    assert_eq!(get_board_character(1 as u8, 1 as u8), "X");
    assert_eq!(get_board_character(2 as u8, 1 as u8), "O");
    assert_eq!(get_board_character(0 as u8, 1 as u8), 1.to_string());
}

fn print_the_board(player: u8, the_board: &[u8; 9]) {
    clear_the_screen();
    println!("Player {}, please select a tile on the board. Only values 1 through 9 are valid.", player);
    println!("");
    let mut tile_ref = 0 as u8;
    for tile in the_board.iter() {
        tile_ref = tile_ref + 1;
        if tile_ref % 3 != 0 {
            print!(" {} |", get_board_character(*tile, tile_ref));
        } else {
            print!(" {} ", get_board_character(*tile, tile_ref));
            io::stdout().flush().unwrap();
            println!("");
            println!("---|---|---");
        }
    }

    ask_for_input();
}

fn take_turn(player: u8, the_board: &mut[u8; 9]) {
    /*
     * declare selection so that it can mutate
     */
    let mut selection = String::new();

    /*
     * After asking the user for their move, read in the value.
     * stdin() constructs a new handle to the stdin (a Stdin struct)
     * read_line() reads from stdin and appends it to the given mutable argument
     * expect() either returns the Ok from the Result or panics with the give message
     */
    io::stdin().read_line(&mut selection).expect("Failed to read line");

    /*
     * trim() trims leading/trailing whitespace
     * parse() attempts to parse the string into another type; returns a Result
     */
    let selection: u8 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_e) =>  {
            println!("Your selection is not a valid number from the board.");
            ask_for_input();
            return take_turn(player, the_board);
        }
    };

    // check that the user has selected a valid number
    if selection > 9 || selection < 1 {
        println!("{} is not a valid tile on the board.", selection);
        ask_for_input();
        return take_turn(player, the_board);
    }

    // check that the user has not selected a tile that has already been chosen
    if the_board[usize::from(selection - 1)] != 0 {
        println!("Tile {} has already been played. Choose another tile.", selection);
        ask_for_input();
        return take_turn(player, the_board);
    }

    // update the board
    the_board[usize::from(selection - 1)] = player;
}

fn player_has_won(player_number: u8, the_board: &[u8; 9]) -> bool {
    // check each row
    if the_board[0] == player_number
        && the_board[1] == player_number
        && the_board[2] == player_number {
        return true;
    }

    if the_board[3] == player_number
        && the_board[4] == player_number
        && the_board[5] == player_number {
        return true;
    }

    if the_board[6] == player_number
        && the_board[7] == player_number
        && the_board[8] == player_number {
        return true;
    }

    // check each column
    if the_board[0] == player_number
        && the_board[3] == player_number
        && the_board[6] == player_number {
        return true;
    }

    if the_board[1] == player_number
        && the_board[4] == player_number
        && the_board[7] == player_number {
        return true;
    }

    if the_board[2] == player_number
        && the_board[5] == player_number
        && the_board[8] == player_number {
        return true;
    }

    // check both diagnals
    if the_board[0] == player_number
        && the_board[4] == player_number
        && the_board[8] == player_number {
        return true;
    }

    if the_board[2] == player_number
        && the_board[4] == player_number
        && the_board[6] == player_number {
        return true;
    }
    false
}

#[test]
fn test_player_has_won() {
    let mut board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(player_has_won(1, &board), false);
    board = [1, 1, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(player_has_won(1, &board), true);
    board = [1, 1, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(player_has_won(1, &board), true);
    board = [0, 0, 0, 1, 1, 1, 0, 0, 0];
    assert_eq!(player_has_won(1, &board), true);
    board = [0, 0, 0, 0, 0, 0, 1, 1, 1];
    assert_eq!(player_has_won(1, &board), true);
    board = [1, 0, 0, 1, 0, 0, 1, 0, 0];
    assert_eq!(player_has_won(1, &board), true);
    board = [0, 1, 0, 0, 1, 0, 0, 1, 0];
    assert_eq!(player_has_won(1, &board), true);
    board = [0, 0, 1, 0, 0, 1, 0, 0, 1];
    assert_eq!(player_has_won(1, &board), true);
    board = [1, 0, 0, 0, 1, 0, 0, 0, 1];
    assert_eq!(player_has_won(1, &board), true);
    board = [0, 0, 1, 0, 1, 0, 1, 0, 0];
    assert_eq!(player_has_won(1, &board), true);
}

fn game_is_draw(the_board: &[u8; 9]) -> bool {
    for tile in the_board.iter() {
        if *tile == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_game_is_draw() {
    let not_draw: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(game_is_draw(&not_draw), false);
    let draw: [u8; 9] = [1, 2, 1, 2, 2, 1, 1, 1, 2];
    assert_eq!(game_is_draw(&draw), true);
}

fn toggle_player(player: u8) -> u8 {
    if player == 1 {
        2
    } else {
        1
    }
}

#[test]
fn test_toggle_player() {
    assert_eq!(toggle_player(1 as u8), 2);
    assert_eq!(toggle_player(2 as u8), 1);
}

fn main() {
    /*
     * For keeping track of game progress, we're going to use an array. Array's must use the same
     * type and have a predefined length. So in our case, we're going to keep track of which
     * player has selected a tile on the board. We're number the tiles 1 - 9 top to botton, left
     * to right.
     */
    let mut the_board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut player_number: u8 = 1;

    loop {
        print_the_board(player_number, &the_board);
        take_turn(player_number, &mut the_board);
        print_the_board(player_number, &the_board);

        if player_has_won(player_number, &the_board) {
            println!("\nPlayer {} has won!", player_number);
            break;
        } else if game_is_draw(&the_board) {
            println!("\nDraw! Game over.");
            break;
        } else {
            player_number = toggle_player(player_number);
        }
    }
}
