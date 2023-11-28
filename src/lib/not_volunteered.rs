pub struct Knight {
    current_x: u8,
    current_y: u8,
    target_x: u8,
    target_y: u8,
}

impl Knight {
    pub fn new(src: u8, dest: u8) -> Knight {
        Knight {
            current_x: src % 8,
            current_y: src / 8,
            target_x: dest % 8,
            target_y: dest / 8,
        }
    }

    // clock-wise movement definition
    fn up_right(&mut self) {
        self.current_y -= 2;
        self.current_x += 1;
    }

    fn right_up(&mut self) {
        self.current_x += 2;
        self.current_y -= 1;
    }

    // TODO: unfinished
}
