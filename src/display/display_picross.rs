use super::*;

use std::io::{stdout, Write};
use crossterm::{ queue, cursor };


impl Displayable for PicrossBoard {
    fn display_in_context(&self, context : & DisplayContext) {
        let mut stdout = stdout();

        let mut local_context = context.clone();
        
        //Draw top
        local_context.left_offset = self.left.display_width();
        self.top.display_in_context(&local_context);

        //Draw left
        local_context.left_offset = 0;
        self.left.display_in_context(&local_context);
    
        //Draw image
        queue!(stdout, cursor::MoveUp(self.left.display_height()), cursor::MoveRight(self.left.display_width())).ok();
        local_context.draw_inplace = true;
        self.image.display_in_context(&local_context);

        //Back to line
        self.go_to_next_line(context);
    }

    fn display_height(&self) -> u16 {
        self.top.display_height() + self.left.display_height()
    }
    
    fn display_width(&self) -> u16 {
        self.left.display_width() +self.top.display_width()
    }
}