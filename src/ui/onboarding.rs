use std::{cell::RefCell, error::Error, path::PathBuf, str::FromStr};

use adw::{
    ViewStack,
    gtk::{
        Align, Box, Button, Entry, EntryBuffer, InputPurpose, Label, Orientation,
        prelude::{BoxExt, ButtonExt, EntryBufferExtManual, WidgetExt},
    },
};

use crate::preferences::Preferences;

pub fn page(preferences: RefCell<Option<Preferences>>, view_stack: ViewStack) -> Box {
    let title = Label::new(Some("Welcome to MBMM"));
    title.add_css_class("title-1");

    let subtitle = Label::new(Some("Please select you game's mod folder"));
    subtitle.add_css_class("title-2");

    let folder_buffer = EntryBuffer::new(None::<String>);
    let folder_entry = Entry::builder()
        .placeholder_text("Mods folder")
        .input_purpose(InputPurpose::Terminal)
        .buffer(&folder_buffer)
        .build();

    let folder_button = Button::builder()
        .label("Browse")
        .css_classes(["suggested-action"])
        .build();
    let folder_buffer_clone = folder_buffer.clone();
    folder_button.connect_clicked(move |_| pick_mod_folder(&folder_buffer_clone));

    let folder_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(4)
        .halign(Align::Center)
        .build();
    folder_box.append(&folder_entry);
    folder_box.append(&folder_button);

    let save_button = Button::builder()
        .label("Save")
        .css_classes(["suggested-action"])
        .build();
    let folder_buffer_clone = folder_buffer.clone();
    save_button.connect_clicked(move |_| {
        let res = save_preferences(preferences.clone(), &folder_buffer_clone.text());
        if let Err(e) = res {
            eprintln!("{:?}", e);
            return;
        }

        view_stack.set_visible_child_name("manage");
    });

    let content = Box::new(Orientation::Vertical, 0);
    content.set_valign(Align::Center);
    content.set_spacing(16);
    content.append(&title);
    content.append(&subtitle);
    content.append(&folder_box);
    content.append(&save_button);

    content
}

fn pick_mod_folder(folder_buffer: &EntryBuffer) {
    let path = rfd::FileDialog::new()
        .set_title("Select mod folder")
        .pick_folder();
    if path.is_none() {
        return;
    }

    folder_buffer.set_text(path.unwrap().to_str().unwrap_or(""));
}

fn save_preferences(
    mut preferences: RefCell<Option<Preferences>>,
    path: &str,
) -> Result<(), std::boxed::Box<dyn Error>> {
    let path_buf = PathBuf::from_str(path)?;
    if !path_buf.is_dir() {
        return Err("Folder doesn't exist".into());
    }

    let preferences = preferences.get_mut();
    let new_preferences = Some(Preferences { game_dir: path_buf });
    new_preferences.clone().unwrap().save().unwrap();
    *preferences = new_preferences;
    Ok(())
}
