pub struct Game {
    matrix: Vec<char>,
    player: char
}

impl Game {

    pub fn player(&self) -> char {
        self.player
    }


    pub fn new() -> Self{
        let value: Vec<char> = [' '].repeat(9);
        Self{
            matrix: value,
            player: 'X'
        }
    }

    fn printline(value: Vec<char>) {
        println!("\t| {} | {} | {} |", value[0], value[1], value[2]);
    }

    pub fn print(&self) {
        println!("\t{}-----------{}", "o", "o");
        for i in 0..3 {
            let row = &self.matrix[((i) * 3)..((i + 1) * 3)];
            Game::printline(row.to_vec()); 
            if i < 2 {
                println!("\t ----------- ");
            }
        }
        println!("\t{}-----------{}\n", "o", "o"); 
    }

    pub fn is_valid_index(&self, index: usize) -> bool {
       if index > 8 || self.matrix[index] == ' ' {
           true 
       } else {
           false 
       }
    }

    pub fn update_matrix(& mut self, index: usize) {
        self.matrix[index] = self.player;
    }

    pub fn update_index(&mut self) {
        self.player = if self.player == 'X' {'O'} else {'X'};
    }


    fn row_win(&self) -> bool {

        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[1] && 
            self.matrix[0] == self.matrix[2]) {
            true 
        } else if self.matrix[3] != ' ' && (self.matrix[3] == self.matrix[4] && 
            self.matrix[3] == self.matrix[5])  {
            true
        } else if  self.matrix[3] != ' ' && (self.matrix[3] == self.matrix[4] && 
            self.matrix[3] == self.matrix[5]) {
            true 
        } else {
            false
        }

    }

    fn column_win(&self) -> bool {

        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[3] &&
                                     self.matrix[0] == self.matrix[6]) {
            true 
        } else if self.matrix[1] != ' ' && (self.matrix[1] == self.matrix[4] &&
                                     self.matrix[1] == self.matrix[7]) {
            true 
        } else if self.matrix[2] != ' ' && (self.matrix[2] == self.matrix[5] &&
                                     self.matrix[2] == self.matrix[8]) {
            true 
        } else {
            false 
        }

    }

    fn diagonal_win(&self) -> bool {
        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[4] &&
                                     self.matrix[0] == self.matrix[8]) {
            true 
        } else if self.matrix[2] != ' ' && (self.matrix[2] == self.matrix[4] &&
                                            self.matrix[2] == self.matrix[6]) {
            true 
        } else {
            false 
        }

    }


    fn win(&self) -> bool {
        if self.row_win() || self.column_win() || self.diagonal_win() {
            true
        } else {
            false 
        }
    }

    fn draw(&self) -> bool {
       for val in &self.matrix {
           if val == &' ' {
               return false
           }

       }
       true
    }


    // Check if current value of matrix results in win (0), draw (1) or
    // incomplete (2)
    //
    pub fn status(&self) -> u8 {
        if self.win() {
            0
        } else if self.draw() {
            1
        } else {
            2
        }
    }



}


