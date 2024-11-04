use super::*;

use std::cmp;

pub type ClueArray = Vec<Vec<usize>>;
pub type ClueLine = Vec<usize>;

#[derive(Debug)]
pub struct ClueBoard {
    orientation: Orientation,
    values: ClueArray,
}

impl ClueBoard {
    pub fn new_empty(orientation: Orientation, size: usize) -> Self {
        Self {
            orientation,
            values: vec![ Vec::with_capacity(0); size],
        }
    }

    pub fn new_from_clue_array(orientation: Orientation, values: ClueArray) -> Self {
        Self {
            orientation,
            values
        }
    }

    pub fn new_from_image(orientation: Orientation, image: & ImageBoard) -> Self {
        if orientation == Orientation::Vertical {
            let mut values = vec![ Vec::with_capacity(0); image.height()];
            let mut sum : usize = 0;
            for y in 0..image.height() {
                for x in 0..image.width() {
                    match image.get_value(x, y) {
                        CaseState::ON => sum += 1,
                        _ => {
                            if sum > 0 {
                                values[y].push(sum);
                                sum = 0;
                            }
                        },
                    }
                }
                if sum > 0 {
                    values[y].push(sum);
                    sum = 0;
                }
            }
            Self {
                orientation,
                values,
            }
        } else {
            let mut values = vec![ Vec::with_capacity(0); image.width()];
            let mut sum : usize = 0;
            for x in 0..image.width() {
                for y in 0..image.height() {
                    match image.get_value(x, y) {
                        CaseState::ON => sum += 1,
                        _ => {
                            if sum > 0 {
                                values[x].push(sum);
                                sum = 0;
                            }
                        },
                    }
                }
                if sum > 0 {
                    values[x].push(sum);
                    sum = 0;
                }
            }
            Self {
                orientation,
                values,
            }
        }
    }

    pub fn get_orientation(&self) -> Orientation { self.orientation }
    pub fn get_size(&self) -> usize { self.values.len() }
    pub fn get_sub_size(&self, pos : usize) -> usize { self.values[pos].len() }
    pub fn get_max_sub_size(&self) -> usize {
        let mut max_size = 0;
        for i in 0..self.get_size() {
            max_size = cmp::max(max_size, self.get_sub_size(i));
        }
        max_size
    }

    pub fn push_value(&mut self, pos : usize, val: usize) {
        self.values[pos].push(val);
    }

    pub fn get_value(&self, x: usize, y: usize) -> usize {
        match self.orientation {
            Orientation::Vertical => self.values[y][x],
            Orientation::Horizontal => self.values[x][y]
        }
    }

    pub fn get_line(&self, idx : usize) -> &ClueLine {
        return &self.values[idx];
    }

    pub fn get_line_copy(&self, idx : usize) -> ClueLine {
        return self.values[idx].to_vec();
    }
}

#[allow(dead_code)]
pub fn create_triangle_clue_board(size: usize, orientation: Orientation) -> ClueBoard {
    let mut clue = ClueBoard::new_empty(orientation, size);
    let mut cmp = 0;
    for i in 0..size {
        for _j in 0..(i+1) {
            clue.push_value(i, cmp);
            cmp += 1;
        }
    }
    clue
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_v_clue_board() {
        let mut v_clue = ClueBoard::new_empty(Orientation::Vertical, 2);
        assert_eq!(v_clue.get_size(), 2);
        assert_eq!(v_clue.get_sub_size(0), 0);
        assert_eq!(v_clue.get_sub_size(1), 0);
        v_clue.push_value(0, 1);
        v_clue.push_value(1, 2);
        v_clue.push_value(1, 3);
        assert_eq!(v_clue.get_sub_size(0), 1);
        assert_eq!(v_clue.get_sub_size(1), 2);
        v_clue.push_value(0, 4);
        assert_eq!(v_clue.get_sub_size(0), 2);
        assert_eq!(v_clue.get_value(0,0), 1);
        assert_eq!(v_clue.get_value(0,1), 2);
        assert_eq!(v_clue.get_value(1,1), 3);
        assert_eq!(v_clue.get_value(1,0), 4);
    }

    #[test]
    fn test_h_clue_board() {
        let mut h_clue = ClueBoard::new_empty(Orientation::Horizontal, 2);
        assert_eq!(h_clue.get_size(), 2);
        assert_eq!(h_clue.get_sub_size(0), 0);
        assert_eq!(h_clue.get_sub_size(1), 0);
        h_clue.push_value(0, 1);
        h_clue.push_value(1, 2);
        h_clue.push_value(1, 3);
        assert_eq!(h_clue.get_sub_size(0), 1);
        assert_eq!(h_clue.get_sub_size(1), 2);
        h_clue.push_value(0, 4);
        assert_eq!(h_clue.get_sub_size(0), 2);
        assert_eq!(h_clue.get_value(0,0), 1);
        assert_eq!(h_clue.get_value(1,0), 2);
        assert_eq!(h_clue.get_value(1,1), 3);
        assert_eq!(h_clue.get_value(0,1), 4);
    }
}