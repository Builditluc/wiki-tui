use cursive::Vec2;

/// A struct handling link selection
pub struct LinkHandler {
    /// An array of registered links
    links: Vec<Link>,

    /// The index of the current link
    current_link: usize,
}

impl LinkHandler {
    /// Creates a new link handler
    pub fn new() -> Self {
        log::debug!("creating a new instance of LinkHandler");
        Self {
            links: Vec::new(),
            current_link: 0,
        }
    }

    /// Returns the total number of registered links
    pub fn registered_links(&self) -> usize {
        self.links.len()
    }

    /// Adds a new link with the given id and position
    /// It is required to add the links from left to right and top to bottom in order for the
    /// selection to work
    pub fn push_link(&mut self, id: i32, x: usize, y: usize) {
        self.links.push(Link { id, x, y })
    }

    /// Retrieves the id of the currently selected link
    pub fn get_current_link(&self) -> i32 {
        assert!(self.links.len() > self.current_link);
        self.links[self.current_link].id
    }

    /// Returns the position of the currently selected link
    pub fn get_current_link_pos(&self) -> Vec2 {
        assert!(self.links.len() > self.current_link);
        let link = &self.links[self.current_link];
        Vec2::new(link.x, link.y)
    }

    /// Moves the selection up by a given amount
    pub fn move_up(&mut self, amount: usize) {
        assert!(self.links.len() > self.current_link);

        // save the minimum y-position
        let min_y = self.links[self.current_link].y.saturating_sub(amount);

        // go through every link above the current one
        for i in (0..self.current_link).rev() {
            // if the link has the right y-position, save it as the new current link and return
            if self.links[i].y <= min_y {
                self.current_link = i;
                return;
            }
        }

        // if we can't move the link further up, just select the first one
        self.current_link = 0;
    }

    /// Moves the selection down by a given amount
    pub fn move_down(&mut self, amount: usize) {
        // save the minimum y-position
        let min_y = self.links[self.current_link].y.saturating_add(amount);

        // go through every link below the current one
        for i in self.current_link..self.links.len() {
            // if the link has the right y-position, save it as the new current link and return
            if self.links[i].y >= min_y {
                self.current_link = i;
                return;
            }
        }

        // if we can't move the link further down, just select the last one
        self.current_link = self.links.len().saturating_sub(1);
    }

    /// Moves the selection left by a given amount
    pub fn move_left(&mut self, amount: usize) {
        self.current_link = self.current_link.saturating_sub(amount);
    }

    /// Moves the selection right by a given amount
    pub fn move_right(&mut self, amount: usize) {
        // if we don't have enough links on the right, just select the last one
        if self.current_link + amount >= self.links.len() {
            self.current_link = self.links.len().saturating_sub(1);
            return;
        }

        self.current_link += amount
    }
}

/// A struct representing a Link. It contains an id to reference it to an ArticleElement and
/// relative x and y coordinates
struct Link {
    /// The id of the Link. This is also used to reference it to an ArticleElement
    id: i32,

    /// The relative x coordinate of the Link
    x: usize,
    /// The relative y coordinate of the Link
    y: usize,
}
