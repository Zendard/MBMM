use crate::preferences::Preferences;
use adw::{
    HeaderBar, ToolbarView, ViewStack, ViewSwitcher, ViewSwitcherPolicy,
    gtk::{Box, prelude::BoxExt},
};

mod browse;
mod manage;
mod onboarding;

pub fn content(preferences: Option<Preferences>) -> Box {
    let content = Box::new(adw::gtk::Orientation::Vertical, 0);

    let view_stack = ViewStack::builder().build();
    view_stack.add_titled_with_icon(&manage::page(), Some("manage"), "Manage", "folder-symbolic");
    view_stack.add_titled_with_icon(
        &browse::page(),
        Some("browse"),
        "Browse",
        "system-search-symbolic",
    );
    view_stack.add_named(&onboarding::page(), Some("onboarding"));

    let view_switcher = ViewSwitcher::builder()
        .stack(&view_stack)
        .policy(ViewSwitcherPolicy::Wide)
        .build();

    let header_bar = HeaderBar::builder().title_widget(&view_switcher).build();

    let toolbar_view = ToolbarView::new();
    toolbar_view.set_content(Some(&view_stack));
    toolbar_view.add_top_bar(&header_bar);

    content.append(&toolbar_view);

    if preferences.is_none() {
        view_stack.set_visible_child_name("onboarding");
    }

    content
}
