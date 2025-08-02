use std::marker::PhantomData;

use gtk4::{gio::prelude::{ApplicationExt, ApplicationExtManual}, glib::object::ObjectExt, prelude::{GtkWindowExt, WidgetExt}, subclass::prelude::GtkApplicationImpl, Application, ApplicationWindow};

use crate::{MoonhareWindow, WindowResult};

#[derive(Debug)]
pub struct GTKWindow {
    application: Application,
}

const APP_ID: &str = "de.lunarakai.moonhare_engine";

impl GTKWindow {
    pub fn get_application(self) -> Application {
        self.application
    }

    fn build_ui(application: &Application) {
        let window = ApplicationWindow::new(application);
        window.set_title(Some("Moonhare Engine GTK"));
        window.set_default_size(1280, 720);

        window.show();

    }
}

impl MoonhareWindow for GTKWindow {  
    type WindowResult = GTKWindow; 
    fn init() -> Self::WindowResult {
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_activate(GTKWindow::build_ui);

        Self {
            application: app
        }
    }


    fn on_update() {
        
    }

    fn shutdown() {
    }
    

}

