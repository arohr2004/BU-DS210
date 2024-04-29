// Function to calculate the liveness of a cell
fn calculate_liveness(board: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let mut live_neighbors = 0;
    let n = board.len();
    let m = board[0].len();

    // Iterate through the neighbors
    for i in 0..3 {
        for j in 0..3 {
            if i != 1 || j != 1 {
                let nx = (x + i + n - 1) % n;
                let ny = (y + j + m - 1) % m;
                live_neighbors += board[nx][ny];
            }
        }
    }

    // Appling the rules
    if live_neighbors == 2 {
        board[x][y]
    } else if live_neighbors == 3 {
        1
    } else {
        0
    }
}

// Function to evolve the board
fn evolve(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = board.len();
    let m = board[0].len();
    let mut new_board = vec![vec![0; m]; n];

    // Iterate through each cell to calculate its liveness
    for i in 0..n {
        for j in 0..m {
            new_board[i][j] = calculate_liveness(&board, i, j);
        }
    }

    new_board
}

// print the board
fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for cell in row {
            print!("{}", if *cell == 1 { "X" } else { "." });
        }
        println!();
    }
}

fn main() {
    // Initialize the board
    let mut board = vec![vec![0; 16]; 16];
    let initial_state = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    for (x, y) in initial_state {
        board[x][y] = 1;
    }

    // Creating 10 iterations 
    for generation in 0..10 {
        println!("Generation {}:", generation + 1);
        print_board(&board);
        println!();

        // Evolve the board
        board = evolve(&board);
    }
}
//Varrious tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_liveness() {
        // cell with 2 live neighbors survives
        let board1 = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
        ];
        assert_eq!(calculate_liveness(&board1, 1, 1), 1);

        // dead cell with 3 live neighbors becomes alive
        let board2 = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![1, 1, 0],
        ];
        assert_eq!(calculate_liveness(&board2, 1, 1), 1);

        // live cell with 4 live neighbors dies
        let board3 = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 0],
        ];
        assert_eq!(calculate_liveness(&board3, 1, 1), 0);

        // dead cell with 2 live neighbors remains dead
        let board4 = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(calculate_liveness(&board4, 2, 2), 0);
    }
}

