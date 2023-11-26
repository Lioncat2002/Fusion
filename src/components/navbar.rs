use gtk::prelude::*;
use gtk::CssProvider;
pub fn navbar(css_provider: &CssProvider) -> gtk::Box {
    let navigator_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    navigator_container.set_size_request(240, 720);
    navigator_container
        .style_context()
        .add_provider(css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    navigator_container.set_widget_name("navbar");

    navigator_container
}
