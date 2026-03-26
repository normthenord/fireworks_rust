use macroquad::color::*;
use macroquad::rand::gen_range;

pub const COLORS_LIST: [Color; 10] = [
        RED, GREEN, BLUE, YELLOW, ORANGE,
        PINK, PURPLE, SKYBLUE, LIME, GOLD,
    ];


pub fn random_macroquad_color () -> Color {
    let idx = gen_range(0, COLORS_LIST.len());

    COLORS_LIST[idx]

}



pub fn random_color() -> Color {
    let red = gen_range(0.0, 1.0);
    let green = gen_range(0.0, 1.0);
    let blue = gen_range(0.0, 1.0);

    Color::new(red, green, blue, 1.0)
}
