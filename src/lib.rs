extern crate enum_iterator;
extern crate int_enum;

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Copy, Clone)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {

    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => String::from("value out of range")
    }

}

pub fn colors() -> Vec<ResistorColor> {

    let mut colors = Vec::new();

    for f in ResistorColor::into_enum_iter() {
        colors.push(f);
    }

    colors.sort_by(|a, b| a.int_value().cmp(&b.int_value()));

    colors
}
