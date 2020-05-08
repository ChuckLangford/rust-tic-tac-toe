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

fn get_board_character(pos: u8, refer: u8) -> String {
    if pos == 1 {
        String::from("X")
    } else if pos == 2 {
        String::from("O")
    } else {
        refer.to_string()
    }
}

fn print_the_board(player: u8, the_board: &[u8; 9]) {
    clear_the_screen();
    println!("Player {}, please select a position on the board. Only values 1 through 9 are valid.", player);
    println!("");
    let mut position_ref = 0 as u8;
    for position in the_board.iter() {
        position_ref = position_ref + 1;
        if position_ref % 3 != 0 {
            print!(" {} |", get_board_character(*position, position_ref));
        } else {
            print!(" {} ", get_board_character(*position, position_ref));
            io::stdout().flush().unwrap();
            println!("");
            println!("---|---|---");
        }
    }

    print!("Enter your selection: ");
    io::stdout().flush().unwrap();
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
        Err(e) =>  panic!("Yikes {}", e),
    };

    // check that the selection is acceptable
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

fn game_is_draw(the_board: &[u8; 9]) -> bool {
    for position in the_board.iter() {
        if *position == 0 {
            return false;
        }
    }
    true
}

fn toggle_player(player: u8) -> u8 {
    if player == 1 {
        2
    } else {
        1
    }
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
