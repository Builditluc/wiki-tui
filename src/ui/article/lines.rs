use crate::config::CONFIG;
use cursive::theme::Style;

use crate::ui::article::links::{Link, LinkHandler};
use crate::ui::article::view::{Element, Line, RenderedElement};

pub struct LinesWrapper {
    width: usize,
    pub lines_wrapped: bool,

    pub headers: Vec<String>,
    pub header_coords: Vec<usize>,
    pub lines: Vec<Line>,
}

impl LinesWrapper {
    pub fn new(width: usize, headers: Vec<String>) -> Self {
        return LinesWrapper {
            width,
            lines_wrapped: false,

            headers,
            header_coords: Vec::new(),
            lines: Vec::new(),
        };
    }

    pub fn calculate_lines<'a>(
        mut self,
        content: &'a Vec<RenderedElement>,
        link_handler: &'a mut LinkHandler,
    ) -> Self {
        // the width of the line that is currently calculated
        let mut line_width: usize = 0;

        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Vec<Element> = Vec::new();

        let mut lines_wrapped = false;

        // this is to prevent the program to add more headers of the same name
        self.header_coords.clear();

        // go through every rendered element
        for (idx, element) in content.iter().enumerate() {
            log::debug!("Rendering now the element no: {}", idx);
            let element_width = element.text.chars().count();
            let link_index = match element.link_destination {
                Some(ref destination) => Some(link_handler.push(Link {
                    position: (line_width, lines.len()).into(),
                    width: element_width,
                    destination: destination.to_string(),
                })),
                None => None,
            };

            log::trace!(
                "The element has the link_index: {:?}, and a width of: {}",
                link_index,
                element_width
            );

            if element.newline && element_width.eq(&0) {
                log::debug!("Created a new line");
                lines.push(std::mem::replace(&mut current_line, Vec::new()));
                line_width = 0;

                continue;
            }

            let is_header = self.headers.contains(&element.text);

            // does the element fit in the current line?
            if (line_width + element_width) < self.width {
                // if it goes into a new line, add it to a new one
                if element.newline {
                    log::debug!("Adding the element to a new line");
                    // fill the current line with empty spaces
                    current_line.push(Element {
                        text: " ".repeat(self.width - line_width).to_string(),
                        style: Style::from(CONFIG.theme.text),
                        width: self.width - line_width,
                        link_index: None,
                    });

                    // add the current line to the finished ones and add the new element to another
                    // line
                    line_width = self.add_element_to_new_line(
                        &mut lines,
                        &mut current_line,
                        element,
                        link_index,
                    );

                    if is_header {
                        self.headers.remove(0);
                        self.header_coords.push(lines.len());
                    }

                    // this element is finished, continue with the next one
                    continue;
                }

                // if the element goes to the current line, add it to the line
                log::debug!("Adding the element to the current line");
                current_line.push(self.create_element_from_rendered_element(element, link_index));

                if is_header {
                    self.headers.remove(0);
                    self.header_coords.push(lines.len())
                }

                // don't forget to increase the line width and continue with the next element
                line_width += element_width;
                log::trace!("New line width: {}", line_width);

                continue;
            } else {
                // hmm, the element doesn't fit into the current line
                // however, does the element go into a new line? if so, check if it fits into a
                // single line and split it if not
                log::debug!("The element no: {} doesn't fit into the current line", idx);
                if element.newline {
                    if element_width < self.width {
                        log::debug!("Adding the element to a new line");
                        // fill the current line with empty spaces
                        current_line.push(Element {
                            text: " ".repeat(self.width - line_width).to_string(),
                            style: Style::from(CONFIG.theme.text),
                            width: self.width - line_width,
                            link_index: None,
                        });

                        // add the current line to the finished ones and add the new element to another
                        // line
                        line_width = self.add_element_to_new_line(
                            &mut lines,
                            &mut current_line,
                            element,
                            link_index,
                        );

                        if is_header {
                            self.headers.remove(0);
                            self.header_coords.push(lines.len())
                        }

                        // this element is finished, continue with the next one
                        continue;
                    }

                    // so.. the element goes into a new line, but it doesn't fit into a single one
                    // well, then split it and add it to a new line

                    log::debug!("The element no: {} must be splitted", idx);
                    let mut new_lines = self.split_element(element, link_index, 0, self.width);
                    log::trace!("splitted lines: \n{:?}", new_lines);

                    line_width = 0;
                    lines_wrapped = true;

                    lines.push(std::mem::replace(&mut current_line, Vec::new()));
                    lines.append(&mut new_lines);
                    log::debug!("Added the current line and the new lines to the finished lines");

                    if is_header {
                        self.headers.remove(0);
                        self.header_coords.push(lines.len())
                    }

                    continue;
                }

                // split the element
                log::debug!("The element will now be splitted");
                lines_wrapped = true;
                let mut new_lines = self.split_element(element, link_index, line_width, self.width);
                log::trace!("splitted lines: \n{:?}", new_lines);

                // the first line of these new lines is merged with the current line
                current_line.append(&mut new_lines.remove(0));
                log::debug!("merged the first of these new lines with the current one");
                log::trace!("new_lines: \n{:?}", new_lines);
                log::trace!("new_lines after pop(): \n{:?}", new_lines.pop());
                lines.push(std::mem::replace(
                    &mut current_line,
                    new_lines.pop().unwrap_or_default(),
                ));
                log::debug!("added the other new lines to the finished ones");

                // all the other lines will be added to the finished lines
                lines.append(&mut new_lines);

                line_width = current_line.iter().map(|element| element.width).sum();
                log::debug!("new line width: {}", line_width);

                if is_header {
                    self.headers.remove(0);
                    self.header_coords.push(lines.len())
                }

                continue;
            }
        }
        self.lines_wrapped = lines_wrapped;

        // add the remaining line to the finished ones, because no elements are left
        lines.push(current_line);

        self.lines = lines;
        return self;
    }

    fn create_element_from_rendered_element(
        &self,
        element: &RenderedElement,
        link_index: Option<usize>,
    ) -> Element {
        Element {
            text: element.text.to_string(),
            style: element.style,
            width: element.text.chars().count(),
            link_index,
        }
    }

    fn split_element(
        &self,
        element: &RenderedElement,
        link_index: Option<usize>,
        line_width: usize,
        max_width: usize,
    ) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Line = Vec::new();
        let mut current_line_width = line_width;

        // first, split the element into chunks
        for (idx, chunk) in element.text.split(' ').enumerate().map(|(idx, chunk)| {
            if idx == 0 {
                (idx, chunk.to_string())
            } else {
                (idx, format!(" {}", chunk))
            }
        }) {
            log::debug!("the chunk no: {} will now be added", idx);
            let chunk_width = chunk.chars().count();
            log::trace!("chunk width: {}", chunk_width);

            // does the cunk fit into the current line? then add it and continue with the next one
            if (current_line_width + chunk_width) < max_width {
                current_line.push(Element {
                    text: chunk,
                    style: element.style,
                    width: chunk_width,
                    link_index,
                });
                log::debug!("Added the chunk to the current line");

                current_line_width += chunk_width;
                continue;
            }

            // if not, add it to a new line
            let element_from_chunk = RenderedElement {
                text: chunk,
                style: element.style,
                newline: element.newline,
                link_destination: element.link_destination.clone(),
            };

            current_line_width = self.add_element_to_new_line(
                &mut lines,
                &mut current_line,
                &element_from_chunk,
                link_index,
            );
            log::debug!("Added the chunk no: {} to a new line", idx);
        }
        // add the remaining line to the finished ones, because no chunks are left
        lines.push(current_line);

        lines
    }

    fn add_element_to_new_line(
        &self,
        lines: &mut Vec<Line>,
        current_line: &mut Line,
        element: &RenderedElement,
        link_index: Option<usize>,
    ) -> usize {
        log::trace!("current_line: \n{:?}", current_line);
        let mut element_width: usize = 0;
        lines.push(std::mem::replace(
            current_line,
            vec![{
                let trimmed_element = RenderedElement {
                    text: element.text.trim_start().to_string(),
                    style: element.style,
                    newline: element.newline,
                    link_destination: element.link_destination.clone(),
                };
                let element =
                    self.create_element_from_rendered_element(&trimmed_element, link_index);
                element_width = element.width;
                element
            }],
        ));
        log::trace!("new current line: \n{:?}", current_line);
        element_width
    }
}
