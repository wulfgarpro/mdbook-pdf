mod outline {
    use std::fs;
    use std::io::stdin;

    use anyhow::Result;
    use lopdf::{Document, Object, ObjectId};
    use mdbook::renderer::RenderContext;
    use scraper::element_ref::ElementRef;
    use scraper::{Html, Selector};
    fn get_dest_page_number(pdf: &Document, dest: ObjectId) -> Option<u32> {
        for (number, id) in pdf.get_pages() {
            if id == dest {
                return Some(number);
            }
        }
        None
    }

    fn add_outline(root: ElementRef, pdf: &Document, level: u32) {
        let sections = Selector::parse(".section").unwrap();
        if let Some(s) = root.select(&sections).next() {
            add_outline(s, pdf, level + 1);
        } else {
            let anchor = Selector::parse("a").unwrap();
            for a in root.select(&anchor) {
                let href = a.value().attr("href").unwrap();
                let dest_name =
                    format!("{}", href.strip_suffix(".html").unwrap().replace("/", "-"));

                // `lopdf`'s `get_named_destinations` doesn't support named destinations in
                // the root. Get named destinations from the root.
                let catalog = pdf
                    .trailer
                    .get(b"Root")
                    .and_then(Object::as_reference)
                    .unwrap();
                let dict = pdf.get_dictionary(catalog).unwrap();
                let dests = pdf.get_dict_in_dict(dict, b"Dests").unwrap();
                let named_dest = dests
                    .into_iter()
                    .filter_map(|(key, value)| {
                        if String::from_utf8_lossy(key) == dest_name {
                            let value = value.as_array().unwrap();
                            return Some((
                                value[0].clone().as_reference().unwrap(), // Oid
                                value[1].clone(),                         // Type
                                value[2].clone(),                         // Top
                                value[3].clone(),                         // Left
                            ));
                        }
                        None
                    })
                    .collect::<Vec<_>>();

                let page_number = get_dest_page_number(pdf, named_dest[0].0);
                println!(
                    "{:?} : {:?} : {}",
                    dest_name,
                    named_dest[0],
                    page_number.unwrap()
                );

                for _ in 1..level {}
            }
        }

        // let names: Vec<String> = html
        //     .select(&sections)
        //     // .filter_map(|e| e.parent_element())
        //     .filter_map(|p| p.value().attr("href"))
        //     .map(String::from)
        //     .collect::<Vec<String>>();
        //
        // for name in names {
        //     let f = name.split('#').collect::<Vec<&str>>();
        // println!("{f:?}");
        // let selector = Selector::parse(&format!("#{id}")).unwrap();
        // for tag in html.select(&selector) {}
        // }

        // let id = catalog.id().unwrap();
        // dbg!(pdf.get_dict_in_dict(catalog, b"/Dest").unwrap());

        // let mut outlines = Dictionary::new();
        // let mut outline_entry = Dictionary::new();
        // outline_entry.set("Title", Object::string_literal("Sample entry"));
        //
        // outline_entry.set(
        //     "Dest",
        //     Object::Array(vec![
        //         Object::Reference(*pdf.get_pages().get(&3).unwrap()),
        //         Object::Name("Fit".into()),
        //     ]),                                ;
        // );
        //
        // let outline_item = pdf.add_object(outline_entry.clone());
        // outlines.set("First", Object::Reference(outline_item));
        // outlines.set("Last", Object::Reference(outline_item));
        //
        // let outlines_b = pdf.add_object(outlines);
        //
        // let catalog = pdf.trailer.get(b"Root").unwrap().as_reference().unwrap();
        // if let Ok(x) = pdf.get_object_mut(catalog) {
        //     if let Object::Dictionary(ref mut dict) = x {
        //         dict.set("Outlines", Object::Reference(outlines_b));
        //     }
        // }
        //
        // println!("updated catalog: {:#?}", pdf.get_object(catalog));
        // pdf.save("output_outline.pdf").unwrap();
    }

    fn main_org() -> Result<()> {
        let mut stdin = stdin();

        let _ = RenderContext::from_json(&mut stdin).unwrap();

        // let mut pdf = Document::load("../pdf/output.pdf")?;

        let print = fs::read_to_string("../html/print.html").unwrap();
        let html = Html::parse_document(&print);
        let chapters = Selector::parse(".chapter").unwrap();
        let level = 1;
        for c in html.select(&chapters) {
            add_outline(c, &pdf, level);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_outline() {
        let print = fs::read_to_string("print.html").unwrap();
        let html = Html::parse_document(&print);
        let chapter_selector = Selector::parse(".chapter").unwrap();
        let section_selector = Selector::parse(".section").unwrap();
        for c in html.select(&chapter_selector) {
            // let chapter = html.select(&chapter_selector).next().unwrap();
            for element in c.select(&section_selector) {
                println!("HERE");
                // add_outline(c);
            }
            println!("HERE");
        }
    }
}
