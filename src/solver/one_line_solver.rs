use crate::display::{Displayable, display_picross_line_unit::display_line};
use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Verbose {
    Quiet,
    Partial,
    Full,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LineSolveMethod {
    BrutForce,
    SpaceDistribution,
}

pub fn validate_line_unit(line : &PicrossLineUnit) -> bool {
    let nb_clues = line.clues.len();
    let mut current_clue : usize = 0;
    let mut current_block_size : usize = 0;
    for i in 0..line.values.len() {
        match line.values[i] {
            CaseState::ON => { //Start or continuing a block
                //Check if a clue remains
                if current_clue == nb_clues {
                    return false;
                }
                
                //Then increase block size
                current_block_size += 1;

                //Check if block is already too big
                if current_block_size > line.clues[current_clue] {
                    return false;
                }
            },
            _ => {
                if current_block_size > 0 {

                    //Check if terminated block has good size
                    if current_block_size != line.clues[current_clue] {
                        return false;
                    }

                    //Go to next clue and reset block size
                    current_clue += 1;
                    current_block_size = 0;
                }
            }
        }
    }
    //Finish last block
    if current_block_size > 0 {
        if current_block_size != line.clues[current_clue] {
            return false;
        }
        current_clue += 1;
    }

    //Check if we have validated all clues
    if (current_clue != nb_clues) && (line.clues[current_clue] != 0) {
        return false;
    }

    true
}

pub struct LineUnitSolver<'a> {
    pub picross: &'a mut PicrossLineUnit,
    pub editable : Vec<usize>,
    pub fixed : Vec<usize>,
    pub count : usize,
    pub verbose : Verbose,
    pub solve_method : LineSolveMethod,
    pub spaces : Vec<usize>,
}

impl<'a> LineUnitSolver<'a>{
    pub fn new(picross : &'a mut PicrossLineUnit) -> Self {
        let mut editable : Vec<usize> = Vec::new();
        let mut fixed : Vec<usize> = Vec::new(); 
        for i in 0..picross.values.len() {
            if picross.values[i] == CaseState::UNKNOWN {
                editable.push(i);
            } else {
                fixed.push(i);
            }
        }

        let space_slot_count = picross.clues.len() + 1;
        let spaces : Vec<usize> = vec![0; space_slot_count];

        Self {
            picross,
            editable,
            fixed,
            count : 0,
            verbose : Verbose::Quiet,
            solve_method : LineSolveMethod::BrutForce,
            spaces,
        }
    }

    pub fn init_solve(&mut self) {
        if self.solve_method == LineSolveMethod::SpaceDistribution {
            self.spaces.fill(0);
            let last_index = self.spaces.len() - 1;
            self.spaces[last_index] = self.picross.free_space_count();
        }
        self.count = 0;

        for i in 0..self.editable.len() {
            self.picross.values[self.editable[i]] = CaseState::OFF;
        }
        
    }

    pub fn reset_solve(&mut self) {
        if self.solve_method == LineSolveMethod::SpaceDistribution {
            self.spaces.fill(0);
            let last_index = self.spaces.len() - 1;
            self.spaces[last_index] = self.picross.free_space_count();
        }
        self.count = 0;

        for i in 0..self.editable.len() {
            self.picross.values[self.editable[i]] = CaseState::UNKNOWN;
        }
    }

    pub fn next_try(&mut self) {
        for i in (0..self.editable.len()).rev() {
            match self.picross.values[self.editable[i]] {
                CaseState::OFF => {
                    self.picross.values[self.editable[i]] = CaseState::ON;
                    self.count += 1;
                    return
                }
                _ => self.picross.values[self.editable[i]] = CaseState::OFF,
            }
        }
        panic!("One line solver has overflown. Program stops to prevent infinite loop.");
    }

    pub fn next_space_try(&mut self) {

        if self.count == 0 {
            self.count += 1;
            return;
        }

        let last_idx = self.spaces.len() - 1;
        for i in (0..self.spaces.len()).rev() {
            let value = self.spaces[i];
            match value {
                0 => {}
                _ => { 
                    self.spaces[i-1] += 1;
                    if i != last_idx {
                        self.spaces[last_idx] = value - 1;
                        self.spaces[i] = 0;
                    } else {
                        self.spaces[last_idx] -= 1;
                    }
                    self.count += 1;
                    return
                }
            }
        }
    }

    pub fn construct_line_from_spaces(& self) -> Vec<CaseState> {
        let mut line = vec![CaseState::OFF; self.picross.values.len()];
        let mut cursor = self.spaces[0];
        for i in 0..self.picross.clues.len() {
            for _ in 0..self.picross.clues[i] {
                line[cursor] = CaseState::ON;
                cursor += 1;
            }
            cursor += self.spaces[i+1] + 1;
        }
        return line;
    }

