use std::env;

struct History {
    h: Vec<(usize, usize)>
}

struct Board {
        board: Vec<usize>,
        size: usize
}

impl Board{
    fn new(size: usize) -> Board {
        Board{
            board: vec![size; size],
            size: size,
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
            if self.board[i] == self.size {
                return false;
            }
        }
        return true;
    }

    fn print(&self){
        self.print_line();
        for i in 0..self.size {
            print!("|");
            for j in 0..self.size {
                if self.get(j) == i{
                    print!(" {} |", "◉");
                } else {
                    print!(" {} |", " ");
                }
            }
            self.print_line();
        }
    }

    fn print_line(&self) {
        print!("\n+");
        for _i in 0..self.size {
            print!("---+");
        }
        print!("\n");
    }

    fn collision(&self, x: usize, y: usize) -> bool {
        // col
        if self.get(x) != self.size {
            return true;
        }

        for i in 0..self.size {
            if self.pos_set(i, y) {
                return true;
            }
        }

        let mut x_pos;
        let mut y_pos;
        if x < (self.size - 1 - y) {
            x_pos = 0;
            y_pos = y + x;
        } else {
            x_pos = x - (self.size - 1 - y);
            y_pos = self.size - 1;
        }

        loop {
            if self.pos_set(x_pos, y_pos){
                //println!("diag 1 found");
                return true;
            }
            if y_pos == 0 || x_pos == self.size - 1 {
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
            if y_pos == self.size - 1 || x_pos == self.size - 1 {
                break;
            }
            x_pos = x_pos + 1;
            y_pos = y_pos + 1;
        }

        return false;
    }
}


fn find_my_queens(board: &mut Board, history: &mut History, row: usize, col: usize, tries: usize){
    //println!("{:?}", history.h);

    if board.done() || tries == 0 {
        if board.done() {
            board.print();
        }
        return;
    }

    let r = row % board.size;
    let mut c = 0;
    let mut found = false;
    for i in col..board.size {
        if !board.collision(i, r){
            c = i;
            found = true;
            break;
        }
    }

    if found {
        board.set(c, r);
        history.h.push((c, r));
        find_my_queens(board, history, r + 1, 0, tries - 1);

    } else {
        match history.h.pop() {
            Some((x,_)) => {
                let size = board.size;
                board.set(x, size);
                if r == 0 {
                    find_my_queens(board, history, size - 1, (x + 1), tries - 1)
                } else {
                    find_my_queens(board, history, r - 1, (x + 1), tries - 1)
                }
            }
            None => {
                println!("nope");
            }
        }
    }
}

fn main() {
    let size = 8;
    let arg = env::args()
                .nth(1)
                .map(|x| x.trim().parse());

    let size = match arg {
        Some(Ok(n)) => n,
        _ => size
    };

    for j in 0..size {
        let mut board = Board::new(size);
        let mut history = History { h: vec![] };

        let mut nextpos = (0, j);

        loop {
            let (x,y) = nextpos;
            find_my_queens(&mut board, &mut history, x, y, 15300);

            if board.done() {
                println!("Positions: {:?}", history.h);
                break;
            } else {
                let h = history.h.last();
                match h {
                    Some(xy) => {
                        let (x, y) = *xy;
                        nextpos = (x, y + 1);
                    },
                    None => {
                        println!("No last history element :(");
                        nextpos = (j, 0);
                    }
                }
            }
        }
    }
}
