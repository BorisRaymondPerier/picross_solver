use super::*;

use std::io::{stdout, Write};
use crossterm::{ queue, style::{self, SetForegroundColor, ResetColor, Colorize}};


pub fn display_line(values : &Vec<CaseState>) {
    let mut stdout = stdout();
    for i in 0..values.len() {
        match values[i] {
            CaseState::UNKNOWN => queue!(stdout, SetForegroundColor(UNKNOWN_COLOR), style::PrintStyledContent( UNKNOWN_STR.grey() ) , ResetColor).ok(),
            CaseState::ON => queue!(stdout, SetForegroundColor(ON_COLOR), style::PrintStyledContent( ON_STR.black() ), ResetColor).ok(),
            CaseState::OFF => queue!(stdout, SetForegroundColor(OFF_COLOR), style::PrintStyledContent( OFF_STR.white() ), ResetColor).ok(),
        };
    }
}

impl Displayable for PicrossLineUnit {
    fn display_in_context(&self, context : & DisplayContext) {
        let mut stdout = stdout();

        for i in 0..self.clues.len() {
            queue!(stdout, style::Print( format!("{:>3}", self.clues[i]))).ok();
        }

        display_line(&self.values);

        self.go_to_next_line(context);
    }

    fn display_width(&self) -> u16 {
       (self.clues.len() * EMPTY_STR_VERTICAL.len() + (self.values.len() * CELL_SIZE)) as u16
    }

    fn display_height(&self) -> u16 {
        1
    }
}
