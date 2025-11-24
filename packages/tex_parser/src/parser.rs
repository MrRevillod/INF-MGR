use crate::*;

pub struct LaTexParser;

#[derive(Debug, Clone)]
struct SectionParseResult {
    title: String,
    content: String,
}

impl LaTexParser {
    /// Parses LaTeX content into structured chunks for plagiarism analysis.
    pub fn parse(content: &str) -> ParsedTex {
        let cleaned_content = Self::strip_comments(content);
        let sections = Self::extract_sections(&cleaned_content);
        let chunks = Self::process_sections(sections);

        ParsedTex { chunks }
    }

    /// Extracts all sections using balanced brace parsing.
    fn extract_sections(content: &str) -> Vec<SectionParseResult> {
        let balanced_titles = Self::extract_balanced_title(content, "\\section");

        if balanced_titles.is_empty() {
            return Self::extract_sections_fallback(content);
        }

        Self::build_sections_from_balanced(content, balanced_titles)
    }

    /// Fallback method using regex when balanced parsing fails.
    fn extract_sections_fallback(content: &str) -> Vec<SectionParseResult> {
        let parts: Vec<&str> = SECTION_RE.split(content).collect();
        let titles: Vec<String> = SECTION_RE
            .captures_iter(content)
            .map(|cap| cap[1].to_string())
            .collect();

        titles
            .into_iter()
            .enumerate()
            .filter_map(|(i, title)| {
                parts.get(i + 1).map(|&content| SectionParseResult {
                    title,
                    content: content.to_string(),
                })
            })
            .collect()
    }

    /// Builds sections from extracted balanced titles.
    fn build_sections_from_balanced(
        content: &str,
        balanced_titles: Vec<(usize, String)>,
    ) -> Vec<SectionParseResult> {
        balanced_titles
            .iter()
            .enumerate()
            .map(|(i, (pos, title))| {
                let section_end = balanced_titles
                    .get(i + 1)
                    .map(|(next_pos, _)| *next_pos)
                    .unwrap_or(content.len());

                let title_end = pos + "\\section".len() + title.len() + 2; // +2 for {}
                let section_content = &content[title_end..section_end];

                SectionParseResult {
                    title: title.clone(),
                    content: section_content.to_string(),
                }
            })
            .collect()
    }

    /// Processes sections and converts them to chunks.
    fn process_sections(sections: Vec<SectionParseResult>) -> Vec<TextChunk> {
        let mut chunks = Vec::new();

        for section in sections {
            if Self::is_appendix_section(&section.title) {
                continue; // Skip this section but continue processing others
            }

            if Self::contains_bibliography(&section.content) {
                continue; // Skip bibliography sections
            }

            let section_chunks =
                Self::parse_section_to_chunks(&section.title, &section.content);
            chunks.extend(section_chunks);
        }

        chunks
    }

    /// Parses section and subsections into flat chunk structure.
    fn parse_section_to_chunks(title: &str, content: &str) -> Vec<TextChunk> {
        Self::parse_section_recursive(title, content, 1, None)
    }

    /// Creates a leaf chunk (no subsections).
    fn create_leaf_chunk(
        title: &str,
        content: &str,
        level: u8,
        parent_id: Option<String>,
    ) -> Vec<TextChunk> {
        let section_id = Self::build_section_id(title, &parent_id);
        let integrated_content = Self::parse_integrated_content(content);

        // Filter out chunks that are too short or have empty titles
        let cleaned_title = Self::clean_title(title);
        let trimmed_content = integrated_content.trim();
        
        // Stricter validation to prevent corrupt chunks
        if trimmed_content.is_empty() 
            || cleaned_title.is_empty() 
            || cleaned_title.trim().is_empty()
            || trimmed_content.len() < 100  // Increased minimum for better quality
            || cleaned_title == "Sección sin título"
            || cleaned_title.chars().all(|c| !c.is_alphanumeric()) {
            return Vec::new();
        }

        vec![TextChunk {
            id: section_id,
            title: cleaned_title,
            content: integrated_content,
            level,
            parent_id,
        }]
    }

