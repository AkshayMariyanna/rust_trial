mod encapsulation;
mod trait_object;
mod state_pattern;
mod state_pattern_redef;

pub use self::trait_object::gui;
pub use self::encapsulation::avg;
pub use self::state_pattern::blog;
pub use self::state_pattern_redef::blog2;

use self::gui::Draw;

pub struct TextBlock {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for TextBlock {
    fn draw(&self) {
        println!("Drawing a TextBlock");
    }
}
