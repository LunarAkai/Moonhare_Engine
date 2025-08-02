use gtk4::{gio::prelude::ApplicationExt, prelude::{GtkWindowExt, WidgetExt}, Application, ApplicationWindow};

use crate::{window_config, MoonhareWindow};

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
        let window_config = window_config::WindowConfig::default();
        window.set_title(Some(format!("{} GTK", window_config.title).as_str()));
        window.set_default_size(window_config.width as i32, window_config.height as i32);
        window.set_visible(window_config.visble);

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

