use super::*;
use std::cmp;

use std::io::{stdout, Write};
use crossterm::{ queue, style, cursor};

pub const EMPTY_STR_VERTICAL : &str = "   ";
pub const EMPTY_STR_HORIZONTAL : &str = "  ";
pub const HINT_VAL_VERTICAL_FORMAT : &str = "{:>3}";
pub const HINT_VAL_HORIZONTAL_FORMAT : &str = "{:>2}";

impl ClueBoard {

    fn display_left(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        let max_size = cmp::max(self.get_max_sub_size(), 1);
    
        for i in 0..self.get_size() {
            let empty = max_size - self.get_sub_size(i);
    
            if empty == max_size {
                for _j in 0..(empty-1) {
                    queue!(stdout, style::Print( EMPTY_STR_VERTICAL )).ok();
                }
    
                queue!(stdout, style::Print( format!("{:>3}", 0))).ok();
    
            } else {
                for _j in 0..empty {
                    queue!(stdout, style::Print( EMPTY_STR_VERTICAL )).ok();
                }
        
                for j in 0..self.get_sub_size(i){
                    queue!(stdout, style::Print( format!("{:>3}", self.get_value(j,i)))).ok();
                }
            }
    
            self.go_to_next_line(context);
        }
    }

    fn display_top(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        let  max_size =  cmp::max(self.get_max_sub_size(), 1);
        
        for i in (0..max_size).rev() {
            queue!(stdout, cursor::MoveRight(context.left_offset)).ok();
            for j in 0..self.get_size() {
                let height = self.get_sub_size(j);
                if (height == 0) && i == 0 {
                    queue!(stdout, style::Print( format!("{:>2}", 0))).ok();
                } else if i > (height-1) {
                    queue!(stdout, style::Print( EMPTY_STR_HORIZONTAL )).ok();
                } else {
                    queue!(stdout, style::Print( format!("{:>2}", self.get_value(j,height - 1 - i)))).ok();
                }
            }
    
            self.go_to_next_line(context);
        }
    }
}

impl Displayable for ClueBoard {
    fn display_in_context(&self, context : & DisplayContext) {
        match self.get_orientation() {
            Orientation::Vertical => self.display_left(context),
            Orientation::Horizontal => self.display_top(context),
        }
    }

    fn display_height(&self) -> u16 {
        match self.get_orientation() {
            Orientation::Vertical => self.get_size() as u16,
            Orientation::Horizontal => cmp::max(self.get_max_sub_size(), 1) as u16,
        }
    }
    
    fn display_width(&self) -> u16 {
        match self.get_orientation() {
            Orientation::Vertical => (cmp::max(self.get_max_sub_size(),1) * EMPTY_STR_VERTICAL.len()) as u16,
            Orientation::Horizontal => (self.get_size() * EMPTY_STR_HORIZONTAL.len()) as u16,
        }
    }
}
