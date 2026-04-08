use adw::gtk::{Box, Label, Orientation, prelude::BoxExt};

pub fn page() -> Box {
    let title = Label::new(Some("Welcome to MBMM"));
    let subtitle = Label::new(Some("Please select you game's mod folder"));

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&title);
    content.append(&subtitle);
    return content;
}
