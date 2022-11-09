use dirs;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Button, Label, Orientation};
mod system;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label(&format!("Disk Usage Overview"))
        .build();

    let button = Button::builder()
        .label("Click here")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    let mut disk_usage = system::get_disk_usage();

    button.connect_clicked(move |button| {
        button.set_label(&disk_usage);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Disk Viewer and Analyzer")
        .child(&gtk_box)
        .build();

    window.present();
}