    /// Builds hierarchical section ID.
    fn build_section_id(title: &str, parent_id: &Option<String>) -> String {
        match parent_id {
            Some(parent) => format!("{}_{}", parent, Self::generate_chunk_id(title)),
            None => Self::generate_chunk_id(title),
        }
    }

    /// Determines if a chunk should be created based on level and content.
    fn should_create_main_chunk(level: u8, content: &str) -> bool {
        level == 1 || !content.trim().is_empty()
    }

    /// Recursively processes sections up to 3 levels deep.
    fn parse_section_recursive(
        title: &str,
        content: &str,
        level: u8,
        parent_id: Option<String>,
    ) -> Vec<TextChunk> {
        let mut chunks = Vec::new();

        // Seleccionar patrón según el nivel
        let (pattern, next_level) = match level {
            1 => ("\\subsection", 2),
            2 => ("\\subsubsection", 3),
            3 => {
                // Nivel 3: Solo procesar contenido, no buscar más sub-niveles
                return Self::create_leaf_chunk(title, content, level, parent_id);
            }
            _ => return chunks, // No procesar más de 3 niveles
        };

        // Usar parsing balanceado para sub-secciones
        let balanced_subtitles = Self::extract_balanced_title(content, pattern);

        let (sub_parts, sub_titles) = if !balanced_subtitles.is_empty() {
            let mut content_parts = Vec::new();
            let mut title_strs = Vec::new();

            for (i, (pos, title)) in balanced_subtitles.iter().enumerate() {
                // Agregar contenido antes de esta subsección
                if i == 0 {
                    content_parts.push(&content[0..*pos]);
                }

                // Determinar dónde termina esta subsección
                let subsection_end = if i + 1 < balanced_subtitles.len() {
                    balanced_subtitles[i + 1].0
                } else {
                    content.len()
                };

                // Extraer contenido de la subsección
                let title_end = pos + pattern.len() + title.len() + 2;
                if title_end <= subsection_end {
                    content_parts.push(&content[title_end..subsection_end]);
                    title_strs.push(title.clone());
                }
            }

            (content_parts, title_strs)
        } else {
            // No hay subsecciones, todo el contenido es principal
            (vec![content], vec![])
        };

        // Generar ID para esta sección
        let section_id = Self::build_section_id(title, &parent_id);

        // Procesar contenido principal de esta sección
        let main_content = sub_parts[0].to_string();

        // Determinar si se debe crear chunk principal
        let should_create_chunk = Self::should_create_main_chunk(level, &main_content);

        if should_create_chunk {
            let processed_content = if main_content.trim().is_empty() {
                format!("Sección de estructura: {}", title) // Placeholder descriptivo
            } else {
                Self::parse_integrated_content(&main_content)
            };

            chunks.push(TextChunk {
                id: section_id.clone(),
                title: Self::clean_title(title),
                content: processed_content,
                level,
                parent_id: parent_id.clone(),
            });
        }

        // Procesar sub-secciones recursivamente
        let max_subsections = sub_parts.len().saturating_sub(1).min(sub_titles.len());

        for j in 1..=max_subsections {
            let sub_title = &sub_titles[j - 1];
            let sub_content = sub_parts[j];

            // Llamada recursiva para procesar la sub-sección
            let sub_chunks = Self::parse_section_recursive(
                sub_title,
                sub_content,
                next_level,
                Some(section_id.clone()),
            );
            chunks.extend(sub_chunks);
        }

        chunks
    }

    /// Generates unique chunk ID based on title.
    fn generate_chunk_id(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .trim_matches('_')
            .to_string()
    }

    /// Removes LaTeX commands and normalizes content for plagiarism analysis.
    fn remove_latex_commands(text: &str) -> String {
        let cleaned = Self::remove_non_plagiarizable_elements(text);
        Self::normalize_contextual_elements(&cleaned)
    }

