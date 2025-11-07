use regex_lite::Regex;
use std::sync::LazyLock;

pub static SECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\section\*?\{([^}]+)\}").unwrap());

pub static SUBSECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\subsection\*?\{([^}]+)\}").unwrap());

pub static TABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{tabularx?\}.*?\\end\{tabularx?\}").unwrap()
});

pub static IMAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?s)\\includegraphics(\[.*?\])?\{([^}]+)\}(?:.*?\\caption\{([^}]*)\})?",
    )
    .unwrap()
});

pub static LIST_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{(itemize|enumerate)\}.*?\\end\{(itemize|enumerate)\}")
        .unwrap()
});

pub static EQUATION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(\$\$[^$]+\$\$)|(\$[^\$]+\$)|(\\\[.*?\\\])|(?s)\\begin\{equation\}.*?\\end\{equation\}",
    )
    .unwrap()
});

pub static CLEAN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\\[a-zA-Z]+\*?\{[^}]*\}|\\[a-zA-Z]+|\\begin\{[^}]+\}|\\end\{[^}]+\}",
    )
    .unwrap()
});
