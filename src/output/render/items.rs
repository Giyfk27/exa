use ansi_term::Style;

use output::cell::TextCell;
use fs::fields as f;


impl f::Items {
    pub fn render(&self, style: Style) -> TextCell {
        match *self {
            f::Items::Some(items) => return TextCell::paint(style, items.to_string()),
            f::Items::None        => return TextCell::blank(style),
        }
    }
}