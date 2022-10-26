use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::{clone, rc};
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    pub beach: Vec<Beach>,
    pub reefs: Vec<Rc<RefCell<Reef>>>,
    // TODO: Fill in fields here.
}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean { 
            beach: Vec::new(), 
            reefs: Vec::new(),
        }
        // unimplemented!();
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beach.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beach.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        // unimplemented!();
        let reef = Rc::new(RefCell::new(Reef::new()));
        for i in 0..n_algae {
            let mut b_reef = reef.borrow_mut();
            let prey = Algae::new();
            b_reef.add_prey(Box::new(prey));
        }

        for i in 0..n_minnows {
            let mut b_reef = reef.borrow_mut();
            let prey = Minnow::new(25);
            b_reef.add_prey(Box::new(prey));
        }

        for i in 0..n_shrimp {
            let mut b_reef = reef.borrow_mut();
            let prey = Shrimp::new(1);
            b_reef.add_prey(Box::new(prey));
        }
        
        for i in 0..n_clams {
            let mut b_reef = reef.borrow_mut();
            let prey = Clam::new();
            b_reef.add_prey(Box::new(prey));
        }

        let pt1 = Rc::clone(&reef);
        let pt2 = Rc::clone(&reef);
        self.reefs.push(pt1);
        return pt2;
    }
}
