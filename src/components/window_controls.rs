use gtk::{prelude::*, Button, CssProvider, Window};

pub fn window_controls(css_provider: &CssProvider, window: &Window) -> gtk::Box {
    let close_button = Button::new();
    close_button.set_size_request(20, 20);
    close_button
        .style_context()
        .add_provider(css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    close_button.set_widget_name("close_btn");

    close_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));
    let maximize_button = Button::new();
    maximize_button.set_size_request(20, 20);
    maximize_button
        .style_context()
        .add_provider(css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    maximize_button.set_widget_name("maximize_btn");

    maximize_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.maximize();
    }));

    let minimize_button = Button::new();
    minimize_button.set_size_request(20, 20);
    minimize_button
        .style_context()
        .add_provider(css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    minimize_button.set_widget_name("minimize_btn");

    minimize_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.iconify();
    }));

    let window_control_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    window_control_container.pack_start(&close_button, false, false, 0);
    window_control_container.pack_start(&maximize_button, false, false, 0);
    window_control_container.pack_start(&minimize_button, false, false, 0);
    window_control_container
        .style_context()
        .add_provider(css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    window_control_container.set_widget_name("window_control");
    window_control_container
}
