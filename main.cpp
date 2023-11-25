#include <gtkmm.h>
#include <webkit2/webkit2.h>
#include <iostream>

class MyWindow : public Gtk::Window {
public:
    MyWindow() {
        set_default_size(1280, 720);
        set_decorated(false);

        Glib::RefPtr<Gtk::CssProvider> cssProvider = Gtk::CssProvider::create();
        cssProvider->load_from_path("global.css");
        auto webView =  WEBKIT_WEB_VIEW( webkit_web_view_new() );
        auto view = Glib::wrap( GTK_WIDGET( webView ) );
        webkit_web_view_load_uri(webView,"https://www.google.com/");

        auto browserContainer = Gtk::manage(new Gtk::Box(Gtk::ORIENTATION_HORIZONTAL, 0));
        browserContainer->pack_start(*view, true, true, 12);
        browserContainer->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        browserContainer->set_name("browser_container");


        auto closeButton = Gtk::manage(new Gtk::Button());
        closeButton->set_size_request(20, 20);
        closeButton->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        closeButton->set_name("close_btn");
        closeButton->signal_clicked().connect(sigc::mem_fun(*this, &MyWindow::on_close_clicked));

        auto maximizeButton = Gtk::manage(new Gtk::Button());
        maximizeButton->set_size_request(20, 20);
        maximizeButton->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        maximizeButton->set_name("maximize_btn");
        maximizeButton->signal_clicked().connect(sigc::mem_fun(*this, &MyWindow::on_maximize_clicked));

        auto minimizeButton = Gtk::manage(new Gtk::Button());
        minimizeButton->set_size_request(20, 20);
        minimizeButton->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        minimizeButton->set_name("minimize_btn");
        minimizeButton->signal_clicked().connect(sigc::mem_fun(*this, &MyWindow::on_minimize_clicked));

        auto windowControlContainer = Gtk::manage(new Gtk::Box(Gtk::ORIENTATION_HORIZONTAL, 0));
        windowControlContainer->pack_start(*closeButton, false, false, 0);
        windowControlContainer->pack_start(*maximizeButton, false, false, 0);
        windowControlContainer->pack_start(*minimizeButton, false, false, 0);

        auto labelContainer = Gtk::manage(new Gtk::Box(Gtk::ORIENTATION_HORIZONTAL, 0));
        auto label = Gtk::manage(new Gtk::Label("Lioncat"));
        labelContainer->pack_start(*label, false, false, 0);

        auto navigatorContainer = Gtk::manage(new Gtk::Box(Gtk::ORIENTATION_VERTICAL, 0));
        navigatorContainer->set_size_request(250, 720);
        navigatorContainer->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        navigatorContainer->set_name("navbar");
        navigatorContainer->pack_start(*windowControlContainer, false, false, 0);
        navigatorContainer->pack_start(*labelContainer, false, false, 0);

        auto globalContainer = Gtk::manage(new Gtk::Box(Gtk::ORIENTATION_HORIZONTAL, 0));
        globalContainer->pack_start(*navigatorContainer, false, false, 0);
        globalContainer->pack_start(*browserContainer, true, true, 0);
        globalContainer->get_style_context()->add_provider(cssProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
        globalContainer->set_name("global_container");
        add(*globalContainer);

    /*    auto settings = webView->get_settings();
        settings->set_enable_developer_extras(true);
*/
        show_all();

        signal_delete_event().connect(sigc::mem_fun(*this, &MyWindow::on_delete_event));
    }

private:
    void on_close_clicked() {
        hide();
    }

    void on_maximize_clicked() {
        maximize();
    }

    void on_minimize_clicked() {
        iconify();
    }

    bool on_delete_event(GdkEventAny* event) {
        Gtk::Main::quit();
        return true;
    }
};

int main(int argc, char* argv[]) {
    auto app = Gtk::Application::create(argc, argv, "com.example.my_webkit_app");
    MyWindow window;
    return app->run(window);
}
