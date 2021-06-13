pub struct Replay {
    movement_stack: Vec<u8>
}

impl Replay {
    pub fn pop(&mut self) -> u8 {
        self.movement_stack.pop().expect("testing")
    }

    pub fn push(&mut self, movement: u8) {
        self.movement_stack.push(movement);
    }

    pub fn is_empty(&self) -> bool {
        self.movement_stack.len() == 0
    }

    pub fn clear(&mut self) {
        self.movement_stack.clear();
    }
}

pub fn new() -> Replay {
    Replay {
        movement_stack: Vec::new()
    }
}