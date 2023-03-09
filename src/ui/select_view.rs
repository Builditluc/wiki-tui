use std::{borrow::Borrow, cmp::Ordering, rc::Rc};

use cursive::{
    align::{Align, HAlign, VAlign},
    event::{Callback, Event, EventResult, Key},
    utils::markup::StyledString,
    view::ViewWrapper,
    views, Cursive, View,
};

use crate::config::CONFIG;

/// A Wrapper for the cursive::views::SelectView. Allows us to overwrite the keybindings
pub struct SelectView<T> {
    view: views::SelectView<T>,
}

impl<T: 'static> SelectView<T> {
    /// Creates a new empty SelectView
    pub fn new() -> Self {
        SelectView {
            view: views::SelectView::new(),
        }
    }

    /// Sets the "auto-jump" property for this view.
    ///
    /// If enabled, when a key is pressed, the selection will jump to the next
    /// item beginning with the pressed letter.
    pub fn set_autojump(&mut self, autojump: bool) {
        self.view.set_autojump(autojump)
    }

    /// Sets the "auto-jump" property for this view.
    ///
    /// If enabled, when a key is pressed, the selection will jump to the next
    /// item beginning with the pressed letter.
    ///
    /// Chainable variant.
    #[must_use]
    pub fn autojump(mut self) -> Self {
        self.view = self.view.autojump();
        self
    }

    /// Sets the "inactive highlight" property for this view.
    ///
    /// * If true (the default), the selected row will be highlighted when the
    ///   view is not focused.
    /// * If false, the selected row will be printed like the others if inactive.
    pub fn set_inactive_highlight(&mut self, inactive_highlight: bool) {
        self.view.set_inactive_highlight(inactive_highlight)
    }

    /// Sets the "inactive highlight" property for this view.
    ///
    /// * If true (the default), the selected row will be highlighted when the
    ///   view is not focused.
    /// * If false, the selected row will be printed like the others if inactive.
    ///
    /// Chainable variant.
    pub fn with_inactive_highlight(mut self, inactive_highlight: bool) -> Self {
        self.view.set_inactive_highlight(inactive_highlight);
        self
    }

    /// Returns the current status of the "inactive highlight" property.
    pub fn get_inactive_highlight(&self) -> bool {
        self.view.get_inactive_highlight()
    }

    /// Turns `self` into a popup select view.
    ///
    /// Chainable variant.
    #[must_use]
    pub fn popup(mut self) -> Self {
        self.view = self.view.popup();
        self
    }

    /// Turns `self` into a popup select view.
    pub fn set_popup(&mut self, popup: bool) {
        self.view.set_popup(popup)
    }

    /// Sets a callback to be used when an item is selected.
    pub fn set_on_select<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &T) + 'static,
    {
        self.view.set_on_select(cb)
    }

    /// Sets a callback to be used when an item is selected.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::traits::Nameable;
    /// use cursive_core::views::{SelectView, TextView};
    ///
    /// let text_view = TextView::new("").with_name("text");
    ///
    /// let select_view = SelectView::new()
    ///     .item("One", 1)
    ///     .item("Two", 2)
    ///     .on_select(|s, item| {
    ///         let content = match *item {
    ///             1 => "Content number one",
    ///             2 => "Content number two! Much better!",
    ///             _ => unreachable!("no such item"),
    ///         };
    ///
    ///         // Update the textview with the currently selected item.
    ///         s.call_on_name("text", |v: &mut TextView| {
    ///             v.set_content(content);
    ///         })
    ///         .unwrap();
    ///     });
    /// ```
    #[must_use]
    pub fn on_select<F>(mut self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &T) + 'static,
    {
        self.view = self.view.on_select(cb);
        self
    }

    /// Sets a callback to be used when `<Enter>` is pressed.
    ///
    /// Also happens if the user clicks an item.
    ///
    /// The item currently selected will be given to the callback.
    ///
    /// Here, `V` can be `T` itself, or a type that can be borrowed from `T`.
    pub fn set_on_submit<F, V: ?Sized>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, &V),
        T: Borrow<V>,
    {
        self.view.set_on_submit(cb)
    }

    /// Sets a callback to be used when `<Enter>` is pressed.
    ///
    /// Also happens if the user clicks an item.
    ///
    /// The item currently selected will be given to the callback.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::{Dialog, SelectView};
    ///
    /// let select_view = SelectView::new()
    ///     .item("One", 1)
    ///     .item("Two", 2)
    ///     .on_submit(|s, item| {
    ///         let content = match *item {
    ///             1 => "Content number one",
    ///             2 => "Content number two! Much better!",
    ///             _ => unreachable!("no such item"),
    ///         };
    ///
    ///         // Show a popup whenever the user presses <Enter>.
    ///         s.add_layer(Dialog::info(content));
    ///     });
    /// ```
    #[must_use]
    pub fn on_submit<F, V: ?Sized>(mut self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &V) + 'static,
        T: Borrow<V>,
    {
        self.view = self.view.on_submit(cb);
        self
    }

    /// Sets the alignment for this view.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::align;
    /// use cursive_core::views::SelectView;
    ///
    /// let select_view = SelectView::new()
    ///     .item("One", 1)
    ///     .align(align::Align::top_center());
    /// ```
    #[must_use]
    pub fn align(mut self, align: Align) -> Self {
        self.view = self.view.align(align);
        self
    }

    /// Sets the vertical alignment for this view.
    /// (If the view is given too much space vertically.)
    #[must_use]
    pub fn v_align(mut self, v: VAlign) -> Self {
        self.view = self.view.v_align(v);
        self
    }

    /// Sets the horizontal alignment for this view.
    #[must_use]
    pub fn h_align(mut self, h: HAlign) -> Self {
        self.view = self.view.h_align(h);
        self
    }

    /// Returns the value of the currently selected item.
    ///
    /// Returns `None` if the list is empty.
    pub fn selection(&self) -> Option<Rc<T>> {
        self.view.selection()
    }

    /// Removes all items from this view.
    pub fn clear(&mut self) {
        self.view.clear()
    }

    /// Adds a item to the list, with given label and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let mut select_view = SelectView::new();
    ///
    /// select_view.add_item("Item 1", 1);
    /// select_view.add_item("Item 2", 2);
    /// ```
    pub fn add_item<S: Into<StyledString>>(&mut self, label: S, value: T) {
        self.view.add_item(label, value)
    }

    /// Gets an item at given idx or None.
    ///
    /// ```
    /// use cursive_core::views::{SelectView, TextView};
    /// use cursive_core::Cursive;
    /// let select = SelectView::new().item("Short", 1);
    /// assert_eq!(select.get_item(0), Some(("Short", &1)));
    /// ```
    pub fn get_item(&self, i: usize) -> Option<(&str, &T)> {
        self.view.get_item(i)
    }

    /// Gets a mut item at given idx or None.
    pub fn get_item_mut(&mut self, i: usize) -> Option<(&mut StyledString, &mut T)> {
        self.view.get_item_mut(i)
    }

    /// Iterate mutably on the items in this view.
    ///
    /// Returns an iterator with each item and their labels.
    ///
    /// In some cases some items will need to be cloned (for example if a
    /// `Rc<T>` is still alive after calling `SelectView::selection()`).
    ///
    /// If `T` does not implement `Clone`, check `SelectView::try_iter_mut()`.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut StyledString, &mut T)>
    where
        T: Clone,
    {
        self.view.iter_mut()
    }

    /// Try to iterate mutably on the items in this view.
    ///
    /// Returns an iterator with each item and their labels.
    ///
    /// Some items may not be returned mutably, for example if a `Rc<T>` is
    /// still alive after calling `SelectView::selection()`.
    pub fn try_iter_mut(&mut self) -> impl Iterator<Item = (&mut StyledString, Option<&mut T>)> {
        self.view.try_iter_mut()
    }

    /// Iterate on the items in this view.
    ///
    /// Returns an iterator with each item and their labels.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &T)> {
        self.view.iter()
    }

    /// Removes an item from the list.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn remove_item(&mut self, id: usize) -> Callback {
        self.view.remove_item(id)
    }

    /// Inserts an item at position `index`, shifting all elements after it to
    /// the right.
    pub fn insert_item<S>(&mut self, index: usize, label: S, value: T)
    where
        S: Into<StyledString>,
    {
        self.view.insert_item(index, label, value)
    }

    /// Chainable variant of add_item
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let select_view = SelectView::new()
    ///     .item("Item 1", 1)
    ///     .item("Item 2", 2)
    ///     .item("Surprise item", 42);
    /// ```
    #[must_use]
    pub fn item<S: Into<StyledString>>(mut self, label: S, value: T) -> Self {
        self.view.add_item(label, value);
        self
    }

    /// Adds all items from from an iterator.
    pub fn add_all<S, I>(&mut self, iter: I)
    where
        S: Into<StyledString>,
        I: IntoIterator<Item = (S, T)>,
    {
        self.view.add_all(iter)
    }

    /// Adds all items from from an iterator.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// // Create a SelectView with 100 items
    /// let select_view =
    ///     SelectView::new().with_all((1u8..100).into_iter().map(|i| (format!("Item {}", i), i)));
    /// ```
    #[must_use]
    pub fn with_all<S, I>(mut self, iter: I) -> Self
    where
        S: Into<StyledString>,
        I: IntoIterator<Item = (S, T)>,
    {
        self.view.add_all(iter);
        self
    }

    /// Returns the id of the item currently selected.
    ///
    /// Returns `None` if the list is empty.
    pub fn selected_id(&self) -> Option<usize> {
        self.view.selected_id()
    }

    /// Returns the number of items in this list.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let select_view = SelectView::new()
    ///     .item("Item 1", 1)
    ///     .item("Item 2", 2)
    ///     .item("Item 3", 3);
    ///
    /// assert_eq!(select_view.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.view.len()
    }

    /// Returns `true` if this list has no item.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let mut select_view = SelectView::new();
    /// assert!(select_view.is_empty());
    ///
    /// select_view.add_item("Item 1", 1);
    /// select_view.add_item("Item 2", 2);
    /// assert!(!select_view.is_empty());
    ///
    /// select_view.clear();
    /// assert!(select_view.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.view.is_empty()
    }

    /// Sort the current items lexicographically by their label.
    ///
    /// Note that this does not change the current focus index, which means that the current
    /// selection will likely be changed by the sorting.
    ///
    /// This sort is stable: items with identical label will not be reordered.
    pub fn sort_by_label(&mut self) {
        self.view.sort_by_label()
    }

    /// Sort the current items with the given comparator function.
    ///
    /// Note that this does not change the current focus index, which means that the current
    /// selection will likely be changed by the sorting.
    ///
    /// The given comparator function must define a total order for the items.
    ///
    /// If the comparator function does not define a total order, then the order after the sort is
    /// unspecified.
    ///
    /// This sort is stable: equal items will not be reordered.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.view.sort_by(compare)
    }

    /// Sort the current items with the given key extraction function.
    ///
    /// Note that this does not change the current focus index, which means that the current
    /// selection will likely be changed by the sorting.
    ///
    /// This sort is stable: items with equal keys will not be reordered.
    pub fn sort_by_key<K, F>(&mut self, key_of: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.view.sort_by_key(key_of)
    }

    /// Moves the selection to the given position.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn set_selection(&mut self, i: usize) -> Callback {
        self.view.set_selection(i)
    }

    /// Sets the selection to the given position.
    ///
    /// Chainable variant.
    ///
    /// Does not apply `on_select` callbacks.
    #[must_use]
    pub fn selected(mut self, i: usize) -> Self {
        self.view.set_selection(i);
        self
    }

    /// Moves the selection up by the given number of rows.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`:
    ///
    /// ```rust
    /// # use cursive_core::Cursive;
    /// # use cursive_core::views::SelectView;
    /// fn select_up(siv: &mut Cursive, view: &mut SelectView<()>) {
    ///     let cb = view.select_up(1);
    ///     cb(siv);
    /// }
    /// ```
    pub fn select_up(&mut self, n: usize) -> Callback {
        self.view.select_up(n)
    }

    /// Moves the selection down by the given number of rows.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn select_down(&mut self, n: usize) -> Callback {
        self.view.select_down(n)
    }
}

