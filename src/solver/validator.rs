use super::*;

pub fn validate_line(clues : & ClueLine, line : & ImageLine) -> bool {
    let nb_clues = clues.len();
    let mut current_clue : usize = 0;
    let mut current_block_size : usize = 0;
    for i in 0..line.size() {
        match line.get(i) {
            CaseState::ON => { //Start or continuing a block
                //Check if a clue remains
                if current_clue == nb_clues {
                    return false;
                }
                
                //Then increase block size
                current_block_size += 1;

                //Check if block is already too big
                if current_block_size > clues[current_clue] {
                    return false;
                }
            },
            _ => {
                if current_block_size > 0 {

                    //Check if terminated block has good size
                    if current_block_size != clues[current_clue] {
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
        if current_block_size != clues[current_clue] {
            return false;
        }
        current_clue += 1;
    }

    //Check if we have validated all clues
    if (current_clue != nb_clues) && (clues[current_clue] != 0) {
        return false;
    }

    true
}

pub fn validate_row(picross : & mut PicrossBoard, idx : usize) -> bool{
    validate_line(&picross.left.get_line(idx), &picross.image.get_row(idx))
}

pub fn validate_col(picross : & mut PicrossBoard, idx : usize) -> bool{
    validate_line(&picross.top.get_line(idx), &picross.image.get_col(idx))
}

pub fn validate_picross(picross : & mut PicrossBoard) -> bool {
    let height = picross.height();
    let width = picross.width();
    for i in 0..height {
        if !validate_row(picross, i) {
            return false;
        }
    }
    for i in 0..width {
        if !validate_col(picross, i) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::image::CaseState::*;

    #[test]
    fn test_validation() {
        let size : usize = 10;
        let img : ImageBoard = create_cross_image_board(size);
        let mut picross : PicrossBoard = PicrossBoard::new_from_image(&img);
        validate_picross(& mut picross);
    }

    #[test]
    fn test_full_line_validation4() {
        let img : ImageBoard = create_one_line_board(&[ON, ON, ON, ON]);
        let clues = vec![4];
        assert!(validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_full_line_validation2() {
        let img : ImageBoard = create_one_line_board(&[ON, ON]);
        let clues = vec![2];
        assert!(validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_empty_line_validation() {
        let img : ImageBoard = create_one_line_board(&[OFF, OFF, OFF, OFF]);
        let clues = vec![0];
        assert!(validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_one_block_validation() {
        let clues = vec![2];

        let img : ImageBoard = create_one_line_board(&[ON, ON, OFF, OFF]);
        assert!(validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, ON, OFF]);
        assert!(validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, OFF, ON, ON]);
        assert!(validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_two_block_validation() {
        let clues = vec![1, 1];

        let img : ImageBoard = create_one_line_board(&[ON, OFF, ON, OFF]);
        assert!(validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, OFF, ON]);
        assert!(validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[ON, OFF, OFF, ON]);
        assert!(validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_too_big_block_validation() {
        let clues = vec![5];
        let img : ImageBoard = create_one_line_board(&[ON, ON, ON, ON]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, OFF, OFF, OFF]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let clues = vec![1, 3];
        let img : ImageBoard = create_one_line_board(&[ON, OFF, ON, ON]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let clues = vec![3, 1];
        let img : ImageBoard = create_one_line_board(&[ON, ON, OFF, ON]);
        assert!(!validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_too_small_block_validation() {
        let clues = vec![3];

        let img : ImageBoard = create_one_line_board(&[ON, ON, OFF, OFF]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, ON, OFF]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, OFF, ON, ON]);
        assert!(!validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_too_many_clues_validation() {

        let clues = vec![2,1];
        let img : ImageBoard = create_one_line_board(&[ON, ON, OFF, OFF]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let clues = vec![1,1,1];
        let img : ImageBoard = create_one_line_board(&[ON, OFF, ON, OFF]);
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, OFF, ON]);        
        assert!(!validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_too_few_clues_validation() {

        let clues = vec![1];
        let img : ImageBoard = create_one_line_board(&[ON, OFF, ON, OFF]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, OFF, ON]);        
        assert!(!validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_not_empty_line_validation() {

        let clues = vec![0];
        let img : ImageBoard = create_one_line_board(&[ON, OFF, OFF, OFF]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, OFF, ON, OFF]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, OFF, OFF, ON]);        
        assert!(!validate_line(&clues, &img.get_row(0)));
    }

    #[test]
    fn test_not_full_line_validation() {

        let clues = vec![4];
        let img : ImageBoard = create_one_line_board(&[ON, ON, ON, OFF]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[OFF, ON, ON, ON]);        
        assert!(!validate_line(&clues, &img.get_row(0)));

        let img : ImageBoard = create_one_line_board(&[ON, ON, OFF, ON]);        
        assert!(!validate_line(&clues, &img.get_row(0)));
    }
}