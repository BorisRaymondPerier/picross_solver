use super::*;

pub fn invalidate_line(clues : & ClueLine, line : & ImageLine) -> bool {
    let size = line.size();
    let range = get_clue_index_range(clues, size);
    println!("range : {:?}, size {}", range, size);
    return false;
}

pub fn invalidate_row(picross : & mut PicrossBoard, idx : usize) -> bool{
    invalidate_line(&picross.left.get_line(idx), &picross.image.get_row(idx))
}

pub fn invalidate_col(picross : & mut PicrossBoard, idx : usize) -> bool{
    invalidate_line(&picross.top.get_line(idx), &picross.image.get_col(idx))
}

pub fn invalidate_picross(picross : & mut PicrossBoard) -> bool {
    let height = picross.height();
    let width = picross.width();
    for i in 0..height {
        if invalidate_row(picross, i) {
            return true;
        }
    }
    for i in 0..width {
        if invalidate_col(picross, i) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::image::CaseState::*;

    #[test]
    fn test_full_line_validation() {
        let img = create_one_line_board(&[ON, OFF, ON, OFF, OFF]);
        let clues = vec![1,1];
        assert!(!invalidate_line(&clues, &img.get_row(0)));
    }
}