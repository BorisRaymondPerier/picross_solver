use super::*;

pub struct RowColPicross {
    pub rows: Vec<PicrossLineUnit>,
    pub cols: Vec<PicrossLineUnit>,
    pub line_solve_method: LineSolveMethod,
}

fn get_rows_from_picross(picross : & PicrossBoard) -> Vec<PicrossLineUnit> {
    let height = picross.height();
    let width = picross.width();
    let mut rows: Vec<PicrossLineUnit> = Vec::with_capacity(height);
    for i in 0..height { 
        rows.push(PicrossLineUnit::new(width, picross.left.get_line_copy(i)));
    }
    return rows;
}

fn get_cols_from_picross(picross : & PicrossBoard) -> Vec<PicrossLineUnit> {
    let height = picross.height();
    let width = picross.width();
    let mut cols: Vec<PicrossLineUnit> = Vec::with_capacity(width);
    for i in 0..width { 
        cols.push(PicrossLineUnit::new(height, picross.top.get_line_copy(i)));
    }
    return cols;
}

fn get_solve_combination(line_solve: Solve, global_solve: Solve) -> Solve {
    match (global_solve, line_solve) {
        (Solve::None, Solve::None) => Solve::None,
        (Solve::Full, Solve::Full) => Solve::Full,
        (Solve::Unknown, _ ) => line_solve,
        _ => Solve::Partial
    }
}

pub fn solve_picross_board(picross : &mut PicrossBoard, method : LineSolveMethod) -> Solve {
    let mut solver = RowColPicross::new(&picross);
    solver.line_solve_method = method;
    let solve = solver.solve();
    match solve {
        Solve::Partial | Solve::Full => {
            for x in 0..solver.cols.len() {
                for y in 0..solver.rows.len() {
                    picross.image.set_value(x,y,solver.rows[y].values[x]);
                }
            }
        }
        Solve::Unknown => {
            panic!("Solve shouln't end as unknown");
        }
        _ => {}
    }
    return solve;
}

impl RowColPicross{
    pub fn new(picross : & PicrossBoard) -> Self {
        Self {
            rows : get_rows_from_picross(picross),
            cols : get_cols_from_picross(picross),
            line_solve_method : LineSolveMethod::BrutForce,
        }
    }

    pub fn merge_rows_and_cols(&mut self) {
        for r in 0..self.rows.len() {
            for c in 0..self.cols.len() {
                match (self.rows[r].values[c], self.cols[c].values[r]) {
                    (_, CaseState::UNKNOWN) => self.cols[c].values[r] = self.rows[r].values[c],
                    (CaseState::UNKNOWN,_) => self.rows[r].values[c] = self.cols[c].values[r],
                    (_,_) => {
                        if self.rows[r].values[c] != self.cols[c].values[r] {
                            panic!("Horizontal Solve and Vertical Solve give incompatible result");
                        }
                    }
                }
            }
        }
    }

    pub fn solve_iteration(&mut self) -> Solve {
        let mut current_solve = Solve::Unknown;
        for row in self.rows.iter_mut() {
            let mut solver = LineUnitSolver::new(row);
            if self.line_solve_method == LineSolveMethod::SpaceDistribution { solver.solve_method = LineSolveMethod::SpaceDistribution; }
            current_solve = get_solve_combination(solver.solve(), current_solve);
        }
        for col in self.cols.iter_mut() {
            let mut solver = LineUnitSolver::new(col);
            if self.line_solve_method == LineSolveMethod::SpaceDistribution { solver.solve_method = LineSolveMethod::SpaceDistribution; }
            current_solve = get_solve_combination(solver.solve(), current_solve);
        }
        return current_solve;
    }
}

