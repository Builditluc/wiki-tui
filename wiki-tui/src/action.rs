#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Resume,
    Suspend,
    RenderTick,
    Resize(u16, u16),
    Noop,
}
