mod application;
mod components;

use gtk::{prelude::*, CssProvider, SearchBar, Window, WindowType};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_default_size(1280, 720);
    window.set_decorated(false);

    let css_provider = CssProvider::new();
    css_provider
        .load_from_path("global.css")
        .expect("Failed to load css from file");

    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    webview.load_uri("https://www.google.com/");

    let browser_container = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    browser_container.pack_start(&webview, true, true, 8);
    browser_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    browser_container.set_widget_name("browser_container");

    let window_control_container =
        components::window_controls::window_controls(&css_provider, &window);

    let search_bar = SearchBar::new();
    search_bar.set_size_request(750, 25);
    search_bar.set_search_mode(true);

    search_bar
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    search_bar.set_widget_name("search_bar");

    let titlebar_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    titlebar_container.pack_start(&window_control_container, false, false, 0);
    titlebar_container.pack_end(&search_bar, false, false, 6);
    titlebar_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    titlebar_container.set_widget_name("titlebar");

    let navigator_container = components::navbar::navbar(&css_provider);

    let view_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    view_container.pack_start(&navigator_container, false, false, 0);
    view_container.pack_start(&browser_container, true, true, 0);
    view_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    view_container.set_widget_name("view_container");

    let global_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    global_container.pack_start(&titlebar_container, false, false, 0);
    global_container.pack_start(&view_container, false, false, 0);

    window.add(&global_container);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Proceed
    });

    gtk::main();
}
