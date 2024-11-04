use super::*;

pub struct ImageLine<'a> {
    pub image: &'a ImageBoard,
    pub idx: usize,
    pub orientation: Orientation,
}

impl ImageLine<'_> {
    pub fn get(&self, idx : usize) -> CaseState {
        match self.orientation {
            Orientation::Horizontal => self.image.get_value(idx, self.idx),
            Orientation::Vertical => self.image.get_value(self.idx, idx),
        }
    }

    pub fn size(&self) -> usize {
        match self.orientation {
            Orientation::Horizontal => self.image.width(),
            Orientation::Vertical => self.image.height(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_line_access() {
        let mut image = ImageBoard::new_empty(3,3);
        image.set_value(0,0,CaseState::OFF);
        image.set_value(1,1,CaseState::ON);
        image.set_value(2,2,CaseState::OFF);

        let row = image.get_row(0);
        assert_eq!(row.size(), 3);
        assert!(row.get(0) == CaseState::OFF);
        assert!(row.get(1) == CaseState::UNKNOWN);
        assert!(row.get(2) == CaseState::UNKNOWN);

        let row = image.get_row(1);
        assert_eq!(row.size(), 3);
        assert!(row.get(0) == CaseState::UNKNOWN);
        assert!(row.get(1) == CaseState::ON);
        assert!(row.get(2) == CaseState::UNKNOWN);

        let row = image.get_row(2);
        assert_eq!(row.size(), 3);
        assert!(row.get(0) == CaseState::UNKNOWN);
        assert!(row.get(1) == CaseState::UNKNOWN);
        assert!(row.get(2) == CaseState::OFF);

        let col = image.get_col(0);
        assert_eq!(col.size(), 3);
        assert!(col.get(0) == CaseState::OFF);
        assert!(col.get(1) == CaseState::UNKNOWN);
        assert!(col.get(2) == CaseState::UNKNOWN);

        let col = image.get_col(1);
        assert_eq!(col.size(), 3);
        assert!(col.get(0) == CaseState::UNKNOWN);
        assert!(col.get(1) == CaseState::ON);
        assert!(col.get(2) == CaseState::UNKNOWN);

        let col = image.get_col(2);
        assert_eq!(col.size(), 3);
        assert!(col.get(0) == CaseState::UNKNOWN);
        assert!(col.get(1) == CaseState::UNKNOWN);
        assert!(col.get(2) == CaseState::OFF);
    }
}