impl SelectView<String> {
    /// Convenient method to use the label as value.
    pub fn add_item_str<S: Into<String>>(&mut self, label: S) {
        self.view.add_item_str(label)
    }

    /// Chainable variant of add_item_str
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let select_view = SelectView::new()
    ///     .item_str("Paris")
    ///     .item_str("New York")
    ///     .item_str("Tokyo");
    /// ```
    #[must_use]
    pub fn item_str<S: Into<String>>(mut self, label: S) -> Self {
        self.view.add_item_str(label);
        self
    }

    /// Convenient method to use the label as value.
    pub fn insert_item_str<S>(&mut self, index: usize, label: S)
    where
        S: Into<String>,
    {
        self.view.insert_item_str(index, label)
    }

    /// Adds all strings from an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cursive_core::views::SelectView;
    /// let mut select_view = SelectView::new();
    /// select_view.add_all_str(vec!["a", "b", "c"]);
    /// ```
    pub fn add_all_str<S, I>(&mut self, iter: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.view.add_all_str(iter)
    }

    /// Adds all strings from an iterator.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::SelectView;
    ///
    /// let text = "..."; // Maybe read some config file
    ///
    /// let select_view = SelectView::new().with_all_str(text.lines());
    /// ```
    #[must_use]
    pub fn with_all_str<S, I>(mut self, iter: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.view.add_all_str(iter);
        self
    }
}

impl<T: 'static> ViewWrapper for SelectView<T> {
    wrap_impl!(self.view: views::SelectView<T>);

    fn wrap_on_event(&mut self, ch: Event) -> EventResult {
        match ch {
            // we have to do it this way, because the functions for changing the focus are, of
            // course, private...
            key if key == CONFIG.keybindings.up => self.view.on_event(Event::Key(Key::Up)),
            key if key == CONFIG.keybindings.down => self.view.on_event(Event::Key(Key::Down)),
            Event::Key(Key::PageUp) => self.view.on_event(ch),
            Event::Key(Key::PageDown) => self.view.on_event(ch),
            Event::Key(Key::Home) => self.view.on_event(ch),
            Event::Key(Key::End) => self.view.on_event(ch),
            Event::Mouse { .. } => self.view.on_event(ch),
            Event::Key(Key::Enter) => self.view.on_event(ch),
            Event::Char(_) => self.view.on_event(ch),
            _ => EventResult::Ignored,
        }
    }
}
