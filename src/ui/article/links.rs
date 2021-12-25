use cursive::vec::Vec2;
use std::rc;

pub struct LinkHandler {
    pub links: Vec<Link>,
    pub current_link: usize,

    pub on_link_submit_callback: Option<rc::Rc<dyn Fn(&mut cursive::Cursive, &str) + 'static>>,
}

#[derive(Debug)]
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

            on_link_submit_callback: None,
        }
    }

    pub fn push(&mut self, link: Link) -> usize {
        self.links.push(link);
        self.links.len() - 1
    }

    pub fn move_link(&mut self, direction: Directions, amount: i32) -> usize {
        match direction {
            Directions::HORIZONTAL => self.move_horizontal(amount),
            Directions::VERTICAL => self.move_vertical(amount),
        }
    }

    fn move_vertical(&mut self, amount: i32) -> usize {
        log::debug!("Moving {} vertical", amount);
        if amount > 0 {
            let current_pos = self.links[self.current_link].position;
            for (idx, link) in self.links[self.current_link..].iter().enumerate() {
                log::debug!("Selecting {} {:?}", idx, link);
                if link.position.y >= current_pos.y + amount as usize {
                    log::debug!("Found the currect link");
                    self.current_link += idx;
                    break;
                }
            }
        } else {
            let current_pos = self.links[self.current_link].position;
            for (idx, link) in self.links[0..self.current_link].iter().enumerate().rev() {
                log::debug!("Selecting {} {:?}", idx, link);
                if link.position.y < current_pos.y.saturating_sub((0 - amount) as usize) {
                    self.current_link = idx;
                    log::debug!("Found the currect link");
                    break;
                }
            }
        }

        self.links[self.current_link].position.y
    }

    fn move_horizontal(&mut self, amount: i32) -> usize {
        let new_idx = (self.current_link as i32) + amount;
        if new_idx >= 0 {
            self.current_link = new_idx as usize;
        } else {
            self.current_link = 0;
        }

        self.links[self.current_link].position.y
    }

    pub fn reset(&mut self) {
        self.links.clear();
        self.current_link = 0;
    }
    pub fn has_links(&self) -> bool {
        !self.links.is_empty()
    }
}

pub enum Directions {
    HORIZONTAL,
    VERTICAL
}

impl Default for LinkHandler {
    fn default() -> Self {
        Self::new()
    }
}
