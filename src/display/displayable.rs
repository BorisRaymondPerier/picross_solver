use super::*;

use std::io::{stdout, Write};
use crossterm::{ queue, style, cursor};

pub trait Displayable {
    fn display_in_context(&self, context : & DisplayContext);
    fn display_width(&self) -> u16;
    fn display_height(&self) -> u16;

    fn display(&self) {
        let context : DisplayContext = DisplayContext::new(false);
        self.display_in_context(&context);
    }

    fn go_to_next_line(&self, context: & DisplayContext) {
        let mut stdout = stdout();
        if context.draw_inplace {
            queue!(stdout, cursor::MoveDown(1), cursor::MoveLeft(self.display_width())).ok();
        } else { 
            queue!(stdout, style::Print("\n")).ok();
        }
    }
}