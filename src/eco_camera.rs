use crate::prelude::*;

pub struct EcoCamera {
    pub center: Point,
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl EcoCamera {
    pub fn new(center: Point) -> Self {
        Self {
            center,
            left_x: center.x - DISPLAY_WIDTH / 2,
            right_x: center.x + DISPLAY_WIDTH / 2,
            top_y: center.y - DISPLAY_HEIGHT / 2,
            bottom_y: center.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn center_on_camera_move(&mut self, offset: Point) {
        self.center = self.center + offset;
        self.left_x = self.center.x - DISPLAY_WIDTH / 2;
        self.right_x = self.center.x + DISPLAY_WIDTH / 2;
        self.top_y = self.center.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = self.center.y + DISPLAY_HEIGHT / 2;
    }
}
