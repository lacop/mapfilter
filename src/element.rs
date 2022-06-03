use osmpbf::Element;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct OwnedElement {
    id: i64,
    tags: BTreeMap<String, String>,
    lat_lon: Option<(f64, f64)>,
}

// TODO implement fancy Display

pub fn element_id(element: &Element) -> i64 {
    match element {
        Element::Node(n) => n.id(),
        Element::DenseNode(dn) => dn.id(),
        Element::Way(w) => w.id(),
        Element::Relation(r) => r.id(),
    }
}

pub fn element_tags(element: &Element) -> BTreeMap<String, String> {
    let mf = |(k, v): (&str, &str)| (k.to_owned(), v.to_owned());
    match element {
        Element::Node(n) => n.tags().map(mf).collect(),
        Element::DenseNode(dn) => dn.tags().map(mf).collect(),
        Element::Way(w) => w.tags().map(mf).collect(),
        Element::Relation(r) => r.tags().map(mf).collect(),
    }
}

pub fn element_lat_lon(element: &Element) -> Option<(f64, f64)> {
    match element {
        Element::Node(n) => Some((n.lat(), n.lon())),
        Element::DenseNode(dn) => Some((dn.lat(), dn.lon())),
        _ => None,
    }
}

impl<'a> From<Element<'a>> for OwnedElement {
    fn from(element: Element<'a>) -> Self {
        OwnedElement {
            id: element_id(&element),
            tags: element_tags(&element),
            lat_lon: element_lat_lon(&element),
        }
    }
}
