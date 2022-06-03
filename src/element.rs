use color_eyre::eyre::Result;
use console::{style, Term};
use ellipse::Ellipse;
use geoutils::Location;
use osmpbf::Element;
use std::collections::BTreeMap;
use thousands::Separable;

#[derive(Debug)]
pub struct OwnedElement {
    id: i64,
    tags: BTreeMap<String, String>,
    lat_lon: Option<(f64, f64)>,
}

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

impl OwnedElement {
    pub fn print(&self, term: &Term, index: u64, query_lat_lon: Option<(f64, f64)>) -> Result<()> {
        let dim_bar = style("┃").dim();
        term.write_line(&format!(
            "{} {} {}",
            style("┏").dim(),
            style(self.get_name().unwrap_or("(unknown name)".to_string()))
                .green()
                .bold(),
            style(&format!("(#{index})")).dim()
        ))?;
        term.write_line(&format!(
            "{}  📍 http://openstreetmap.org/node/{}",
            dim_bar, self.id
        ))?;
        if let Some((lat, lon)) = self.lat_lon {
            term.write_line(&format!(
                "{}  🌍 http://google.com/maps/search/{:.5}+{:.5}",
                dim_bar, lat, lon
            ))?;
            if let Some((query_lat, query_lon)) = query_lat_lon {
                let location_element = Location::new(lat, lon);
                let location_query = Location::new(query_lat, query_lon);
                let distance = location_element
                    .haversine_distance_to(&location_query)
                    .meters();
                term.write_line(&format!(
                    "{}  📏 {} meters",
                    dim_bar,
                    (distance as u64).separate_with_underscores()
                ))?;
            }
        }

        // TODO 🏷️ labels
        term.write_line(&style("┗━━━━").dim().to_string())?;

        Ok(())
    }

    // Return a suitable name, with limited length.
    fn get_name(&self) -> Option<String> {
        for (k, v) in &self.tags {
            if k == "name" {
                return Some(v.as_str().truncate_ellipse(50).to_string());
            }
        }
        None
    }
}
