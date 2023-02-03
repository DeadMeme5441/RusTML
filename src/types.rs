use std::fs;

#[derive(Debug, Clone)]
pub struct SubTag {
    pub name: String,
    pub value: String,
    pub start: i64,
    pub end: i64,
}

impl SubTag {
    pub fn new(name: String, value: String, start: i64, end: i64) -> Self {
        SubTag {
            name,
            value,
            start,
            end,
        }
    }
}
impl PartialEq for SubTag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub subtags: Vec<SubTag>,
}

impl From<&String> for Tag {
    fn from(name: &String) -> Self {
        Tag {
            name: name.to_string(),
            subtags: vec![],
        }
    }
}

impl Tag {
    pub fn update(name: &String, subtags: Vec<SubTag>) -> Self {
        Tag {
            name: name.to_string(),
            subtags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub file_name: String,
    pub file_path: String,
    pub contents: String,
    pub text: String,
    pub tags: Vec<Tag>,
    pub opening_tags: Vec<Tag>,
    pub closing_tags: Vec<Tag>,
    pub errors: Errors,
}

impl From<String> for Document {
    fn from(path: String) -> Self {
        Document {
            file_name: path.split("/").last().unwrap().to_string(),
            file_path: path.clone(),
            contents: fs::read_to_string(path).expect("Should have been able to read the file."),
            text: String::new(),
            tags: vec![],
            opening_tags: vec![],
            closing_tags: vec![],
            errors: Errors {
                tag_errors: vec![],
                subtag_errors: vec![],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Errors {
    pub tag_errors: Vec<Tag>,
    pub subtag_errors: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Search {
    pub text: Vec<(usize, String)>,
    pub tag: Vec<Tag>,
    pub subtag: Vec<Tag>,
}
