#![feature(drain_filter)]
mod types;
use lazy_static::lazy_static;
use regex::{Error, Regex};
use std::collections::HashSet;
use std::env;
use types::{Document, Search, SubTag, Tag};

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<.*?>)").unwrap();
    }

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    // loads document
    let mut doc = Document::from(file_path.to_string());

    doc.text = RE.replace_all(&doc.contents, "").to_string();
    doc.text = doc.text.replace("\n", "");

    doc.opening_tags = get_opening_tags(&doc);
    doc.closing_tags = get_closing_tags(&doc);

    tags_list(&mut doc);
    generate_errors(&mut doc);

    println!("{:?}", doc);
    // println!("{:?}", doc.opening_tags);
    // println!("{:?}", doc.closing_tags);
    // println!("{:?}", search_text(&doc, "Lorem".to_string()));
    // println!("{:?}", search_tag(&doc, "tagOne".to_string()));
    // println!("{:?}", search_subtag(&doc, "subtag".to_string()));
    // println!("{:?}", search_document(&doc, "tag".to_string()));
}

fn get_opening_tags(doc: &Document) -> Vec<Tag> {
    // Generates regular expression and stores it statically so it doesn't have to be computed everytime.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[^/].*?>)").unwrap();
    }

    let document: Document = doc.clone();

    // Gets all opening tags based on regex.
    let all_opening_tags_list = RE.find_iter(&document.contents);

    let mut all_opening_tags_obj_list: Vec<Tag> = vec![];
    let mut opening_tags_name_list: HashSet<_> = HashSet::new();

    // Generates 2 things.
    for tag in all_opening_tags_list {
        // Creates a vector of all Tag objects from the tags list.
        all_opening_tags_obj_list.push(generate_tag_object(&document, &tag.as_str().to_string()));
        // Creates a set of all opening tag names.
        opening_tags_name_list.insert(
            tag.as_str().split(";").collect::<Vec<_>>()[0]
                .replace("<", "")
                .trim()
                .to_string(),
        );
    }

    let mut opening_tags_obj_list: Vec<Tag> = vec![];

    // Generates a vector of Tags with the names of the opening tags.
    for tag_name in &opening_tags_name_list {
        opening_tags_obj_list.push(Tag::from(tag_name));
    }

    // Appends all opening tags and adds the subtags from all opening tags.
    for tag_obj in opening_tags_obj_list.iter_mut() {
        for all_tag in all_opening_tags_obj_list.iter_mut() {
            if &tag_obj.name == &all_tag.name {
                tag_obj.subtags.append(&mut all_tag.subtags);
            }
        }
    }

    // println!("{:?}", opening_tags_obj_list);
    // println!("{:?}", opening_tags_name_list);

    // Returns final object list of opening tags.
    opening_tags_obj_list
}

fn get_closing_tags(doc: &Document) -> Vec<Tag> {
    // Generates regular expression and stores it statically so it doesn't have to be computed everytime.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[/].*?>)").unwrap();
    }

    let document: Document = doc.clone();

    let all_closing_tags_list = RE.find_iter(&document.contents);

    let mut all_closing_tags_obj_list: Vec<Tag> = vec![];
    let mut closing_tags_name_list: HashSet<_> = HashSet::new();

    // Generates 2 things.
    for tag in all_closing_tags_list {
        // Creates a vector of all Tag objects from the tags list.
        all_closing_tags_obj_list.push(generate_tag_object(&document, &tag.as_str().to_string()));
        // Creates a set of all opening tag names.
        closing_tags_name_list.insert(
            tag.as_str().split(";").collect::<Vec<_>>()[0]
                .replace("<", "")
                .replace("/", "")
                .trim()
                .to_string(),
        );
    }

    let mut closing_tags_obj_list: Vec<Tag> = vec![];

    // Generates a vector of Tags with the names of the opening tags.
    for tag_name in &closing_tags_name_list {
        closing_tags_obj_list.push(Tag::from(tag_name));
    }

    // Appends all opening tags and adds the subtags from all opening tags.
    for tag_obj in closing_tags_obj_list.iter_mut() {
        for all_tag in all_closing_tags_obj_list.iter_mut() {
            if &tag_obj.name == &all_tag.name {
                tag_obj.subtags.append(&mut all_tag.subtags);
            }
        }
    }

    // println!("{:?}", closing_tags_obj_list);
    // println!("{:?}", closing_tags_name_list);

    // Returns final object list of closing tags.
    closing_tags_obj_list
}

