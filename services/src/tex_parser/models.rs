use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub path: String,
    pub caption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equation {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: String,
    pub subsections: Vec<Section>,
    pub tables: Vec<Table>,
    pub images: Vec<Image>,
    pub lists: Vec<List>,
    pub equations: Vec<Equation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedTex {
    pub sections: Vec<Section>,
}