    pub fn next_space(&mut self) -> bool {
        while self.spaces[0] != self.picross.free_space_count() {
            
            self.next_space_try();

            let try_line = self.construct_line_from_spaces();

            if self.verbose == Verbose::Full { print!("try{:?}", self.spaces); display_line(&try_line); println!(""); }

            let mut is_valid = true;
            for i in 0..self.fixed.len() {
                let idx = self.fixed[i];
                if try_line[idx] != self.picross.values[idx] {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                for i in 0..self.editable.len() {
                    let idx = self.editable[i];
                    self.picross.values[idx] = try_line[idx];
                }
                return true;
            }
        }
        return false;
    }

    pub fn next(&mut self) -> bool {
        if self.solve_method == LineSolveMethod::SpaceDistribution {
            return self.next_space();
        } else {
            while self.count != self.max_count() - 1 {
                self.next_try();
    
                if self.verbose == Verbose::Full { print!("try   "); self.picross.display(); }
    
                if validate_line_unit(self.picross) {
                    return true
                }
            }
            return false
        }
    }

    pub fn max_count(&mut self) -> usize {
        return 2_usize.pow(self.editable.len() as u32);
    }
}

impl<'a> Solvable for LineUnitSolver<'a> {
    fn solve(&mut self) -> Solve {
        let mut solve_res = Solve::None;

        if self.solve_method == LineSolveMethod::SpaceDistribution && self.picross.free_space_count() == 0 {
            self.picross.values = self.construct_line_from_spaces();
            return Solve::Full;
        }

        if self.verbose != Verbose::Quiet { print!("init  "); self.picross.display(); println!(""); }

        self.init_solve();
        if validate_line_unit(self.picross) {
            solve_res = Solve::Full;    
        }
        let mut common = self.picross.values.to_vec();

        while self.next() {

            if solve_res == Solve::None {
                common = self.picross.values.to_vec();
                solve_res = Solve::Full;
            }

            for i in 0..self.editable.len() {
                let idx = self.editable[i];
                if self.picross.values[idx] != common[idx] {
                    solve_res = Solve::Partial;
                    common[idx] = CaseState::UNKNOWN;
                }
            }

            if self.verbose != Verbose::Quiet { print!("valid "); self.picross.display(); }

        }

        self.picross.values = common.to_vec();
        if self.verbose != Verbose::Quiet {  println!(""); print!("final "); self.picross.display(); }
        return solve_res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::panic;

    #[test]
    fn test_line_unit_solver_constructor() {
        let clues: ClueLine = vec![1,1];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        picross.values[1] = CaseState::OFF;
        let solver = LineUnitSolver::new(& mut picross);

        assert_eq!(solver.picross.clues.len(), 2);
        assert_eq!(solver.picross.clues[0],1);
        assert_eq!(solver.picross.clues[1],1);

        assert_eq!(solver.picross.values.len(), 3);
        assert!(solver.picross.values[0] == CaseState::UNKNOWN);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::UNKNOWN);

        assert_eq!(solver.editable.len(), 2);
        assert_eq!(solver.editable[0],0);
        assert_eq!(solver.editable[1],2);
    }

    #[test]
    #[should_panic]
    fn test_line_unit_tries() {
        let clues: ClueLine = vec![1,1];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        picross.values[0] = CaseState::ON;
        let mut solver = LineUnitSolver::new(& mut picross);

        solver.init_solve();
        assert_eq!(solver.picross.values.len(), 3);
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::OFF);

        solver.reset_solve();
        assert_eq!(solver.picross.values.len(), 3);
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::UNKNOWN);
        assert!(solver.picross.values[2] == CaseState::UNKNOWN);

        solver.init_solve();
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::OFF);
        solver.next_try();
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::ON);
        solver.next_try();
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::ON);
        assert!(solver.picross.values[2] == CaseState::OFF);
        solver.next_try();
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::ON);
        assert!(solver.picross.values[2] == CaseState::ON);

        panic::set_hook(Box::new(|_| {
            println!("Intended Panic Happened");
        }));

        solver.next_try();
    }

    #[test]
    fn test_line_unit_next() {
        let clues: ClueLine = vec![2];
        let size: usize = 5;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);

        solver.init_solve();
        assert!(solver.next());
        assert!(solver.next());
        assert!(solver.next());
        assert!(solver.next());
        assert!(!solver.next());
    }


    #[test]
    fn test_solve_empty() {
        let clues: ClueLine = vec![0];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        assert!(solver.solve() == Solve::Full);
        assert!(solver.picross.values[0] == CaseState::OFF);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::OFF);
    }
    
    #[test]
    fn test_solve_full() {
        let clues: ClueLine = vec![1,1];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        assert!(solver.solve() == Solve::Full);
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::OFF);
        assert!(solver.picross.values[2] == CaseState::ON);
    }

    #[test]
    fn test_solve_full_one_block() {
        let clues: ClueLine = vec![2];
        let size: usize = 2;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        assert!(solver.solve() == Solve::Full);
        assert!(solver.picross.values[0] == CaseState::ON);
        assert!(solver.picross.values[1] == CaseState::ON);
    }

    #[test]
    fn test_solve_partial_one_block() {
        let clues: ClueLine = vec![3];
        let size: usize = 4;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        assert!(solver.solve() == Solve::Partial);
        assert!(solver.picross.values[0] == CaseState::UNKNOWN);
        assert!(solver.picross.values[1] == CaseState::ON);
        assert!(solver.picross.values[2] == CaseState::ON);
        assert!(solver.picross.values[3] == CaseState::UNKNOWN);
    }

    #[test]
    fn test_solve_partial_complex() {
        let clues: ClueLine = vec![2,2,2];
        let size: usize = 9;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        assert!(solver.solve() == Solve::Partial);
        assert!(solver.picross.values[0] == CaseState::UNKNOWN);
        assert!(solver.picross.values[1] == CaseState::ON);
        assert!(solver.picross.values[2] == CaseState::UNKNOWN);
        assert!(solver.picross.values[3] == CaseState::UNKNOWN);
        assert!(solver.picross.values[4] == CaseState::ON);
        assert!(solver.picross.values[5] == CaseState::UNKNOWN);
        assert!(solver.picross.values[6] == CaseState::UNKNOWN);
        assert!(solver.picross.values[7] == CaseState::ON);
        assert!(solver.picross.values[8] == CaseState::UNKNOWN);
    }

    #[test]
    fn test_space_solve() {
        let clues: ClueLine = vec![1,1,1];
        let size: usize = 8;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        solver.verbose = Verbose::Full;
        solver.solve_method = LineSolveMethod::SpaceDistribution;
        println!("");
        assert!(solver.solve() == Solve::Partial);
    }
}