use gtk::{prelude::*, Application, ApplicationWindow, Button};

mod model;
use crate::model::counter::Counter;

fn on_activate(app: &Application) {
    let app_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let add_button = Button::builder().label("add counter").build();
    let new_counter = gtk::Entry::new();
    
    app_box.append(&new_counter);

    let copied = app_box.clone();
    add_button.connect_clicked(move |_| {
        let a = new_counter.text();
        let new_counter = Counter::new(a.as_str());
        let new_label = new_counter.counter_widget();
        copied.prepend(&new_label);
    });

    app_box.append(&add_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Counter App")
        .child(&app_box)
        .build();

    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
