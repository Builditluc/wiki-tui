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

    pub fn move_current_link(&mut self, direction: Directions) -> usize {
        match direction {
            Directions::LEFT => self.next_link_horizontal(-1),
            Directions::RIGHT => self.next_link_horizontal(1),

            Directions::UP => self.next_link_vertical(-1),
            Directions::DOWN => self.next_link_vertical(1),
        }
    }

    fn next_link_horizontal(&mut self, direction: i32) -> usize {
        let new_current_link = (self.current_link as i32) + direction;
        if new_current_link > 0 {
            self.current_link = new_current_link as usize
        }

        self.links[self.current_link].position.y
    }
    fn next_link_vertical(&mut self, direction: i32) -> usize {
        //  go through the links until the porgram finds one that is one (or more) lines below
        //  the current one

        if direction > 0 {
            let current_position = self.links[self.current_link].position;
            for (idx, link) in self.links[self.current_link..].iter().enumerate() {
                if link.position.y > current_position.y {
                    self.current_link += idx;
                    return self.links[self.current_link].position.y;
                }
            }
        } else if direction == -1 {
            let current_position = self.links[self.current_link].position;
            for i in (0..self.current_link).rev() {
                if self.links[i].position.y < current_position.y {
                    self.current_link = i;
                    return self.links[self.current_link].position.y;
                }
            }
        }

        0
    }

    pub fn reset(&mut self) {
        self.links.clear();
        self.current_link = 0;
    }
    pub fn has_links(&self) -> bool {
        self.links.len() != 0
    }
}

pub enum Directions {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Default for LinkHandler {
    fn default() -> Self {
        Self::new()
    }
}
