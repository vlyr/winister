use crate::core::Area;

pub enum Layout {
    Bsp,
    Winister,
    Stacking,
    Floating,
}

impl Layout {
    pub fn generate_window_sizes(
        &self,
        window_count: usize,
        idx: usize,
        x: u32,
        y: u32, 
        width: u32,
        height: u32,
        vertical: usize
    ) -> Vec<Area> {
        match window_count {
            0 => vec![],

            1 => vec![Area { x, y, width, height }],

            _ => {
                if idx % 2 == vertical {
                    let mut vec = vec![Area { x, y, width, height: height / 2 }];

                    vec.append(&mut self.generate_window_sizes(
                        window_count - 1,
                        idx + 1,
                        x,
                        y + height / 2,
                        width,
                        height / 2,
                        0
                    ));

                    vec
                } else {
                    let mut vec = vec![Area { x, y, width: width / 2, height }];

                    vec.append(&mut self.generate_window_sizes(
                        window_count - 1,
                        idx + 1,
                        x + width / 2,
                        y, 
                        width / 2,
                        height,
                        1
                    ));

                    vec
                }
            }
        }
    }
}
