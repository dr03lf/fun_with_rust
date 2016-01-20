use std::cmp::Ordering;

struct History {
    h: Vec<(usize, usize)>
}

struct Board {
        board: [usize; 8],
}

impl Board{
    fn new() -> Board {
        Board{
            board: [8,8,8,8,8,8,8,8]
        }
    }

    fn set(&mut self, x: usize, y: usize){
        self.board[x] = y;
    }

    fn get(&self, x: usize) -> usize {
        self.board[x]
    }

    fn pos_set(&self, x: usize, y: usize) -> bool {
        self.get(x) == y
    }

    fn done(&self) -> bool {
        for i in 0..self.board.len() {
            if self.board[i] == 8 {
                return false;
            }
        }
        return true;
    }

    fn print(&self){
        println!("\n---------------------------------");
        for i in 0..8 {
            print!("|");
            for j in 0..8 {
                match self.get(j).cmp(&i) {
                    Ordering::Equal     => print!(" {} |", "X"),
                    Ordering::Greater   => print!(" {} |", " "),
                    Ordering::Less      => print!(" {} |", " "),
                }
            }
            println!("\n---------------------------------");
        }
    }

    fn collision(&self, x: usize, y: usize) -> bool {
        // col
        if self.get(x) != 8 {
            //println!("column true");
            return true;
        }

        for i in 0..8 {
            if self.pos_set(i, y) {
                //println!("row true");
                return true;
            }
        }

        let mut x_pos;
        let mut y_pos;
        if x < (7 - y) {
            x_pos = 0;
            y_pos = y + x;
        } else {
            x_pos = x - (7 - y);
            y_pos = 7;
        }

        loop {
            if self.pos_set(x_pos, y_pos){
                //println!("diag 1 found");
                return true;
            }
            if y_pos == 0 {
                break;
            }
            if x_pos == 7 {
                break;
            }
            x_pos = x_pos + 1;
            y_pos = y_pos - 1;
        }

        let mut x_pos;
        let mut y_pos;
        if y < x {
            x_pos = x - y;
            y_pos = 0;
        } else {
            x_pos = 0;
            y_pos = y - x;
        }

        loop {
            if self.pos_set(x_pos, y_pos){
                //println!("diag 1 found");
                return true;
            }
            if y_pos == 7 {
                break;
            }
            if x_pos == 7 {
                break;
            }
            x_pos = x_pos + 1;
            y_pos = y_pos + 1;
        }

        return false;
    }
}


fn find_my_queens(board: &mut Board, history: &mut History, row: usize, col: usize){
    if board.done() {
        board.print();
        return;
    }

    let r = row % 8;
    let mut c = 0;
    let mut found = false;
    for i in col..8 {
        if !board.collision(i, r){
            c = i;
            found = true;
            break;
        }
    }

    if found {
        board.set(c, r);
        history.h.push((c, r));
        find_my_queens(board, history, r + 1, 0);
    } else {
        match history.h.pop() {
            Some((x,_)) => {
                board.set(x, 8);
                if r == 0 {
                    find_my_queens(board, history, r, x + 1)
                } else {
                    find_my_queens(board, history, r - 1, x + 1)
                }
            }
            None => println!("Nope")
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut history = History { h: vec![] };
    find_my_queens(&mut board, &mut history, 5, 5);
}
