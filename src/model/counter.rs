use gtk::{prelude::*, Box, Button};
use std::{cell::Cell, rc::Rc};

pub struct Counter {
    name: String,
    value: Cell<i32>,
}

impl Counter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: Cell::new(0),
        }
    }

    pub fn counter_widget(&self) -> Box {
        let counter_box = Box::new(gtk::Orientation::Vertical, 2);

        let counter = Rc::new(self.value.clone());

        let component = Rc::new(CounterComponent::new(&self));

        {
            let counter = counter.clone();
            let label = component.value.clone();
            component.increase.connect_clicked(move |_| {
                counter.set(counter.get() + 1);
                label.set_label(&counter.get().to_string());
            });
        }
        {
            let counter = counter.clone();
            let label = component.value.clone();
            component.decrease.connect_clicked(move |_| {
                counter.set(counter.get() - 1);
                label.set_label(&counter.get().to_string());
            });
        }
        counter_box.append(&component.header);
        counter_box.append(&component.counter_body());
        counter_box
    }
}

struct CounterComponent {
    header: gtk::Box,
    increase: gtk::Button,
    decrease: gtk::Button,
    value: gtk::Label,
}

impl CounterComponent {
    pub fn new(counter: &Counter) -> Self {
        Self {
            header: CounterComponent::counter_header(&counter.name),
            increase: CounterComponent::increase_button(),
            decrease: CounterComponent::decrease_button(),
            value: gtk::Label::new(Some(&counter.value.get().to_string())),
        }
    }

    pub fn counter_body(&self) -> gtk::Box {
        let body = gtk::Box::new(gtk::Orientation::Horizontal, 7);
        body.append(&empty_space(gtk::Orientation::Horizontal));
        body.append(&self.decrease);
        body.append(&empty_space(gtk::Orientation::Horizontal));
        body.append(&self.value);
        body.append(&empty_space(gtk::Orientation::Horizontal));
        body.append(&self.increase);
        body.append(&empty_space(gtk::Orientation::Horizontal));
        body
    }

    fn counter_header(name: &str) -> gtk::Box {
        let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 3);
        header_box.append(&empty_space(gtk::Orientation::Horizontal));
        header_box.append(&gtk::Label::new(Some(name)));
        header_box.append(&empty_space(gtk::Orientation::Horizontal));
        header_box
    }

    fn increase_button() -> gtk::Button {
        let bt = Button::builder().label("+").build();
        bt
    }
    fn decrease_button() -> gtk::Button {
        let bt = Button::builder().label("-").build();
        bt
    }
}

fn empty_space(orientation: gtk::Orientation) -> gtk::Box {
    let space = Box::new(orientation, 0);
    if orientation == gtk::Orientation::Horizontal {
        space.set_hexpand(true);
        return space;
    }
    space.set_vexpand(true);
    space
}
