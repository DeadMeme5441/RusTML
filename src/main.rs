mod types;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use types::{Document, SubTag, Tag};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut doc = Document::from(file_path.to_string());

    doc.opening_tags = get_opening_tags(&doc);
    doc.closing_tags = get_closing_tags(&doc);

    println!("{:?}", doc);
    println!("");

    tags_list(&doc);
}

fn get_opening_tags(doc: &Document) -> Vec<Tag> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[^/].*?>)").unwrap();
    }

    let document: Document = doc.clone();

    let all_opening_tags_list = RE.find_iter(&document.contents);

    let mut all_opening_tags_obj_list: Vec<Tag> = vec![];
    let mut opening_tags_name_list: HashSet<_> = HashSet::new();

    for tag in all_opening_tags_list {
        all_opening_tags_obj_list.push(generate_tag_object(&document, &tag.as_str().to_string()));
        opening_tags_name_list.insert(
            tag.as_str().split(";").collect::<Vec<_>>()[0]
                .replace("<", "")
                .trim()
                .to_string(),
        );
    }

    let mut opening_tags_obj_list: Vec<Tag> = vec![];

    for tag_name in &opening_tags_name_list {
        opening_tags_obj_list.push(Tag::from(tag_name));
    }

    for tag_obj in opening_tags_obj_list.iter_mut() {
        for all_tag in all_opening_tags_obj_list.iter_mut() {
            if &tag_obj.name == &all_tag.name {
                tag_obj.subtags.append(&mut all_tag.subtags);
            }
        }
    }

    println!("{:?}", opening_tags_obj_list);
    println!("{:?}", opening_tags_name_list);

    opening_tags_obj_list
}

fn get_closing_tags(doc: &Document) -> Vec<Tag> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[/].*?>)").unwrap();
    }

    let document: Document = doc.clone();

    let all_closing_tags_list = RE.find_iter(&document.contents);

    let mut all_closing_tags_obj_list: Vec<Tag> = vec![];
    let mut closing_tags_name_list: HashSet<_> = HashSet::new();

    for tag in all_closing_tags_list {
        all_closing_tags_obj_list.push(generate_tag_object(&document, &tag.as_str().to_string()));
        closing_tags_name_list.insert(
            tag.as_str().split(";").collect::<Vec<_>>()[0]
                .replace("<", "")
                .replace("/", "")
                .trim()
                .to_string(),
        );
    }

    let mut closing_tags_obj_list: Vec<Tag> = vec![];

    for tag_name in &closing_tags_name_list {
        closing_tags_obj_list.push(Tag::from(tag_name));
    }

    for tag_obj in closing_tags_obj_list.iter_mut() {
        for all_tag in all_closing_tags_obj_list.iter_mut() {
            if &tag_obj.name == &all_tag.name {
                tag_obj.subtags.append(&mut all_tag.subtags);
            }
        }
    }

    println!("{:?}", closing_tags_obj_list);
    println!("{:?}", closing_tags_name_list);

    closing_tags_obj_list
}

fn generate_tag_object(doc: &Document, tag: &String) -> Tag {
    let mut tag_name: String = tag.split(";").collect::<Vec<_>>()[0]
        .replace("<", "")
        .trim()
        .to_string();

    let subtag_list = &tag.split(";").collect::<Vec<_>>()[1..];

    let mut subtag_obj_list: Vec<SubTag> = vec![];

    let mut location = doc.contents.find(tag).unwrap();

    if !tag_name.contains("/") {
        location += tag_name.len();

        for sub_item in subtag_list {
            subtag_obj_list.push(SubTag {
                name: sub_item.split("=").collect::<Vec<_>>()[0]
                    .trim()
                    .to_string(),
                value: sub_item.split("=").collect::<Vec<_>>()[1]
                    .trim()
                    .replace(">", "")
                    .to_string(),
                start: location as i64,
                end: -1,
            })
        }
    } else {
        tag_name = tag_name.to_string().replace("/", "");
        for sub_item in subtag_list {
            subtag_obj_list.push(SubTag {
                name: sub_item.split("=").collect::<Vec<_>>()[0]
                    .trim()
                    .to_string(),
                value: sub_item.split("=").collect::<Vec<_>>()[1]
                    .trim()
                    .replace(">", "")
                    .to_string(),
                start: -1,
                end: location as i64,
            })
        }
    }

    Tag {
        name: tag_name,
        subtags: subtag_obj_list,
    }
}

fn tags_list(doc: &Document) {
    let document = doc.clone();

    let opening_tags_list: HashSet<String> = document
        .opening_tags
        .into_iter()
        .map(|tags| tags.name)
        .collect::<HashSet<_>>();

    let closing_tags_list: HashSet<String> = document
        .closing_tags
        .into_iter()
        .map(|tags| tags.name)
        .collect::<HashSet<_>>();

    println!("Opening Tags : {:?}", opening_tags_list);
    println!("Closing Tags : {:?}", closing_tags_list);

    let tags_names_list: HashSet<String> = (&opening_tags_list
        .union(&closing_tags_list)
        .collect::<HashSet<_>>())
        .difference(
            &opening_tags_list
                .intersection(&closing_tags_list)
                .collect::<HashSet<_>>(),
        )
        .collect::<HashSet<_>>();

    println!("{:?}", &tags_names_list);
}
