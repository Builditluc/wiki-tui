use cursive::vec::Vec2;

pub struct LinkHandler {
    pub links: Vec<Link>,
    pub current_link: usize,
}

pub struct Link {
    pub position: Vec2,
    pub width: usize,
    pub destination: String,
}

pub struct Element;

impl LinkHandler {
    pub fn new() -> LinkHandler {
        LinkHandler {
            links: Vec::new(),
            current_link: 0,
        }
    }

    pub fn push(&mut self, link: Link) -> usize {
        self.links.push(link);
        self.links.len() - 1
    }
}
