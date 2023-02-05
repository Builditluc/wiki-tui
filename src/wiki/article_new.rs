use crate::wiki::parser::traits::Element;
use anyhow::Result;

use super::{
    api::mediawiki::MediawikiArticle,
    parser::{mediawiki::MediawikiParser, traits::Parser},
};

#[derive(Debug, Clone)]
pub struct Section {
    pub id: u32,
    title: String,
    number: String,
    pub anchor: String,
    header_type: HeaderType,
}

impl Section {
    pub fn new(
        id: u32,
        title: String,
        number: String,
        anchor: String,
        header_type: HeaderType,
    ) -> Self {
        Section {
            id,
            title,
            number,
            anchor,
            header_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HeaderType {
    Main,
    Sub,
    Section,
    Subsection,
    Minor,
    Detail,
}

impl From<usize> for HeaderType {
    fn from(val: usize) -> Self {
        match val {
            0 => HeaderType::Main,
            1 => HeaderType::Sub,
            2 => HeaderType::Section,
            3 => HeaderType::Subsection,
            4 => HeaderType::Minor,
            5 => HeaderType::Detail,
            _ => HeaderType::Main,
        }
    }
}

pub struct Article {
    title: String,
    id: u64,
    content: Vec<Box<dyn Element>>,
    sections: Vec<Section>,
}

impl Article {
    pub fn from_mediawiki(article: MediawikiArticle) -> Result<Self> {
        let mut sections: Vec<Section> = Vec::new();
        article.sections.map(|x| {
            sections.append(
                &mut x
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        Section::new(
                            i as u32,
                            x.text.to_string(),
                            x.number.to_string(),
                            x.anchor.to_string(),
                            HeaderType::from(x.level),
                        )
                    })
                    .collect::<Vec<Section>>(),
            );
        });

        let content =
            MediawikiParser::new().parse_document(article.text.unwrap().as_bytes(), &sections)?;
        Ok(Article {
            title: article.title,
            id: article.id,
            content,
            sections,
        })
    }

    pub fn section_from_id(&self, id: u32) -> Option<&Section> {
        self.sections.iter().find(|x| x.id == id)
    }

    pub fn content(&self) -> &Vec<Box<dyn Element>> {
        &self.content
    }
}

impl std::fmt::Debug for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Article {{ title: {}, id: {}, content: Vec {:?}, sections: {:?} }}",
            self.title,
            self.id,
            self.content.len(),
            self.sections
        )?;
        Ok(())
    }
}
