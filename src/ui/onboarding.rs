use std::cell::RefCell;

use adw::gtk::{
    Align, Box, Button, Label, Orientation,
    prelude::{BoxExt, ButtonExt, WidgetExt},
};

use crate::preferences::Preferences;

pub fn page(preferences: RefCell<Option<Preferences>>) -> Box {
    let title = Label::new(Some("Welcome to MBMM"));
    title.add_css_class("title-1");

    let subtitle = Label::new(Some("Please select you game's mod folder"));
    subtitle.add_css_class("title-2");

    let folder_button = Button::builder()
        .label("Browse")
        .halign(Align::Center)
        .css_classes(["suggested-action"])
        .build();
    folder_button.connect_clicked(move |_| pick_mod_folder(preferences.clone()));

    let content = Box::new(Orientation::Vertical, 0);
    content.set_valign(Align::Center);
    content.set_spacing(16);
    content.append(&title);
    content.append(&subtitle);
    content.append(&folder_button);

    content
}

fn pick_mod_folder(mut preferences: RefCell<Option<Preferences>>) {
    let path = rfd::FileDialog::new()
        .set_title("Select mod folder")
        .pick_folder()
        .unwrap();

    let preferences = preferences.get_mut();
    let new_preferences = Some(Preferences { game_dir: path });
    new_preferences.clone().unwrap().save().unwrap();
    *preferences = new_preferences;
}
