use cool_site_builder::{
    attributes::href::Href,
    elements::{a::A, text::Text},
    Element,
};
use std::rc::Rc;

pub fn anchor_with_href_and_text(href: &str, text: &str) -> Rc<A> {
    A::new(vec![Href::new(href)], vec![Text::new(text)])
}

pub fn anchor_with_href_and_element(href: &str, element: Rc<dyn Element>) -> Rc<A> {
    A::new(vec![Href::new(href)], vec![element])
}
