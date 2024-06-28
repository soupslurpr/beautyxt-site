use std::rc::Rc;

use cool_site_builder::{
    attributes::{href::Href, id::Id, open::Open},
    elements::{
        a::A, details::Details, li::Li, p::P, strong::Strong, summary::Summary, text::Text,
    },
};

pub fn simple_feature_list_item(summary: &str, details: &str) -> Rc<Li> {
    feature_list_item(summary, P::new(vec![], vec![Text::new(details)]))
}

pub fn feature_list_item(summary: &str, details: Rc<P>) -> Rc<Li> {
    Li::new(
        vec![Id::new(summary)],
        vec![Details::new(
            vec![Open::new()],
            vec![
                Summary::new(
                    vec![],
                    vec![Strong::new(
                        vec![],
                        vec![A::new(
                            vec![Href::new(&format!("#{}", summary))],
                            vec![Text::new(summary)],
                        )],
                    )],
                ),
                details,
            ],
        )],
    )
}
