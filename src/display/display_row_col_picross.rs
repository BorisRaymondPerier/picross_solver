use super::*;

use std::io::{stdout, Write};
use crossterm::{ queue, style::{self, SetForegroundColor, ResetColor, Colorize}, cursor };
use std::cmp;

fn get_max_clue_size(lines : & Vec<PicrossLineUnit>) -> usize {
    let mut max = lines[0].clues.len();
    for line in lines[1..].iter() {
        max = cmp::max(max, line.clues.len());
    }
    cmp::max(max, 1)
}

impl RowColPicross {

    fn top_clues_display_height(&self) -> u16 {
        get_max_clue_size(&self.cols) as u16
    }

    fn top_clues_display_width(&self) -> u16 {
        (self.cols.len() * EMPTY_STR_HORIZONTAL.len()) as u16
    }

    fn left_clues_display_height(&self) -> u16 {
        self.rows.len() as u16
    }

    fn left_clues_display_width(&self) -> u16 {
        (get_max_clue_size(&self.rows) * EMPTY_STR_VERTICAL.len()) as u16
    }

    fn display_top_clues(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        let  max_size =  get_max_clue_size(&self.cols);
        
        for i in (0..max_size).rev() {
            queue!(stdout, cursor::MoveRight(context.left_offset)).ok();
            for j in 0..self.cols.len() {
                let height = self.cols[j].clues.len();
                if (height == 0) && i == 0 {
                    queue!(stdout, style::Print( format!("{:>2}", 0))).ok();
                } else if i > (height-1) {
                        queue!(stdout, style::Print( EMPTY_STR_HORIZONTAL )).ok();
                } else {
                        queue!(stdout, style::Print( format!("{:>2}", self.cols[j].clues[height - 1 - i]))).ok();
                }
            }
    
            self.go_to_next_line(context);
        }
    }

    fn display_left_clues(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        let max_size = get_max_clue_size(&self.rows);
    
        for i in 0..self.rows.len() {
            let empty = max_size - self.rows[i].clues.len();
    
            if empty == max_size {
                for _j in 0..(empty-1) {
                    queue!(stdout, style::Print( EMPTY_STR_VERTICAL )).ok();
                }
    
                queue!(stdout, style::Print( format!("{:>3}", 0))).ok();
    
            } else {
                for _j in 0..empty {
                    queue!(stdout, style::Print( EMPTY_STR_VERTICAL )).ok();
                }
        
                for j in 0..self.rows[i].clues.len(){
                    queue!(stdout, style::Print( format!("{:>3}", self.rows[i].clues[j]))).ok();
                }
            }
    
            self.go_to_next_line(context);
        }
    }

    fn display_image(&self, context : & DisplayContext){
        let mut stdout = stdout();
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].values.len() {
                match self.rows[y].values[x] {
                    CaseState::UNKNOWN => queue!(stdout, SetForegroundColor(UNKNOWN_COLOR), style::PrintStyledContent( UNKNOWN_STR.grey() ) , ResetColor).ok(),
                    CaseState::ON => queue!(stdout, SetForegroundColor(ON_COLOR), style::PrintStyledContent( ON_STR.black() ), ResetColor).ok(),
                    CaseState::OFF => queue!(stdout, SetForegroundColor(OFF_COLOR), style::PrintStyledContent( OFF_STR.white() ), ResetColor).ok(),
                };
            }
            self.go_to_next_line(context);
            queue!(stdout, cursor::MoveRight(context.left_offset)).ok();
        }
    }
}


impl Displayable for RowColPicross {
    fn display_in_context(&self, context : & DisplayContext) {
        let mut stdout = stdout();
        let mut local_context = context.clone();
    
        //Draw top
        local_context.left_offset = self.left_clues_display_width();
        self.display_top_clues(&local_context);
    
        //Draw left then prepare cursor for image
        local_context.left_offset = 0;
        self.display_left_clues(&local_context);
        let up_offset = self.left_clues_display_height();
        let right_offset = self.left_clues_display_width();
        queue!(stdout, cursor::MoveUp(up_offset), cursor::MoveRight(right_offset)).ok();
    
        //Draw image
        local_context.draw_inplace = true;
        local_context.left_offset = self.left_clues_display_width();
        self.display_image(&local_context);

        //Back to line
        self.go_to_next_line(context);
    }

    fn display_width(&self) -> u16 {
        self.left_clues_display_width() + self.top_clues_display_width()
    }

    fn display_height(&self) -> u16 {
        self.top_clues_display_height() + self.left_clues_display_height()
    }
}