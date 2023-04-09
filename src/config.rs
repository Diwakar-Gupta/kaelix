pub struct General {
    pub line_number_padding_right: usize,
    pub line_number_padding_left: usize,
    pub file_tree_width: usize,
}

impl General{
    pub fn new() -> Self {
        Self {
            line_number_padding_left: 2,
            line_number_padding_right: 3,
            file_tree_width: 18,
        }
    }
}

pub struct Config{
    general: General,
}

impl Config{
    pub fn new() -> Self{
        Config { general: General::new() }
    }
}