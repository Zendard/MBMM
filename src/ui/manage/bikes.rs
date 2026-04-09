use crate::{manage::bikes, preferences::Preferences};
use adw::{
    ButtonRow,
    gtk::{
        Box, Label, ListBox, Orientation, SelectionMode,
        prelude::{BoxExt, WidgetExt},
    },
};
use std::cell::RefCell;

pub fn page(preferences: RefCell<Option<Preferences>>) -> Box {
    let title = Label::new(Some("Bikes"));
    title.add_css_class("title-1");

    let bikes_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list-separate"])
        .vexpand(true)
        .margin_top(8)
        .margin_end(16)
        .margin_start(16)
        .build();

    let bikes_vec = bikes::get_bikes_list(preferences);
    if bikes_vec.is_err() {
        eprintln!("{:?}", bikes_vec.unwrap_err());
        return Box::new(Orientation::Vertical, 0);
    }

    for bike in bikes_vec.unwrap() {
        let row = ButtonRow::builder().title(bike).build();
        bikes_list.append(&row);
    }

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&title);
    content.append(&bikes_list);
    content
}
