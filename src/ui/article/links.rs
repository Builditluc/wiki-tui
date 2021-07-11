use cursive::vec::Vec2;

pub struct LinkHandler {
    pub links: Vec<Link>,
    pub current_link: usize,

    pub on_link_submit_callback: cursive::event::Callback,
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

            on_link_submit_callback: cursive::event::Callback::dummy(),
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
        let new_current_link = (self.current_link as i32) + direction;
        if new_current_link > 0 {
            self.current_link = new_current_link as usize
        }
    }
    fn next_link_vertical(&mut self, direction: i32) {
        // TODO add something here

        //  go through the links until the porgram finds one that is one (or more) lines below the
        //  current one

        if direction > 0 {
            let current_position = self.links[self.current_link].position;
            for (idx, link) in self.links[self.current_link..].into_iter().enumerate() {
                if link.position.y > current_position.y {
                    self.current_link += idx;
                    return;
                }
            }
        } else {
            let current_position = self.links[self.current_link].position;
            for i in self.current_link..0 {
                if self.links[i].position.y < current_position.y {
                    self.current_link = i;
                    return;
                }
            }
        }
    }
}

pub enum Directions {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
