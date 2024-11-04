extern crate picross_solver;

use crate::picross_solver::display::*;
use crate::picross_solver::solver::*;

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;

    fn test_solve_and_display_common(s : &str, method : LineSolveMethod) {
        println!("");
        let mut picross = PicrossBoard::picross_from_clue_string(&mut s.to_string());
        solve_picross_board(&mut picross, method);
        picross.display()
    }

    #[test] #[serial] fn test_solve_brut_force_5x5() { test_solve_and_display_common(CLUE_STRING_5X5, LineSolveMethod::BrutForce); }
    #[test] #[serial] fn test_solve_brut_force_10x10() { test_solve_and_display_common(CLUE_STRING_10X10, LineSolveMethod::BrutForce); }
    #[test] #[serial] fn test_solve_brut_force_15x15() { test_solve_and_display_common(CLUE_STRING_15X15, LineSolveMethod::BrutForce); }
    #[test] #[serial] #[ignore = "Too long"] fn test_solve_brut_force_20x20() { test_solve_and_display_common(CLUE_STRING_20X20, LineSolveMethod::BrutForce); }
    #[test] #[serial] #[ignore = "Too long"] fn test_solve_brut_force_25x25() { test_solve_and_display_common(CLUE_STRING_25X25, LineSolveMethod::BrutForce); }
    #[test] #[serial] #[ignore = "Too long"] fn test_solve_brut_force_30x30() { test_solve_and_display_common(CLUE_STRING_30X30, LineSolveMethod::BrutForce); }

    #[test] #[serial] fn test_solve_space_distribution_15x15() { test_solve_and_display_common(CLUE_STRING_15X15, LineSolveMethod::SpaceDistribution); }
    #[test] #[serial] fn test_solve_space_distribution_20x20() { test_solve_and_display_common(CLUE_STRING_20X20, LineSolveMethod::SpaceDistribution); }
    #[test] #[serial] #[ignore = "Too long"] fn test_solve_space_distribution_25x25() { test_solve_and_display_common(CLUE_STRING_25X25, LineSolveMethod::SpaceDistribution); }
    #[test] #[serial] #[ignore = "Too long"] fn test_solve_space_distribution_30x30() { test_solve_and_display_common(CLUE_STRING_30X30, LineSolveMethod::SpaceDistribution); }

    #[test] 
    #[serial]
    #[ignore = "Too long"]
    fn test_all_solve_brut_force() {
        test_solve_and_display_common(CLUE_STRING_5X5, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_6X6, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_7X7, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_8X8, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_9X9, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_10X10, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_12X12, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_15X15, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_16X16, LineSolveMethod::BrutForce);
        test_solve_and_display_common(CLUE_STRING_20X20, LineSolveMethod::BrutForce);
    }
}