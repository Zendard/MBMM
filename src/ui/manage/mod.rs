use crate::preferences::Preferences;
use adw::{
    ButtonRow, OverlaySplitView,
    gtk::{
        Box, Label, ListBox, Orientation, SelectionMode,
        prelude::{BoxExt, WidgetExt},
    },
};
use std::cell::RefCell;

mod bikes;
mod misc;
mod paints;
mod rider;
mod tracks;
mod tyres;

pub fn page(preferences: RefCell<Option<Preferences>>) -> Box {
    let split_view = OverlaySplitView::new();

    let title = Label::new(Some("Manage mods"));
    title.add_css_class("title-1");

    let bikes_row = ButtonRow::builder().title("Bikes").build();
    let split_view_clone = split_view.clone();
    bikes_row.connect_activated(move |_| {
        split_view_clone.set_content(Some(&bikes::page(preferences.clone())))
    });

    let misc_row = ButtonRow::builder().title("Misc").build();
    let split_view_clone = split_view.clone();
    misc_row.connect_activated(move |_| split_view_clone.set_content(Some(&misc::page())));

    let paints_row = ButtonRow::builder().title("Paints").build();
    let split_view_clone = split_view.clone();
    paints_row.connect_activated(move |_| split_view_clone.set_content(Some(&paints::page())));

    let rider_row = ButtonRow::builder().title("Rider").build();
    let split_view_clone = split_view.clone();
    rider_row.connect_activated(move |_| split_view_clone.set_content(Some(&rider::page())));

    let tracks_row = ButtonRow::builder().title("Tracks").build();
    let split_view_clone = split_view.clone();
    tracks_row.connect_activated(move |_| split_view_clone.set_content(Some(&tracks::page())));

    let tyres_row = ButtonRow::builder().title("Tyres").build();
    let split_view_clone = split_view.clone();
    tyres_row.connect_activated(move |_| split_view_clone.set_content(Some(&tyres::page())));

    let sidebar = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list-separate"])
        .vexpand(true)
        .margin_top(8)
        .margin_end(16)
        .margin_start(16)
        .build();

    sidebar.append(&bikes_row);
    sidebar.append(&misc_row);
    sidebar.append(&paints_row);
    sidebar.append(&rider_row);
    sidebar.append(&tracks_row);
    sidebar.append(&tyres_row);

    split_view.set_sidebar(Some(&sidebar));
    split_view.set_content(Some(&title));

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&split_view);
    content
}
