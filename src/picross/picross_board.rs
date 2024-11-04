use super::*;

#[derive(Debug)]
pub struct PicrossBoard {
    pub left: ClueBoard,
    pub top: ClueBoard,
    pub image: ImageBoard,
}

impl PicrossBoard {
    pub fn new_empty(height: usize, width: usize) -> Self {
        Self {
            left: ClueBoard::new_empty(Orientation::Vertical, height),
            top: ClueBoard::new_empty(Orientation::Horizontal, width),
            image: ImageBoard::new_empty(height, width),
        }
    }

    pub fn picross_from_clue_string( clue_string : & String) -> Self {
         let (top_array, left_array) = clue_boards_from_clue_string(clue_string);
         let top = ClueBoard::new_from_clue_array(Orientation::Horizontal, top_array);
         let left = ClueBoard::new_from_clue_array(Orientation::Vertical, left_array);
         let height = left.get_size();
         let width = top.get_size();
         Self {
             left,
             top,
             image: ImageBoard::new_empty(height, width)
         }
    }

    pub fn new_from_image(image: &ImageBoard) -> Self {
        Self {
            left: ClueBoard::new_from_image(Orientation::Vertical, image),
            top: ClueBoard::new_from_image(Orientation::Horizontal, image),
            image: ImageBoard::new_empty(image.height(), image.width()),
        }
    }

    pub fn height(&self) -> usize {
        self.image.height()
    }

    pub fn width(&self) -> usize {
        self.image.width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picross_board() {
        let mut picross = PicrossBoard::new_empty(2, 3);
        assert_eq!(picross.image.height(), 2);
        assert_eq!(picross.image.width(), 3);
        assert_eq!(picross.left.get_size(), 2);
        assert_eq!(picross.top.get_size(), 3);
        
        picross.image.set_value(2,1,CaseState::ON);
        assert!(picross.image.get_value(2,1) == CaseState::ON);
        picross.left.push_value(1,1);
        assert_eq!(picross.left.get_value(0,1), 1);
        picross.top.push_value(2,1);
        assert_eq!(picross.top.get_value(2,0), 1);
    }
}
