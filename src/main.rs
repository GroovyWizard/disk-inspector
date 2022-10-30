use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Click here")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Disk");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Disk Viewer and Analyzer")
        .child(&button)
        .build();

    window.present();
}
