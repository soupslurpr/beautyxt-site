use std::rc::Rc;

use cool_site_builder::{
    elements::{details::Details, li::Li, p::P, strong::Strong, summary::Summary, text::Text},
    Element,
};

pub fn simple_feature_list_item(summary: &str, details: &str) -> Rc<Li> {
    feature_list_item(Text::new(summary), P::new(vec![], vec![Text::new(details)]))
}

pub fn feature_list_item(summary: Rc<dyn Element>, details: Rc<P>) -> Rc<Li> {
    Li::new(
        vec![],
        vec![Details::new(
            vec![],
            vec![
                Summary::new(vec![], vec![Strong::new(vec![], vec![summary])]),
                details,
            ],
        )],
    )
}
