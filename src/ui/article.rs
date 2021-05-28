use crate::config::CONFIG;
use crate::wiki::article::*;
use cursive::align::Align;
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::lines::spans::*;
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::XY;
use cursive::{Printer, Vec2};

pub struct ArticleView {
    content: ArticleContent,
    lines: Vec<Row>,

    last_size: Vec2,
    width: Option<usize>,
}

struct ArticleContent {
    content: ArticleContentInner,

    elements_count: usize,
}

struct ArticleContentInner {
    content_value: StyledString,
    size_cache: Option<XY<SizeCache>>,

    // cache more values to improve performance
    historical_caches: Vec<(Vec2, Vec2)>,
}

impl ArticleContentInner {
    fn is_chache_valid(&self, size: Vec2) -> bool {
        match self.size_cache {
            None => false,
            Some(ref last) => last.x.accept(size.x) && last.y.accept(size.y),
        }
    }
}

impl ArticleContent {
    pub fn new(content: StyledString) -> ArticleContent {
        ArticleContent {
            content: ArticleContentInner {
                content_value: content,
                size_cache: None,
                historical_caches: Vec::new(),
            },
            elements_count: 0,
        }
    }

    fn render(&mut self, article: Article) {
        self.elements_count = article.elements.len();
        let mut rendered_article = StyledString::new();

        // go trough every element in the article
        for element in article.elements.into_iter() {
            match element.element_type {
                // if its a link, make it underlined
                ArticleElementType::Link => {
                    let link_span = StyledString::styled(
                        element.content,
                        Style::from(CONFIG.theme.text).combine(Effect::Underline),
                    );

                    rendered_article.append(link_span);
                }
                // if its text, just append it to the rendered article
                ArticleElementType::Text => {
                    let text_span =
                        StyledString::styled(element.content, Style::from(CONFIG.theme.text));

                    rendered_article.append(text_span);
                }
                // if its a header, add some linebreaks and make the header bold
                ArticleElementType::Header => {
                    let header_span = StyledString::styled(
                        format!("\n{}\n\n", element.content),
                        Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                    );

                    rendered_article.append(header_span);
                }
            }
        }

        self.content.content_value = rendered_article;
    }

    fn set_article(&mut self, article: Article) {
        // render the new article
        self.render(article);

        // after rendering, flush the caches to prevent crashes
        self.content.historical_caches.clear();
    }
}

impl ArticleView {
    pub fn new<S>(content: S) -> ArticleView
    where
        S: Into<StyledString>,
    {
        ArticleView {
            content: ArticleContent::new(content.into()),
            lines: Vec::new(),
            last_size: Vec2::zero(),
            width: None,
        }
    }

    pub fn set_article(&mut self, article: Article) {
        self.content.set_article(article)
    }

    fn calculate_lines(&mut self, size: Vec2) {
        if self.content.content.is_chache_valid(size) || size.x == 0 {
            return;
        }

        self.content.content.size_cache = None;

        self.lines = LinesIterator::new(&self.content.content.content_value, size.x).collect();

        self.width = if self.lines.iter().any(|line| line.is_wrapped) {
            Some(size.x)
        } else {
            self.lines.iter().map(|line| line.width).max()
        };
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let h = self.lines.len();

        let offset = Align::top_left().v.get_offset(h, printer.size.y);
        let printer = &printer.offset((0, offset));

        let miny = printer.content_offset.y;
        let maxy = printer.output_size.y + printer.content_offset.y;

        // got through every row and print it to the screen
        for (y, line) in self.lines.iter().enumerate() {
            if y < miny || y >= maxy {
                continue;
            }

            let l = line.width;
            let mut x = Align::top_left().h.get_offset(l, printer.size.x);

            for span in line.resolve(&self.content.content.content_value) {
                // print every span in a line with it's style and increase the x
                // value by the width of the span to prevent overwriting a previous span
                printer.with_style(*span.attr, |printer| {
                    printer.print((x, y), &span.content);
                    x += span.width;
                });
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // is this the same size as before? stop recalculating things!
        if self.last_size == size {
            return;
        }

        // set the new width and calculate the lines
        self.last_size = size;
        self.calculate_lines(size);

        let my_size = Vec2::new(self.width.unwrap_or(0), self.lines.len());
        self.content.content.size_cache = Some(SizeCache::build(my_size, size));
        self.content.content.historical_caches.clear();
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        // do we already have the required size calculated and cached?
        for previous_size in self.content.content.historical_caches.iter() {
            let req_size = previous_size.0;
            if req_size == size {
                return previous_size.1;
            }
        }

        // if we don't have the size calculated, calculate it and add it to the cache
        info!("[ui::article::AritlceView::required_size] Recalculating Size");
        self.calculate_lines(size);
        let required_size = Vec2::new(self.width.unwrap_or(0), self.lines.len());

        self.content
            .content
            .historical_caches
            .insert(0, (size, required_size));

        required_size
    }

    fn needs_relayout(&self) -> bool {
        self.content.content.size_cache.is_none()
    }
}
