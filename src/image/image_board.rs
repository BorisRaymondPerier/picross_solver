use super::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CaseState {
    UNKNOWN = 0,
    ON = 1,
    OFF = 2,
}

#[derive(Debug)]
pub struct ImageBoard {
    height: usize,
    width: usize,
    values: Vec<CaseState>,
}

impl ImageBoard {
    pub fn new_empty(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            values: vec![CaseState::UNKNOWN; height*width],
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn set_value(&mut self, x : usize, y : usize, val : CaseState) {
        self.values[x + y * self.width] = val;
    }

    pub fn get_value(&self, x : usize, y : usize) -> CaseState {
        self.values[x + y * self.width]
    }

    pub fn get_row(& self, idx : usize) -> ImageLine {
        return ImageLine {
            image : self,
            idx,
            orientation : Orientation::Horizontal,
        }
    }

    pub fn get_col(& self, idx : usize) -> ImageLine {
        return ImageLine {
            image : self,
            idx,
            orientation : Orientation::Vertical,
        }
    }
}

#[allow(dead_code)]
pub fn create_one_line_board(values : & [CaseState]) -> ImageBoard {
    let mut image : ImageBoard = ImageBoard::new_empty(1, values.len());
    for i in 0..values.len() {
        image.set_value(i, 0, values[i]);
    }
    image
}

#[allow(dead_code)]
pub fn create_cross_image_board(size: usize) -> ImageBoard {
    let mut image = ImageBoard::new_empty(size,size);
    for y in 0..size {
        for x in 0..size {
            if y == 0 || y == (size-1) ||  x == 0 || x == (size-1) || x == y || x == (size - 1 - y)  {
                image.set_value(x,y,CaseState::ON);
            }
            else if x % 2 == y % 2 {
                 image.set_value(x,y,CaseState::OFF);
            }
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_board_basics() {
        let mut image = ImageBoard::new_empty(1,2);
        assert_eq!(image.height, 1);
        assert_eq!(image.width, 2);
        assert_eq!(image.values.len(), 2);
        assert!(image.get_value(0,0) == CaseState::UNKNOWN);
        assert!(image.get_value(1,0) == CaseState::UNKNOWN);
        image.set_value(0,0,CaseState::ON);
        image.set_value(1,0,CaseState::OFF);
        assert!(image.get_value(0,0) == CaseState::ON);
        assert!(image.get_value(1,0) == CaseState::OFF);
    }
}