    /// Removes non-plagiarizable elements (tables, images, code, equations).
    fn remove_non_plagiarizable_elements(text: &str) -> String {
        let mut cleaned = text.to_string();

        cleaned = TABLE_RE.replace_all(&cleaned, "").to_string();
        cleaned = IMAGE_RE.replace_all(&cleaned, "").to_string();
        cleaned = CODE_RE.replace_all(&cleaned, "").to_string();
        cleaned = COMPLEX_EQUATION_RE.replace_all(&cleaned, "").to_string();
        cleaned = DISPLAY_EQUATION_RE.replace_all(&cleaned, "").to_string();

        cleaned
    }

    /// Normalizes contextually valuable elements.
    fn normalize_contextual_elements(text: &str) -> String {
        let mut cleaned = text.to_string();

        cleaned = INLINE_EQUATION_RE
            .replace_all(&cleaned, "fórmula matemática")
            .to_string();
        cleaned = CITATION_RE
            .replace_all(&cleaned, "referencia bibliográfica")
            .to_string();
        cleaned = REFERENCE_RE.replace_all(&cleaned, "referencia").to_string();
        cleaned = URL_RE.replace_all(&cleaned, "enlace web").to_string();
        cleaned = TEXT_FORMAT_RE.replace_all(&cleaned, "$2").to_string();
        // NOTE: COMMENT_RE removal is now done in strip_comments() at the start of parsing
        // Doing it here again would incorrectly truncate content at escaped percentages (\%)

        cleaned
    }

    /// Integrates section content including normalized lists.
    fn parse_integrated_content(text: &str) -> String {
        let mut content = Self::remove_latex_commands(text);

        content = Self::normalize_itemize(&content);
        content = Self::normalize_enumerate(&content);

        Self::clean_content(&content)
    }

    /// Converts LaTeX itemize lists to bullet points.
    fn normalize_itemize(text: &str) -> String {
        Self::normalize_list_with_pattern(text, &ITEMIZE_RE, |item, _| {
            format!("• {}", item)
        })
    }

    /// Generic method for normalizing LaTeX lists.
    fn normalize_list_with_pattern<F>(
        text: &str,
        pattern: &::regex::Regex,
        formatter: F,
    ) -> String
    where
        F: Fn(&str, usize) -> String,
    {
        let mut result = text.to_string();

        while let Some(caps) = pattern.captures(&result) {
            let full_match = caps.get(0).unwrap().as_str();
            let list_content = caps.get(1).unwrap().as_str();
            let items = Self::extract_list_items(list_content);

            let formatted_items: Vec<String> = items
                .iter()
                .enumerate()
                .map(|(i, item)| formatter(item, i + 1))
                .collect();

            let replacement = if formatted_items.is_empty() {
                String::new()
            } else {
                // Join with spaces instead of newlines for better consistency
                format!(" {} ", formatted_items.join(". "))
            };

            result = result.replace(full_match, &replacement);
        }

        result
    }

