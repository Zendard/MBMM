use adw::gtk::{
    Box, Label, Orientation,
    prelude::{BoxExt, WidgetExt},
};

pub fn page() -> Box {
    let title = Label::new(Some("Rider"));
    title.add_css_class("title-1");

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&title);
    content
}
