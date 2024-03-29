use gtk::{prelude::*, Application, ApplicationWindow, Box, Button, Label, glib};
use std::{cell::Cell, rc::Rc};

#[derive(Debug)]
pub struct Counter {
    name: String,
    value: i32,
    child: Vec<Self>,
}

impl Counter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: 0,
            child: Vec::new(),
        }
    }

    pub fn counter_widget(&self) -> Box {
        let counter_box = Box::builder().build();
        let increase_button = Button::builder().label("increase").build();
        let decrease_button = Button::builder().label("decrease").build();

        let copied = Rc::new(Cell::new(self.value));
        let counter_label = Label::builder().label(self.value.to_string()).build();
        let counter_name_label = Label::builder().label(self.name.to_string()).build();

        decrease_button.connect_clicked(glib::clone!(@strong copied, @weak counter_label => move |_| {
            copied.set(copied.get() - 1);
            counter_label.set_label(&copied.get().to_string());
        }));

        increase_button.connect_clicked(glib::clone!(@strong copied, @weak counter_label => move |_| {
            copied.set(copied.get() + 1);
            counter_label.set_label(&copied.get().to_string());
        }));

        counter_box.append(&counter_name_label);
        counter_box.append(&decrease_button);
        counter_box.append(&counter_label);
        counter_box.append(&increase_button);
        counter_box
    }
}