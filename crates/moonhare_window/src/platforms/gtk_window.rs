use gtk4::{gio::{prelude::{ActionMapExtManual, ApplicationExt}, ActionEntry}, glib, prelude::{GtkWindowExt, WidgetExt}, Application, ApplicationWindow};
use moonhare_event::{event::Event, events::window_events::window_close_event::WindowCloseEvent};

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

        // Add action "close" to `window` taking no parameter
        let action_close = ActionEntry::builder("close")
            .activate(|window: &ApplicationWindow, _, _| {
                GTKWindow::shutdown();
                window.close();
            })
            .build();

        window.add_action_entries([action_close]);

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
        // todo: emit WindowCloseEvent
        
    }
}

