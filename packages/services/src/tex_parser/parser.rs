use std::env::var;
use sword::core::injectable;

use crate::tex_parser::*;

#[injectable(provider)]
pub struct PracticeReportParser {
    documents_dir: String,
}

impl PracticeReportParser {
    pub fn new() -> Self {
        PracticeReportParser {
            documents_dir: var("DOCUMENTS_DIR").expect("DOCUMENTS_DIR must be set"),
        }
    }

    // pub async fn parse_tex_file() -> Result<ParsedTex, ServiceError> {}
}

pub async fn parse_tex_from_content(
    content: &str,
) -> Result<ParsedTex, Box<dyn std::error::Error>> {
    let mut sections = Vec::new();

    // Split content by sections
    let parts: Vec<&str> = SECTION_RE.split(content).collect();
    let titles: Vec<String> = SECTION_RE
        .captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect();

    // Skip the preamble (before first section)
    for i in 1..parts.len() {
        let title = titles[i - 1].clone();
        let section_content = parts[i];

        let (content, subsections, tables, images, lists, equations) =
            parse_subsections(section_content);

        sections.push(Section {
            title,
            content,
            subsections,
            tables,
            images,
            lists,
            equations,
        });
    }

    Ok(ParsedTex { sections })
}

fn parse_subsections(
    content: &str,
) -> (
    String,
    Vec<Section>,
    Vec<Table>,
    Vec<Image>,
    Vec<List>,
    Vec<Equation>,
) {
    let (content, tables) = extract_tables(content);
    let (content, images) = extract_images(&content);
    let (content, lists) = extract_lists(&content);
    let (content, equations) = extract_equations(&content);

    let mut subsections = Vec::new();
    let sub_parts: Vec<&str> = SUBSECTION_RE.split(&content).collect();
    let sub_titles: Vec<String> = SUBSECTION_RE
        .captures_iter(&content)
        .map(|cap| cap[1].to_string())
        .collect();

    let pre_content = clean_content(sub_parts[0]);

    for j in 1..sub_parts.len() {
        let sub_title = sub_titles[j - 1].clone();
        let sub_content = sub_parts[j];

        let (
            sub_pre_content,
            sub_subsections,
            sub_tables,
            sub_images,
            sub_lists,
            sub_equations,
        ) = parse_subsections(sub_content);

        subsections.push(Section {
            title: sub_title,
            content: sub_pre_content,
            subsections: sub_subsections,
            tables: sub_tables,
            images: sub_images,
            lists: sub_lists,
            equations: sub_equations,
        });
    }

    (pre_content, subsections, tables, images, lists, equations)
}

fn extract_tables(text: &str) -> (String, Vec<Table>) {
    let mut tables = Vec::new();
    let mut cleaned = text.to_string();

    for cap in TABLE_RE.captures_iter(text) {
        let table_text = &cap[0];
        // Simple parsing: split by \\ for rows, & for cells
        let rows: Vec<Vec<String>> = table_text
            .lines()
            .filter(|line| {
                !line.trim().is_empty()
                    && !line.contains("\\hline")
                    && !line.contains("\\end")
                    && !line.contains("\\begin")
            })
            .map(|line| line.split('&').map(|cell| clean_content(cell)).collect())
            .collect();

        tables.push(Table { rows });
        cleaned = cleaned.replace(table_text, "");
    }

    (cleaned, tables)
}

fn extract_images(text: &str) -> (String, Vec<Image>) {
    let mut images = Vec::new();
    let mut cleaned = text.to_string();

    for cap in IMAGE_RE.captures_iter(text) {
        let path = cap[2].to_string();
        let caption = cap.get(3).map(|m| m.as_str().to_string());
        images.push(Image { path, caption });

        cleaned = cleaned.replace(&cap[0], "");
    }

    (cleaned, images)
}

fn extract_lists(text: &str) -> (String, Vec<List>) {
    let mut lists = Vec::new();
    let mut cleaned = text.to_string();
    for cap in LIST_RE.captures_iter(text) {
        let list_text = &cap[0];
        // Extract items
        let items: Vec<String> = list_text
            .split("\\item")
            .skip(1)
            .map(|item| clean_content(item.trim()))
            .collect();

        lists.push(List { items });
        cleaned = cleaned.replace(list_text, "");
    }

    (cleaned, lists)
}

fn extract_equations(text: &str) -> (String, Vec<Equation>) {
    let mut equations = Vec::new();
    let mut cleaned = text.to_string();

    for cap in EQUATION_RE.captures_iter(text) {
        let eq_text = cap[0].to_string();
        let content = eq_text
            .trim_start_matches('$')
            .trim_end_matches('$')
            .trim_start_matches("$$")
            .trim_end_matches("$$")
            .trim_start_matches("\\[")
            .trim_end_matches("\\]")
            .trim_start_matches("\\begin{equation}")
            .trim_end_matches("\\end{equation}")
            .to_string();

        equations.push(Equation {
            content: clean_content(&content),
        });

        cleaned = cleaned.replace(&eq_text, "");
    }

    (cleaned, equations)
}

fn clean_content(text: &str) -> String {
    let cleaned = CLEAN_RE.replace_all(text, "");
    cleaned
        .replace("\\", "")
        .replace("\n\n", " ")
        .trim()
        .to_string()
}