    /// Extracts and cleans LaTeX list items.
    fn extract_list_items(list_content: &str) -> Vec<String> {
        list_content
            .split("\\item")
            .skip(1)
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| CLEAN_RE.replace_all(item, "").trim().to_string())
            .filter(|item| !item.is_empty())
            .collect()
    }

    /// Converts LaTeX enumerate lists to numbered lists.
    fn normalize_enumerate(text: &str) -> String {
        Self::normalize_list_with_pattern(text, &ENUMERATE_RE, |item, index| {
            format!("{}. {}", index, item)
        })
    }

    /// Removes LaTeX comments from content.
    fn strip_comments(content: &str) -> String {
        let mut cleaned = String::new();
        let mut in_comment_block = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if in_comment_block {
                if trimmed.starts_with("\\end{comment}") {
                    in_comment_block = false;
                }
                continue;
            }

            if trimmed.starts_with("\\begin{comment}") {
                in_comment_block = true;
                continue;
            }

            // Buscar comentarios, pero evitar % escapado (\%)
            // Check if line contains % and if it's escaped
            let mut line_to_add = line;
            if let Some(pos) = line.find('%') {
                // Check if % is escaped by looking at the byte before
                let is_escaped = if pos > 0 {
                    // Get bytes to check for backslash
                    let bytes = line.as_bytes();
                    bytes.get(pos.saturating_sub(1)) == Some(&b'\\')
                } else {
                    false
                };
                
                if is_escaped {
                    line_to_add = line;  // Keep full line with \%
                } else {
                    line_to_add = &line[..pos];  // Truncate at %
                }
            }
            
            cleaned.push_str(line_to_add);
            cleaned.push('\n');
        }

        cleaned
    }

    /// Final text cleanup and normalization.
    fn clean_content(text: &str) -> String {
        let mut cleaned = CLEAN_RE.replace_all(text, "").to_string();

        // Preserve escaped percentages before removing backslashes
        cleaned = cleaned.replace("\\%", "PERCENT_PLACEHOLDER");
        
        // Remove remaining LaTeX artifacts
        cleaned = cleaned.replace('\\', "");
        cleaned = cleaned.replace('{', "");
        cleaned = cleaned.replace('}', "");
        
        // Restore percentages
        cleaned = cleaned.replace("PERCENT_PLACEHOLDER", "%");

        // Normalize whitespace consistently
        cleaned = cleaned.replace("\n\n", " ");
        cleaned = cleaned.replace("\n", " ");
        cleaned = cleaned.replace("\t", " ");

        // Remove multiple spaces
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ");
        }

        // Final cleanup
        cleaned.trim().to_string()
    }

    /// Extracts section titles handling nested LaTeX commands.
    fn extract_balanced_title(content: &str, pattern: &str) -> Vec<(usize, String)> {
        let mut titles = Vec::new();
        let mut start = 0;

        while let Some(cmd_start) = content[start..].find(pattern) {
            let abs_start = start + cmd_start;
            let after_cmd = abs_start + pattern.len();

            if let Some(title) = Self::extract_balanced_braces(&content[after_cmd..]) {
                let title_len = title.len();
                titles.push((abs_start, title));
                start = after_cmd + title_len + 2; // +2 para las llaves {}
            } else {
                start = after_cmd;
            }
        }

        titles
    }

    /// Extrae contenido entre llaves balanceadas.
    fn extract_balanced_braces(content: &str) -> Option<String> {
        let mut chars = content.chars();

        // Debe empezar con {
        if chars.next() != Some('{') {
            return None;
        }

        let mut result = String::new();
        let mut brace_count = 1;

        for ch in chars {
            if ch == '{' {
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    // Si el título está vacío, asignar placeholder
                    return Some(if result.trim().is_empty() {
                        "Sección sin título".to_string()
                    } else {
                        result
                    });
                }
            }
            result.push(ch);
        }

        None // Llaves no balanceadas
    }



    /// Checks if section content contains bibliography markers.
    fn contains_bibliography(content: &str) -> bool {
        content.contains("\\printbibliography")
    }

    /// Determines if section corresponds to appendices or non-relevant content.
    fn is_appendix_section(title: &str) -> bool {
        let normalized_title = title.trim().to_lowercase();

        let appendix_keywords = [
            "anexos",
            "anexo",
            "apéndices",
            "apéndice",
            "apendices",
            "apendice",
            "bibliografía",
            "bibliografia",
            "referencias",
            "glosario",
            "índice",
            "indice",
        ];

        if normalized_title.starts_with("anexo")
            || normalized_title.starts_with("apéndice")
            || normalized_title.starts_with("apendice")
        {
            return true;
        }

        appendix_keywords.iter().any(|&keyword| {
            normalized_title.contains(keyword) || normalized_title == keyword
        })
    }

    /// Cleans LaTeX commands from section titles.
    pub fn clean_title(title: &str) -> String {
        // Handle empty titles
        if title.trim().is_empty() {
            return "Sección sin título".to_string();
        }

        // Remove text formatting commands
        let cleaned = TEXT_FORMAT_RE.replace_all(title, "$2");

        // Remove any remaining LaTeX commands (simple version)
        let cleaned = cleaned.replace("\\", "").replace("{", "").replace("}", "");

        cleaned.to_string()
    }
}
