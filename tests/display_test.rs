extern crate picross_solver;

use crate::picross_solver::display::*;
use crate::picross_solver::solver::*;

fn copy_image_in_picross(picross : &mut PicrossBoard, img : & ImageBoard){
    for y in 0..img.height() {
        for x in 0..img.width() {
            picross.image.set_value(x, y, img.get_value(x, y));
        }
    }
}

fn display_empty_picross() {
    let width : usize = 16;
    let height : usize = 10;
    let picross : PicrossBoard = PicrossBoard::new_empty(height, width);
    picross.display()
}


fn display_generated_x_shape_picross() {
    let size : usize = 10;
    let img : ImageBoard = create_cross_image_board(size);
    let mut picross : PicrossBoard = PicrossBoard::new_from_image(&img);
    copy_image_in_picross(&mut picross, &img);
    picross.display()
}

fn display_picross_generated_from_string() {
    let picross = PicrossBoard::picross_from_clue_string(&mut CLUE_STRING_20X20.to_string());
    picross.display()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_cross_picross_validation() {
        let size : usize = 10;
        let img : ImageBoard = create_cross_image_board(size);
        let mut picross : PicrossBoard = PicrossBoard::new_from_image(&img);
        copy_image_in_picross(&mut picross, &img);
        assert_eq!(validate_picross(&mut picross), true);
    }

    #[test]
    #[serial]
    fn test_validation_fail() {
        let mut picross = PicrossBoard::picross_from_clue_string(&mut CLUE_STRING_20X20.to_string());
        assert_eq!(validate_picross(& mut picross), false);
    }

    #[test]
    #[serial]
    fn sequential_display_tests(){
        println!("");
        println!("Draw an empty picross");
        display_empty_picross();
        println!("Draw a X shape picross");
        display_generated_x_shape_picross();
        println!("Draw a picross that is generated using : {:?}", CLUE_STRING_20X20.to_string());
        display_picross_generated_from_string();
    }

    #[test]
    #[serial]
    fn row_col_picross_display_test(){
        println!("");
        let picross = PicrossBoard::picross_from_clue_string(& CLUE_STRING_10X10.to_string());
        let mut solver = RowColPicross::new(&picross);
        solver.display();

        while solver.solve_iteration() != Solve::Full {
            solver.merge_rows_and_cols();
            solver.display();
        }

        solver.merge_rows_and_cols();
        solver.display();
    }

    #[test]
    #[serial]
    fn picross_line_unit_display_test(){
        println!("");
        let clues: ClueLine = vec![1,1,2,1,1];
        let size: usize = 10;
        let mut picross = PicrossLineUnit::new(size, clues);
        picross.display();

        let mut solver = LineUnitSolver::new(& mut picross);
        solver.solve();
        solver.picross.display()
    }

    #[test]
    #[serial]
    fn one_line_solve_partial_display_test(){
        println!("");
        let clues: ClueLine = vec![6,1];
        let size: usize = 10;
        let mut picross = PicrossLineUnit::new(size, clues);
        picross.values[7] = CaseState::OFF;
        let mut solver = LineUnitSolver::new(& mut picross);
        solver.verbose = Verbose::Partial;
        solver.solve();
    }

    #[test]
    #[serial]
    fn one_line_solve_full_display_test(){
        println!("");
        let clues: ClueLine = vec![2];
        let size: usize = 3;
        let mut picross = PicrossLineUnit::new(size, clues);
        let mut solver = LineUnitSolver::new(& mut picross);
        solver.verbose = Verbose::Full;
        solver.solve();
    }
}