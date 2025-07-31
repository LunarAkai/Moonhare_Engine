/// General Config for [`WinitWindow`](crate::winit::winit_window::WinitWindow)
#[derive(Debug)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub visble: bool,
    pub decorations: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self { 
            title: "Moonhare Engine".to_owned(), 
            width: 1280, 
            height: 720,
            visble: default_visibility(),
            decorations: default_decorations(),
        }
    }
}

// Todo: Set Functions should be inside WinitWindow i guess?
impl WindowConfig {
    fn set_window_name(mut self, name: String) {
        self.title = name;
    }
    
    fn set_window_height(mut self, new_height: u32) {
        self.height = new_height;
    }

    fn set_window_width(mut self, new_width: u32) {
        self.width = new_width;
    }

    fn set_window_visible(mut self, visible: bool) {
        self.visble = visible;
    }

    fn set_window_decoration(mut self, decoration: bool) {
        self.decorations = decoration;
    }
}



fn default_visibility() -> bool {
    true
}

fn default_decorations() -> bool {
    true
}