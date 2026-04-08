use adw::{
    HeaderBar, ToolbarView, ViewStack, ViewSwitcher, ViewSwitcherPolicy,
    gtk::{Box, prelude::BoxExt},
};

pub mod browse;
pub mod manage;

pub fn content() -> Box {
    let content = Box::new(adw::gtk::Orientation::Vertical, 0);

    let view_stack = ViewStack::builder().build();
    view_stack.add_titled_with_icon(&manage::page(), Some("manage"), "Manage", "folder-symbolic");
    view_stack.add_titled_with_icon(
        &browse::page(),
        Some("browse"),
        "Browse",
        "system-search-symbolic",
    );

    let view_switcher = ViewSwitcher::builder()
        .stack(&view_stack)
        .policy(ViewSwitcherPolicy::Wide)
        .build();

    let header_bar = HeaderBar::builder().title_widget(&view_switcher).build();

    let toolbar_view = ToolbarView::new();
    toolbar_view.set_content(Some(&view_stack));
    toolbar_view.add_top_bar(&header_bar);

    content.append(&toolbar_view);

    content
}
