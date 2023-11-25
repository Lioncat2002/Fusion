use gtk::{prelude::*, Button, CssProvider, Window, WindowType};
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

    let close_button = Button::new();
    close_button.set_size_request(20, 20);
    close_button
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    close_button.set_widget_name("close_btn");

    close_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));

    let maximize_button = Button::new();
    maximize_button.set_size_request(20, 20);
    maximize_button
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    maximize_button.set_widget_name("maximize_btn");

    maximize_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.maximize();
    }));

    let minimize_button = Button::new();
    minimize_button.set_size_request(20, 20);
    minimize_button
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    minimize_button.set_widget_name("minimize_btn");

    minimize_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.iconify();
    }));

    let window_control_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    window_control_container.pack_start(&close_button, false, false, 0);
    window_control_container.pack_start(&maximize_button, false, false, 0);
    window_control_container.pack_start(&minimize_button, false, false, 0);

    let searchbar = gtk::Entry::new();
    searchbar.set_size_request(50, 5);
    let titlebar_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    titlebar_container.pack_start(&window_control_container, false, false, 0);
    titlebar_container.pack_end(&searchbar, false, false, 5);
    titlebar_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    titlebar_container.set_widget_name("titlebar");

    let navigator_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    navigator_container.set_size_request(240, 720);
    navigator_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    navigator_container.set_widget_name("navbar");

    let global_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let view_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    view_container.pack_start(&navigator_container, false, false, 0);
    view_container.pack_start(&browser_container, true, true, 0);
    view_container
        .style_context()
        .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    view_container.set_widget_name("view_container");

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
