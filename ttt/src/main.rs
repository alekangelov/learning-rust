use rand::Rng;

struct Board {
    board: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            board: [[' '; 3]; 3],
        }
    }

    fn print(&self) {
        for (j, &row) in self.board.iter().enumerate() {
            let mut line = String::new();
            for (i, &col) in row.iter().enumerate() {
                line.push(col);
                if i < row.len() - 1 {
                    line.push_str(" | ");
                }
            }
            println!("{}", line);
            if j < self.board.len() - 1 {
                println!("---------");
            }
        }
    }

    fn is_free(&self, row: usize, col: usize) -> bool {
        self.board[row][col] == ' '
    }

    fn is_full(&self) -> bool {
        for row in self.board.iter() {
            for &col in row.iter() {
                if col == ' ' {
                    return false;
                }
            }
        }
        true
    }

    fn is_won(&self) -> bool {
        let mut won = false;
        for row in self.board.iter() {
            if row[0] != ' ' && row[0] == row[1] && row[1] == row[2] {
                won = true;
            }
        }
        for i in 0..self.board.len() {
            if self.board[0][i] != ' '
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
            {
                won = true;
            }
        }
        if self.board[0][0] != ' '
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            won = true;
        }
        if self.board[0][2] != ' '
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            won = true;
        }
        won
    }
}

struct PlayerInput {
    row: usize,
    col: usize,
}

impl PlayerInput {
    fn new(row: usize, col: usize) -> PlayerInput {
        PlayerInput { row, col }
    }

    fn from_string(input: String) -> PlayerInput {
        let mut iter = input.split_whitespace();
        let row = match iter.next() {
            Some(s) => s.parse::<usize>().unwrap(),
            None => panic!("Invalid input"),
        };
        let col = match iter.next() {
            Some(s) => s.parse::<usize>().unwrap(),
            None => panic!("Invalid input"),
        };
        PlayerInput::new(row, col)
    }
}

fn get_player_input(board: &Board) -> PlayerInput {
    println!("Enter row and column (1 2): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let p_input = PlayerInput::from_string(input);
    if board.is_free(p_input.row, p_input.col) {
        return p_input;
    }
    println!("Invalid input \n");
    get_player_input(board)
}

fn get_computer_input(board: &Board) -> PlayerInput {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..3);
    let col = rng.gen_range(0..3);
    if board.is_free(row, col) {
        return PlayerInput::new(row, col);
    }
    get_computer_input(board)
}

fn main() {
    let mut board = Board::new();
    board.print();
    while !board.is_full() || !board.is_won() {
        let player_input = get_player_input(&board);
        board.board[player_input.row][player_input.col] = 'X';
        board.print();
        if board.is_won() {
            println!("You won!");
            break;
        }
        let computer_input = get_computer_input(&board);
        board.board[computer_input.row][computer_input.col] = 'O';
        board.print();
        if board.is_won() {
            println!("Computer won!");
            break;
        }
    }
}