fn generate_tag_object(doc: &Document, tag: &String) -> Tag {
    // Gets tag name from the full tag string.
    let mut tag_name: String = tag.split(";").collect::<Vec<_>>()[0]
        .replace("<", "")
        .trim()
        .to_string();

    // Gets subtags from the tag string.
    let subtag_list = &tag.split(";").collect::<Vec<_>>()[1..];

    let mut subtag_obj_list: Vec<SubTag> = vec![];

    // Gets location of the tag string from the document.
    let mut location = doc.contents.find(tag).unwrap();

    // Checks if tag is opening or closing
    if !tag_name.contains("/") {
        location += tag.len();

        // If tag is opening, sets subtag ending as -1.
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

        // If tag is closing, set subtag starting as -1.
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

    // Returns Tag struct.
    Tag {
        name: tag_name,
        subtags: subtag_obj_list,
    }
}

fn tags_list(doc: &mut Document) {
    let mut document = doc;

    let opening_tags_list: HashSet<String> = document
        .clone()
        .opening_tags
        .into_iter()
        .map(|tags| tags.name)
        .collect::<HashSet<_>>();

    let closing_tags_list: HashSet<String> = document
        .clone()
        .closing_tags
        .into_iter()
        .map(|tags| tags.name)
        .collect::<HashSet<_>>();

    // Gets tag names in both sets as an union.
    let all_tags_list: HashSet<_> = opening_tags_list
        .union(&closing_tags_list)
        .collect::<HashSet<_>>();

    let mut final_tags: Vec<Tag> = Vec::new();

    for name in all_tags_list {
        let opening_subtags_list: Vec<SubTag> = document
            .opening_tags
            .iter()
            .filter(|tag| tag.name == *name)
            .cloned()
            .collect::<Vec<Tag>>()
            .iter()
            .flat_map(|tag| tag.subtags.clone())
            .collect::<Vec<SubTag>>();

        let closing_subtags_list: Vec<SubTag> = document
            .closing_tags
            .iter()
            .filter(|tag| tag.name == *name)
            .cloned()
            .collect::<Vec<Tag>>()
            .iter()
            .flat_map(|tag| tag.subtags.clone())
            .collect::<Vec<SubTag>>();

        let final_subtags_list: Vec<SubTag> =
            update_subtags_location(opening_subtags_list, closing_subtags_list);

        let final_generated_tag = Tag::update(name, final_subtags_list);

        final_tags.push(final_generated_tag);
    }

    document.tags = final_tags;
}

fn update_subtags_location(
    opening_subtags_list: Vec<SubTag>,
    closing_subtags_list: Vec<SubTag>,
) -> Vec<SubTag> {
    let mut final_subtags_list: Vec<SubTag> = Vec::new();

    for subtag_start_item in opening_subtags_list.clone() {
        for subtag_end_item in closing_subtags_list.clone() {
            if subtag_start_item == subtag_end_item {
                let final_subtag_item: SubTag = SubTag {
                    name: subtag_start_item.name.clone(),
                    value: subtag_start_item.value.clone(),
                    start: subtag_start_item.start,
                    end: subtag_end_item.end,
                };
                final_subtags_list.push(final_subtag_item);
            }
        }
    }

    for subtag_start_item in opening_subtags_list.clone() {
        if final_subtags_list.contains(&subtag_start_item) == false {
            final_subtags_list.push(subtag_start_item);
        }
    }
    for subtag_end_item in closing_subtags_list.clone() {
        if final_subtags_list.contains(&subtag_end_item) == false {
            final_subtags_list.push(subtag_end_item);
        }
    }

    final_subtags_list
}

fn tag_errors(doc: &mut Document) -> Vec<String> {
    let document = doc.clone();

    let opening_tags_list: HashSet<String> = document
        .opening_tags
        .iter()
        .cloned()
        .map(|tags| tags.name)
        .collect::<HashSet<String>>();

    let closing_tags_list: HashSet<String> = document
        .closing_tags
        .iter()
        .cloned()
        .map(|tags| tags.name)
        .collect::<HashSet<String>>();

    // Gets tag names in one set but not both sets
    // Basically opened but not closed, and closed but not opened.
    let tag_error_list: Vec<String> = opening_tags_list
        .symmetric_difference(&closing_tags_list)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // println!("{:?}", tag_error_list);

    doc.errors.tag_errors = doc
        .clone()
        .tags
        .into_iter()
        .filter(|tag| tag_error_list.contains(&tag.name))
        .collect::<Vec<Tag>>();

    tag_error_list
}

fn subtag_errors(doc: &mut Document) -> Vec<Tag> {
    let document = doc.clone();

    // println!("{:?}", document.tags);

    let mut subtag_error_list: Vec<Tag> = Vec::new();

    for tag in document.tags.iter() {
        let mut errors: Vec<SubTag> = Vec::new();
        for subtag in tag.subtags.iter() {
            if subtag.start == -1 {
                errors.push(subtag.clone());
                // println!("{:?} -> Closed but not opened", subtag);
            }
            if subtag.end == -1 {
                errors.push(subtag.clone());
                // println!("{:?} -> Opened but not closed", subtag);
            }
        }
        subtag_error_list.push(Tag::update(&tag.name, errors));
    }

    doc.errors.subtag_errors = subtag_error_list.clone();

    subtag_error_list
}

fn generate_errors(doc: &mut Document) {
    tag_errors(doc);
    subtag_errors(doc);
}

fn search_text(doc: &Document, search_term: String) -> Vec<(usize, String)> {
    let results: Vec<(usize, String)> = doc
        .text
        .match_indices(&search_term)
        .map(|(pos, val)| (pos, val.to_string()))
        .collect::<Vec<(usize, String)>>();
    results
}

fn search_tag(doc: &Document, search_term: String) -> Vec<Tag> {
    let results: Vec<Tag> = doc
        .clone()
        .tags
        .drain_filter(|tag| tag.name.contains(&search_term) == true)
        .collect::<Vec<Tag>>();

    results
}

fn search_subtag(doc: &Document, search_term: String) -> Vec<Tag> {
    let results = doc
        .clone()
        .tags
        .into_iter()
        .map(|tag| Tag {
            name: tag.name,
            subtags: tag
                .subtags
                .clone()
                .drain_filter(|subtag| {
                    subtag.name.contains(&search_term) || subtag.value.contains(&search_term)
                })
                .collect::<Vec<SubTag>>(),
        })
        .collect::<Vec<Tag>>();

    results
}

fn search_document(doc: &Document, search_term: String) -> Search {
    Search {
        text: search_text(doc, search_term.clone()),
        tag: search_tag(doc, search_term.clone()),
        subtag: search_subtag(doc, search_term.clone()),
    }
}
