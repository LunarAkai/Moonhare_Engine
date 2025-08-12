#[derive(Debug)]
pub struct RenderSystem;

impl Default for RenderSystem {
    fn default() -> Self {
        Self {}
    }
}
impl RenderSystem {
    pub(crate) fn update(&self) {}
}
