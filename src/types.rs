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

#[derive(Debug, Clone)]
pub struct Document {
    pub file_name: String,
    pub file_path: String,
    pub contents: String,
    pub tags: Vec<Tag>,
    pub opening_tags: Vec<Tag>,
    pub closing_tags: Vec<Tag>,
}

impl From<String> for Document {
    fn from(path: String) -> Self {
        Document {
            file_name: path.split("/").last().unwrap().to_string(),
            file_path: path.clone(),
            contents: fs::read_to_string(path).expect("Should have been able to read the file."),
            tags: vec![],
            opening_tags: vec![],
            closing_tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Errors {
    tag_errors: Vec<Tag>,
    subtag_errors: Vec<SubTag>,
    value: bool,
}
