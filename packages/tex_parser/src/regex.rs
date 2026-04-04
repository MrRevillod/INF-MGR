use regex::Regex;
use std::sync::LazyLock;

// Secciones jerárquicas (usar parsing balanceado en parser)
pub static SECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\section\*?\{([^}]+)\}").unwrap());

pub static SUBSECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\subsection\*?\{([^}]+)\}").unwrap());

pub static SUBSUBSECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\subsubsection\*?\{([^}]+)\}").unwrap());

// ❌ ELEMENTOS A ELIMINAR (no relevantes para plagios)
pub static TABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{(table|tabular|tabularx|longtable|supertabular|array)\*?\}.*?\\end\{(table|tabular|tabularx|longtable|supertabular|array)\*?\}").unwrap()
});

pub static IMAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?s)\\begin\{figure\}.*?\\end\{figure\}|\\includegraphics(\[.*?\])?\{[^}]+\}",
    )
    .unwrap()
});

pub static CODE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{(verbatim|lstlisting|minted|code|algorithm)\*?\}.*?\\end\{(verbatim|lstlisting|minted|code|algorithm)\*?\}").unwrap()
});

// Ecuaciones complejas (eliminar completamente)
pub static COMPLEX_EQUATION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{(equation|align|gather|multline|cases|eqnarray|split)\*?\}.*?\\end\{(equation|align|gather|multline|cases|eqnarray|split)\*?\}").unwrap()
});

pub static DISPLAY_EQUATION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\\[.*?\\\]|\$\$[^$]*\$\$").unwrap());

// 🔄 ELEMENTOS A NORMALIZAR (preservar semántica, limpiar formato)
// Ecuaciones inline simples - preservar en texto
pub static INLINE_EQUATION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$([^$]{1,30})\$").unwrap() // Solo ecuaciones inline cortas
});

// Referencias y citas - convertir a texto descriptivo
pub static CITATION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\cite\{([^}]+)\}").unwrap());

pub static REFERENCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\ref\{([^}]+)\}").unwrap());

pub static URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\url\{([^}]+)\}").unwrap());

// Formato de texto - preservar contenido, eliminar comandos
pub static TEXT_FORMAT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\(textbf|textit|textsc|emph|underline|texttt)\{([^}]*)\}").unwrap()
});

// Listas - usar normalización especial
pub static LIST_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{(itemize|enumerate)\}.*?\\end\{(itemize|enumerate)\}")
        .unwrap()
});

pub static ITEMIZE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)\\begin\{itemize\}(.*?)\\end\{itemize\}").unwrap());

pub static ENUMERATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\\begin\{enumerate\}(.*?)\\end\{enumerate\}").unwrap()
});

// Comentarios LaTeX
pub static COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)%.*$").unwrap());

// Limpieza final - comandos restantes (menos agresiva)
pub static CLEAN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\[a-zA-Z]+\*?(\[[^\]]*\])?\{[^}]*\}|\\[a-zA-Z]+|\\begin\{[^}]+\}|\\end\{[^}]+\}").unwrap()
});
