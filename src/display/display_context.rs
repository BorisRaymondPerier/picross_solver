#[derive(Copy, Clone)]
pub struct DisplayContext {
    pub draw_inplace: bool,
    pub left_offset: u16  
}

impl DisplayContext {
    pub fn new(draw_inplace: bool) -> Self {
        Self {
            draw_inplace,
            left_offset: 0
        }
    }

    pub fn new_with_offset(draw_inplace: bool, left_offset: u16) -> Self {
        Self {
            draw_inplace,
            left_offset,
        }
    }
}