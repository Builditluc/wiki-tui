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
        Self {
            links: Vec::new(),
            current_link: 0,
        }
    }

    /// Updates the links while trying to save the currently selected link
    pub fn update(&mut self, other: Option<LinkHandler>) {
        if other.is_none() {
            return;
        }

        let other = other.unwrap();

        if let Some(current_link_id) = self.get_current_link() {
            self.links = other.links;
            self.set_current_link(current_link_id);
        } else {
            self.links = other.links;
            self.current_link = other.current_link;
        }
    }

    /// Returns the total number of registered links
    pub fn registered_links(&self) -> usize {
        self.links.len()
    }

    /// Adds a new link with the given id and position
    /// It is required to add the links from left to right and top to bottom in order for the
    /// selection to work
    pub fn push_link(&mut self, id: usize, x: usize, y: usize) {
        self.links.push(Link { id, x, y })
    }

    /// Retrieves the id of the currently selected link. If there are no links, None will be returned
    pub fn get_current_link(&self) -> Option<usize> {
        if self.links.is_empty() {
            return None;
        }
        Some(self.links[self.current_link].id)
    }

    /// Returns the position of the currently selected link. If there are no links, None will be returned
    pub fn get_current_link_pos(&self) -> Option<Vec2> {
        if self.links.is_empty() {
            return None;
        }
        let link = &self.links[self.current_link];
        Some(Vec2::new(link.x, link.y))
    }

    /// Moves the selection up by a given amount
    pub fn move_up(&mut self, amount: usize) {
        if self.links.is_empty() {
            warn!("no links are registered, abort moving up by '{}'", amount);
            return;
        }

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
        if self.links.is_empty() {
            warn!("no links are registered, abort moving down by '{}'", amount);
            return;
        }

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
        if self.links.is_empty() {
            warn!("no links are registered, abort moving left by '{}'", amount);
            return;
        }

        let current_link = &self.links[self.current_link];
        debug!(
            "current link: {:?}, index: {}, total len: {}",
            current_link,
            self.current_link,
            self.links.len()
        );

        if let Some(link) = self
            .links
            .iter()
            .rev()
            .skip(
                amount
                    + (self
                        .links
                        .len()
                        .saturating_sub(1)
                        .saturating_sub(self.current_link)),
            )
            .find(|&x| x.id < current_link.id)
        {
            self.current_link = self.links.iter().position(|x| x == link).unwrap();
            debug!("new link: {:?}", self.links[self.current_link]);
        }
    }

    /// Moves the selection right by a given amount
    pub fn move_right(&mut self, amount: usize) {
        if self.links.is_empty() {
            warn!(
                "no links are registered, abort moving right by '{}'",
                amount
            );
            return;
        }

        let current_link = &self.links[self.current_link];
        debug!("current link: {:?}", current_link);

        if let Some(link) = self
            .links
            .iter()
            .skip(self.current_link + amount)
            .find(|&x| x.id > current_link.id)
        {
            self.current_link = self.links.iter().position(|x| x == link).unwrap();
            debug!("new link: {:?}", self.links[self.current_link]);
        }
    }

    /// Overrides the current link
    pub fn set_current_link(&mut self, id: usize) {
        if self.links.is_empty() {
            warn!(
                "no links are registered, abort setting the current link to '{}'",
                id
            );
            return;
        }

        let new_selection = self
            .links
            .iter()
            .position(|l| l.id == id)
            .unwrap_or_default();
        debug!(
            "replacing the current link '{}', with '{}'",
            self.current_link, new_selection
        );
        self.current_link = new_selection;
    }
}

/// A struct representing a Link. It contains an id to reference it to an ArticleElement and
/// relative x and y coordinates
#[derive(Debug, PartialEq)]
struct Link {
    /// The id of the Link. This is also used to reference it to an ArticleElement
    id: usize,

    /// The relative x coordinate of the Link
    x: usize,
    /// The relative y coordinate of the Link
    y: usize,
}
