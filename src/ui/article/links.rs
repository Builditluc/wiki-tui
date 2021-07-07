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

    pub fn move_current_link(&mut self, direction: Directions) {
        match direction {
            Directions::LEFT => self.next_link_horizontal(-1),
            Directions::RIGHT => self.next_link_horizontal(1),

            Directions::UP => self.next_link_vertical(-1),
            Directions::DOWN => self.next_link_vertical(1),
        }
    }

    pub fn select_current_link(&mut self, link_index: usize) -> bool {
        // TODO add a check to identify a false link index
        self.current_link = link_index;
        true
    }

    fn next_link_horizontal(&mut self, direction: i32) {
        log::info!(
            "[ui::article::links::LinkHandler::new_link_horizontal] Moving the current link by {}",
            direction
        );
        let new_current_link = (self.current_link as i32) + direction;
        if new_current_link > 0 {
            self.current_link = new_current_link as usize;
            log::info!("[ui::article::links::LinkHandler::new_link_horizontal] the new link position is {}", self.current_link);
            return;
        }

        log::warn!("[ui::article::links::LinkHandler::new_link_horizontal] New link position is smaller than 0, aborting");
    }
    fn next_link_vertical(&self, direction: i32) {
        // TODO add something here
    }
}

pub enum Directions {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
