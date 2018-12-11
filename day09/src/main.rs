const NUM_PLAYERS: usize = 405;
const TOP_MARBLE: u32 = 71700 * 100;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RingNode(Rc<RingNodeContents>);

struct RingNodeContents {
    value: u32,
    ccw: RefCell<Option<RingNode>>,
    cw: RefCell<Option<RingNode>>,
}

impl Clone for RingNode {
    fn clone(&self) -> RingNode {
        RingNode(Rc::clone(&self.0))
    }
}

impl RingNode {
    pub fn new(value: u32) -> RingNode {
        let node = RingNode(Rc::new(RingNodeContents {
            value,
            ccw: RefCell::new(None),
            cw: RefCell::new(None),
        }));
        *node.0.ccw.borrow_mut() = Some(RingNode(Rc::clone(&node.0)));
        *node.0.cw.borrow_mut() = Some(RingNode(Rc::clone(&node.0)));
        node
    }

    pub fn value(&self) -> u32 {
        self.0.value
    }

    pub fn cw(&self) -> RingNode {
        RingNode(Rc::clone(&self.0.cw.borrow().as_ref().unwrap().0))
    }

    pub fn ccw(&self) -> RingNode {
        RingNode(Rc::clone(&self.0.ccw.borrow().as_ref().unwrap().0))
    }

    pub fn insert_cw(&self, new_value: u32) -> RingNode {
        let new_node = RingNode(Rc::new(RingNodeContents {
            value: new_value,
            ccw: RefCell::new(Some(self.clone())),
            cw: RefCell::new(Some(self.cw())),
        }));
        let cw = self.cw();
        *self.0.cw.borrow_mut() = Some(RingNode(Rc::clone(&new_node.0)));
        *cw.0.ccw.borrow_mut() = Some(RingNode(Rc::clone(&new_node.0)));
        new_node
    }

    pub fn remove(self) {
        let ccw = self.ccw();
        let cw = self.cw();
        *ccw.0.cw.borrow_mut() = Some(self.cw());
        *cw.0.ccw.borrow_mut() = Some(self.ccw());
    }

    pub fn kill_ring(self) {
        let mut this = self;
        while let Some(next) = this.0.cw.borrow().as_ref() {
            *this.0.cw.borrow_mut() = None;
            *this.0.ccw.borrow_mut() = None;
            this = next.clone();
        }
    }
}

fn play_marbles() -> u32 {
    let mut scores = [0; NUM_PLAYERS];
    let mut current_marble = RingNode::new(0);
    let mut current_player = 0;

    for marble in 1..=TOP_MARBLE {
        if marble % 23 == 0 {
            scores[current_player] += marble;
            for _ in 1..=7 {
                current_marble = current_marble.ccw();
            }
            scores[current_player] += current_marble.value();
            let new_current_marble = current_marble.cw();
            current_marble.remove();
            current_marble = new_current_marble;
        } else {
            current_marble = current_marble.cw().insert_cw(marble);
        }
        current_player = (current_player + 1) % NUM_PLAYERS;
    }

    current_marble.kill_ring();

    *scores.iter().max().unwrap()
}

fn main() {
    println!("The winning Elf's score is {}.", play_marbles());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_tests() {
        let mut countdown = RingNode::new(1).insert_cw(2).insert_cw(3);
        for i in (1..=3).rev() {
            assert_eq!(i, countdown.value());
            countdown = countdown.ccw();
        }

        countdown.cw().remove();

        for i in 1..=10 {
            assert_eq!(i % 2, countdown.value() % 2);
            countdown = countdown.ccw();
        }
    }
}
