use super::*;

use std::io::{stdout, Write};
use crossterm::{ queue, style::{self, SetForegroundColor, ResetColor, Color, Colorize} };

pub const CELL_SIZE : usize = 2;
pub const ON_STR : &str = "██";
pub const OFF_STR : &str = "██";
pub const UNKNOWN_STR : &str = "██";
pub const ON_COLOR : Color = Color::Black;
pub const OFF_COLOR : Color = Color::White;
pub const UNKNOWN_COLOR : Color = Color::Grey;

impl Displayable for ImageBoard {

    fn display_in_context(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get_value(x,y) {
                    CaseState::UNKNOWN => queue!(stdout, SetForegroundColor(UNKNOWN_COLOR), style::PrintStyledContent( UNKNOWN_STR.dark_grey() ) , ResetColor).ok(),
                    CaseState::ON => queue!(stdout, SetForegroundColor(ON_COLOR), style::PrintStyledContent( ON_STR.black() ), ResetColor).ok(),
                    CaseState::OFF => queue!(stdout, SetForegroundColor(OFF_COLOR), style::PrintStyledContent( OFF_STR.white() ), ResetColor).ok(),
                };
            }
            self.go_to_next_line(context);
        }
    }
    
    fn display_height(&self) -> u16 {
        self.height() as u16
    }
    
    fn display_width(&self) -> u16 {
        (self.width() * CELL_SIZE) as u16
    }
}