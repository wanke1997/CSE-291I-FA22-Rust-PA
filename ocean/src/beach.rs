use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    pub crabs: Vec<Crab>,
}

impl Beach {
    pub fn new() -> Beach {
        Beach { crabs: Vec::new() }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.crabs.len()==0 {
            None
        } else {
            let mut speed:u32 = self.crabs[0].speed();
            let mut fastest_crab: &Crab = &self.crabs[0];
            for crab in &self.crabs {
                if crab.speed()>speed {
                    fastest_crab = &crab;
                    speed = crab.speed()
                }
            }
            Some(fastest_crab)
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut res_vec:Vec<&Crab> = Vec::new();
        for crab in &self.crabs {
            if crab.name.eq(name) {
                res_vec.push(crab);
            }
        }
        res_vec
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i >= self.crabs.len() || j >= self.crabs.len() {
            panic!("breeding crabs out of bound!");
        }
        let baby_crab = Crab {
            name: name,
            speed: 1,
            color: Color::cross(&self.crabs[i].color(), &self.crabs[j].color()),
            diet: Diet::random_diet()
        };
        self.crabs.push(baby_crab);
    }
}