impl Solvable for RowColPicross {
    fn solve(&mut self) -> Solve {
        let mut global_solve = Solve::Unknown;
        loop {
            let iter_solve = self.solve_iteration();
            match (global_solve, iter_solve) {
                (Solve::Unknown, Solve::None)    => return Solve::None,
                (Solve::Partial, Solve::None)    => return Solve::Partial,
                (_             , Solve::Partial) => { global_solve = Solve::Partial; }
                (_             , Solve::Full)    => return Solve::Full,
                (_,_) => panic!("Impossible solve state"),
            }
            self.merge_rows_and_cols();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::panic;

    #[test]
    fn test_row_col_picross() {
        let mut picross_str = "T1,2L2,1".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        assert_eq!(picross_base.height(), 2);
        assert_eq!(picross_base.width(), 2);
        let picross = RowColPicross::new(&picross_base);
        assert_eq!(picross.rows.len(), 2);
        assert_eq!(picross.cols.len(), 2);
        assert_eq!(picross.rows[0].clues.len(), 1);
        assert_eq!(picross.rows[0].clues[0], 2);
        assert_eq!(picross.rows[1].clues.len(), 1);
        assert_eq!(picross.rows[1].clues[0], 1);
        assert_eq!(picross.cols[0].clues.len(), 1);
        assert_eq!(picross.cols[0].clues[0], 1);
        assert_eq!(picross.cols[1].clues.len(), 1);
        assert_eq!(picross.cols[1].clues[0], 2);
    }

    #[test]
    fn test_rowcolpicross_empty_solve_iteration() {
        let mut picross_str = "T0,0L0,0".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        assert!(picross.solve_iteration() == Solve::Full);
        assert!(picross.rows[0].values[0] == CaseState::OFF);
        assert!(picross.rows[0].values[1] == CaseState::OFF);
        assert!(picross.rows[1].values[0] == CaseState::OFF);
        assert!(picross.rows[1].values[1] == CaseState::OFF);
        assert!(picross.cols[0].values[0] == CaseState::OFF);
        assert!(picross.cols[0].values[1] == CaseState::OFF);
        assert!(picross.cols[1].values[0] == CaseState::OFF);
        assert!(picross.cols[1].values[1] == CaseState::OFF);
    }

    #[test]
    fn test_rowcolpicross_full_solve_iteration() {
        let mut picross_str = "T2,2L2,2".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        assert!(picross.solve_iteration() == Solve::Full);
        assert!(picross.rows[0].values[0] == CaseState::ON);
        assert!(picross.rows[0].values[1] == CaseState::ON);
        assert!(picross.rows[1].values[0] == CaseState::ON);
        assert!(picross.rows[1].values[1] == CaseState::ON);
        assert!(picross.cols[0].values[0] == CaseState::ON);
        assert!(picross.cols[0].values[1] == CaseState::ON);
        assert!(picross.cols[1].values[0] == CaseState::ON);
        assert!(picross.cols[1].values[1] == CaseState::ON);
    }

    #[test]
    fn test_rowcolpicross_solve_iteration_on_already_solved() {
        let mut picross_str = "T1L1".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        picross.rows[0].values[0] = CaseState::ON;
        picross.cols[0].values[0] = CaseState::ON;
        assert!(picross.solve_iteration() == Solve::Full);
        assert!(picross.rows[0].values[0] == CaseState::ON);
        assert!(picross.cols[0].values[0] == CaseState::ON);
    }

    #[test]
    fn test_rowcolpicross_solve_iteration_complex() {
        let mut picross_str = "T1,1,2L11,2".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        assert!(picross.solve_iteration() == Solve::Partial);

        assert!(picross.rows[0].values[0] == CaseState::ON);
        assert!(picross.rows[0].values[1] == CaseState::OFF);
        assert!(picross.rows[0].values[2] == CaseState::ON);
        assert!(picross.rows[1].values[0] == CaseState::UNKNOWN);
        assert!(picross.rows[1].values[1] == CaseState::ON);
        assert!(picross.rows[1].values[2] == CaseState::UNKNOWN);
        assert!(picross.cols[0].values[0] == CaseState::UNKNOWN);
        assert!(picross.cols[0].values[1] == CaseState::UNKNOWN);
        assert!(picross.cols[1].values[0] == CaseState::UNKNOWN);
        assert!(picross.cols[1].values[1] == CaseState::UNKNOWN);
        assert!(picross.cols[2].values[0] == CaseState::ON);
        assert!(picross.cols[2].values[1] == CaseState::ON);
    }

    #[test]
    fn test_rowcolpicross_usefull_merge() {
        let mut picross_str = "T0L0,0".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        
        picross.rows[0].values[0] = CaseState::ON;
        picross.cols[0].values[1] = CaseState::OFF;
        assert!(picross.cols[0].values[0] == CaseState::UNKNOWN);
        assert!(picross.rows[1].values[0] == CaseState::UNKNOWN);
        picross.merge_rows_and_cols();
        assert!(picross.cols[0].values[0] == CaseState::ON);
        assert!(picross.rows[1].values[0] == CaseState::OFF);
    }

    
    #[test]
    fn test_rowcolpicross_merge_with_same_value() {
        let mut picross_str = "T0L0".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);

        picross.rows[0].values[0] = CaseState::ON;
        picross.cols[0].values[0] = CaseState::ON;
        picross.merge_rows_and_cols();
        assert!(picross.rows[0].values[0] == CaseState::ON);
        assert!(picross.cols[0].values[0] == CaseState::ON);
    }

    #[test]
    #[should_panic]
    fn test_rowcolpicross_merge_panic() {
        let mut picross_str = "T0L0".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);

        picross.rows[0].values[0] = CaseState::ON;
        picross.cols[0].values[0] = CaseState::OFF;

        panic::set_hook(Box::new(|_| {
            println!("Intended Panic Happened");
        }));

        picross.merge_rows_and_cols();
    }

    #[test]
    fn test_solve_complex() {
        let mut picross_str = "T1,1,2L11,2".to_string();
        let picross_base = PicrossBoard::picross_from_clue_string(&mut picross_str);
        let mut picross = RowColPicross::new(&picross_base);
        assert!(picross.solve() == Solve::Full);

        assert!(picross.rows[0].values[0] == CaseState::ON);
        assert!(picross.rows[0].values[1] == CaseState::OFF);
        assert!(picross.rows[0].values[2] == CaseState::ON);
        assert!(picross.rows[1].values[0] == CaseState::OFF);
        assert!(picross.rows[1].values[1] == CaseState::ON);
        assert!(picross.rows[1].values[2] == CaseState::ON);

        assert!(picross.cols[0].values[0] == CaseState::ON);
        assert!(picross.cols[0].values[1] == CaseState::OFF);
        assert!(picross.cols[1].values[0] == CaseState::OFF);
        assert!(picross.cols[1].values[1] == CaseState::ON);
        assert!(picross.cols[2].values[0] == CaseState::ON);
        assert!(picross.cols[2].values[1] == CaseState::ON);
    }
}