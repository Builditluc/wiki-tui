use super::traits::Parser;

pub struct MediawikiParser;

impl Parser for MediawikiParser {
    fn parse_document<R: std::io::Read>(
        doc: R,
        sections: &Vec<crate::wiki::article_new::Section>,
    ) -> Vec<Box<dyn super::traits::Element>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn push_element(&mut self, element: Box<dyn super::traits::Element>) {
        todo!()
    }

    fn push_effect(&mut self, effect: cursive::theme::Effect) {
        todo!()
    }

    fn pop_effect(&mut self) {
        todo!()
    }

    fn effects(&self) -> Vec<cursive::theme::Effect> {
        todo!()
    }

    fn next_id(&mut self) -> u32 {
        todo!()
    }
}
