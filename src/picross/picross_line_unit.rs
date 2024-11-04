use super::*;

pub struct PicrossLineUnit {
    pub clues: ClueLine,
    pub values: Vec<CaseState>
}

impl PicrossLineUnit {
    pub fn new(size: usize, clues: ClueLine) -> Self {
        Self {
            clues,
            values: vec![CaseState::UNKNOWN; size],
        }
    }

    pub fn get_value(&self, idx : usize) -> CaseState {
        self.values[idx]
    }

    pub fn set_value(&mut self, idx : usize, val : CaseState) {
        self.values[idx] = val;
    }

    pub fn free_space_count(&self) -> usize {
        let sum_of_occupied : usize = self.clues.iter().sum::<usize>() + self.clues.len() - 1;
        let total_space = self.values.len();
        if sum_of_occupied > total_space {
            panic!("This line is to small for the given clues");
        }
        total_space - sum_of_occupied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::CaseState::*;

    use std::panic;

    #[test]
    fn test_picross_line_unit() {
        let clues: ClueLine = vec![1,1];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        assert_eq!(picross.values.len(), 3);
        assert!(picross.get_value(0) == UNKNOWN);
        picross.set_value(0, ON);
        picross.set_value(1, OFF);
        picross.set_value(2, ON);
        assert!(picross.get_value(0) == ON);
        assert!(picross.get_value(1) == OFF);
    }

    #[test]
    #[should_panic]
    fn test_space_compute() {
        let clues: ClueLine = vec![0];
        let size: usize = 0;
        let picross = PicrossLineUnit::new(size, clues);
        assert_eq!(picross.free_space_count(), 0);

        let clues: ClueLine = vec![5];
        let size: usize = 5;
        let picross = PicrossLineUnit::new(size, clues);
        assert_eq!(picross.free_space_count(), 0);

        let clues: ClueLine = vec![2,1,2];
        let size: usize = 7;
        let picross = PicrossLineUnit::new(size, clues);
        assert_eq!(picross.free_space_count(), 0);

        let clues: ClueLine = vec![2,1,2];
        let size: usize = 10;
        let picross = PicrossLineUnit::new(size, clues);
        assert_eq!(picross.free_space_count(), 3);

        let clues: ClueLine = vec![2,1,2];
        let size: usize = 6;
        let picross = PicrossLineUnit::new(size, clues);

        panic::set_hook(Box::new(|_| {
            println!("Intended Panic Happened");
        }));
        picross.free_space_count();
    }
}