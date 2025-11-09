mod models;
mod parser;
mod regex;

pub use models::*;
pub use parser::*;
pub use regex::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_elimination() {
        let latex = r#"
        \section{Resultados}
        Los datos se muestran en la siguiente tabla:
        \begin{table}[h]
            \centering
            \begin{tabular}{|c|c|}
                \hline
                Método & Precisión \\
                \hline
                A & 85\% \\
                B & 92\% \\
                \hline
            \end{tabular}
            \caption{Comparación de métodos}
        \end{table}
        Esta tabla muestra los resultados obtenidos.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // La tabla debe ser eliminada completamente
        assert!(!content.contains("\\begin{table}"));
        assert!(!content.contains("\\end{table}"));
        assert!(!content.contains("tabular"));
        assert!(!content.contains("Método"));
        assert!(!content.contains("Precisión"));

        // Pero el contexto debe preservarse
        assert!(content.contains("Esta tabla muestra los resultados obtenidos"));
    }

    #[test]
    fn test_figure_elimination() {
        let latex = r#"
        \section{Análisis}
        Como se observa en la siguiente figura:
        \begin{figure}[h]
            \centering
            \includegraphics[width=0.8\textwidth]{grafico.png}
            \caption{Evolución temporal de la variable X}
            \label{fig:evolucion}
        \end{figure}
        Los resultados son consistentes.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // La figura debe ser eliminada completamente
        assert!(!content.contains("\\begin{figure}"));
        assert!(!content.contains("\\end{figure}"));
        assert!(!content.contains("includegraphics"));
        assert!(!content.contains("grafico.png"));
        assert!(!content.contains("\\caption"));
        assert!(!content.contains("\\label"));

        // Pero el contexto debe preservarse
        assert!(content.contains("Los resultados son consistentes"));
    }

    #[test]
    fn test_equation_block_elimination() {
        let latex = r#"
        \section{Matemáticas}
        La fórmula fundamental es:
        \begin{equation}
        \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
        \label{eq:gaussian}
        \end{equation}
        Esta ecuación es muy importante.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // La ecuación debe ser eliminada completamente
        assert!(!content.contains("\\begin{equation}"));
        assert!(!content.contains("\\end{equation}"));
        assert!(!content.contains("\\int"));
        assert!(!content.contains("\\sqrt{\\pi}"));
        assert!(!content.contains("\\label{eq:gaussian}"));

        // Pero el contexto debe preservarse
        assert!(content.contains("Esta ecuación es muy importante"));
    }

    #[test]
    fn test_code_elimination() {
        let latex = r#"
        \section{Implementación}
        El código implementado es:
        \begin{lstlisting}[language=Python]
        def calcular_promedio(lista):
            return sum(lista) / len(lista)
        \end{lstlisting}
        Esta función calcula el promedio.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // El código debe ser eliminado completamente
        assert!(!content.contains("\\begin{lstlisting}"));
        assert!(!content.contains("\\end{lstlisting}"));
        assert!(!content.contains("def calcular_promedio"));
        assert!(!content.contains("return sum"));

        // Pero el contexto debe preservarse
        assert!(content.contains("Esta función calcula el promedio"));
    }

    #[test]
    fn test_bibliography_normalization() {
        let latex = r#"
        \section{Introducción}
        Según el estudio de \cite{garcia2019} y como mencionan \citep{lopez2020, martinez2021},
        la metodología es efectiva.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Las referencias deben ser normalizadas
        assert!(!content.contains("\\cite{garcia2019}"));
        assert!(!content.contains("\\citep{lopez2020, martinez2021}"));
        assert!(content.contains("referencia bibliográfica"));
        assert!(content.contains("la metodología es efectiva"));
    }

    #[test]
    fn test_cross_references_normalization() {
        let latex = r#"
        \section{Discusión}
        Como se observa en la Tabla \ref{tab:resultados} y la Ecuación \eqref{eq:principal},
        los resultados son concluyentes.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Las referencias cruzadas deben ser normalizadas
        assert!(!content.contains("\\ref{tab:resultados}"));
        assert!(!content.contains("\\eqref{eq:principal}"));
        assert!(content.contains("referencia"));
        assert!(content.contains("los resultados son concluyentes"));
    }

    #[test]
    fn test_url_normalization() {
        let latex = r#"
        \section{Metodología}
        Consultamos \url{https://ejemplo.com/datos} y el sitio \href{http://test.org}{oficial}
        para obtener información adicional.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Las URLs deben ser normalizadas
        assert!(!content.contains("\\url{https://ejemplo.com/datos}"));
        assert!(!content.contains("\\href{http://test.org}{oficial}"));
        assert!(content.contains("enlace web"));
        assert!(content.contains("para obtener información adicional"));
    }

    #[test]
    fn test_inline_equation_normalization() {
        let latex = r#"
        \section{Análisis}
        La relación entre variables es $y = mx + b$ donde m representa la pendiente.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Las ecuaciones inline deben ser normalizadas
        assert!(!content.contains("$y = mx + b$"));
        assert!(content.contains("fórmula matemática"));
        assert!(content.contains("donde m representa la pendiente"));
    }

    #[test]
    fn test_text_formatting_normalization() {
        let latex = r#"
        \section{Conclusiones}
        El concepto de \textbf{machine learning} es \textit{fundamental} en \textsc{IA moderna}.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // El formato debe ser normalizado
        assert!(!content.contains("\\textbf{"));
        assert!(!content.contains("\\textit{"));
        assert!(!content.contains("\\textsc{"));
        assert!(content.contains("machine learning"));
        assert!(content.contains("fundamental"));
        assert!(content.contains("IA moderna"));
    }

    #[test]
    fn test_lists_normalization() {
        let latex = r#"
        \section{Metodología}
        Los pasos son:
        \begin{enumerate}
            \item Recolectar \textbf{datos} primarios
            \item Aplicar técnicas de \textit{análisis}
            \begin{itemize}
                \item Estadística descriptiva
                \item Inferencia estadística
            \end{itemize}
            \item Interpretar resultados
        \end{enumerate}
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Las listas deben ser normalizadas
        assert!(!content.contains("\\begin{enumerate}"));
        assert!(!content.contains("\\end{enumerate}"));
        assert!(!content.contains("\\begin{itemize}"));
        assert!(!content.contains("\\end{itemize}"));
        assert!(!content.contains("\\item"));
        assert!(!content.contains("\\textbf{"));
        assert!(!content.contains("\\textit{"));

        // El contenido debe preservarse
        assert!(content.contains("Recolectar datos primarios"));
        assert!(content.contains("Aplicar técnicas de análisis"));
        assert!(content.contains("Estadística descriptiva"));
        assert!(content.contains("Inferencia estadística"));
        assert!(content.contains("Interpretar resultados"));
    }

    #[test]
    fn test_academic_content_preservation() {
        let latex = r#"
        \section{Introducción}
        La metodología cualitativa permite una comprensión profunda de los fenómenos estudiados.
        A través de entrevistas semiestructuradas, se obtiene información rica en matices que
        los métodos cuantitativos no pueden capturar completamente.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // El contenido académico debe preservarse completamente
        assert!(content.contains(
            "La metodología cualitativa permite una comprensión profunda"
        ));
        assert!(content.contains("A través de entrevistas semiestructuradas"));
        assert!(
            content.contains(
                "los métodos cuantitativos no pueden capturar completamente"
            )
        );
    }

    #[test]
    fn test_section_titles_with_latex_commands() {
        let latex = r#"
        \section{Análisis de \textit{Performance} en Sistemas \textsc{Distribuidos}}
        El rendimiento es un factor crítico.
        "#;

        let parsed = LaTexParser::parse(latex);
        let title = &parsed.chunks[0].title;

        // Los comandos LaTeX en títulos deben ser limpiados
        assert_eq!(title, "Análisis de Performance en Sistemas Distribuidos");
        assert!(
            parsed.chunks[0]
                .content
                .contains("El rendimiento es un factor crítico")
        );
    }

    #[test]
    fn test_empty_sections() {
        let latex = r#"
        \section{Introducción}
        Contenido normal.

        \subsection{}
        El contenido aparece inmediatamente después sin título explícito.
        "#;

        let parsed = LaTexParser::parse(latex);

        // Debería haber al menos la sección principal
        assert!(parsed.chunks.len() >= 1);
        assert_eq!(parsed.chunks[0].title, "Introducción");

        // Si hay una subsección vacía, debería tener el título por defecto
        if parsed.chunks.len() > 1 {
            assert_eq!(parsed.chunks[1].title, "Sección sin título");
        }
    }

    #[test]
    fn test_mixed_complex_content() {
        let latex = r#"
        \section{Resultados}

        El análisis reveló patrones interesantes \cite{autor2020}. Los datos se presentan en la Tabla \ref{tab:datos}:

        \begin{table}[h]
            \caption{Datos principales}
            \begin{tabular}{cc}
                Variable & Valor \\
                Media & 4.2 \\
            \end{tabular}
        \end{table}

        La ecuación fundamental es:
        $$E = mc^2$$

        Estos hallazgos confirman la hipótesis inicial sobre la relación entre variables.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Elementos técnicos deben eliminarse
        assert!(!content.contains("\\begin{table}"));
        assert!(!content.contains("$$E = mc^2$$"));
        assert!(!content.contains("\\cite{autor2020}"));
        assert!(!content.contains("\\ref{tab:datos}"));

        // Contenido académico debe preservarse
        assert!(content.contains("El análisis reveló patrones interesantes"));
        assert!(content.contains("Estos hallazgos confirman la hipótesis inicial"));
        assert!(content.contains("referencia bibliográfica"));
        assert!(content.contains("referencia"));
    }

    #[test]
    fn test_special_characters_and_accents() {
        let latex = r#"
        \section{Conclusiones}
        La investigación se realizó en colaboración con universidades de México,
        España y otros países de América Latina.
        "#;

        let parsed = LaTexParser::parse(latex);
        let content = &parsed.chunks[0].content;

        // Los caracteres especiales y acentos deben preservarse
        assert!(content.contains("México"));
        assert!(content.contains("España")); // Corregido: era "Español" pero el texto dice "España"
        assert!(content.contains("América Latina"));
    }

    #[test]
    fn test_bibliography_section_elimination() {
        let latex = r#"
        \section{Referencias}
        \bibliography{referencias}

        \section{Conclusiones}
        Los resultados son concluyentes.
        "#;

        let parsed = LaTexParser::parse(latex);

        // La sección de referencias debe ser eliminada
        assert_eq!(parsed.chunks.len(), 1);
        assert_eq!(parsed.chunks[0].title, "Conclusiones");
        assert!(
            parsed.chunks[0]
                .content
                .contains("Los resultados son concluyentes")
        );
    }

    #[test]
    fn test_appendix_elimination() {
        let latex = r#"
        \section{Resultados}
        Los datos muestran tendencias claras.

        \section{Apéndice}
        Aquí va información adicional.
        "#;

        let parsed = LaTexParser::parse(latex);

        // La sección de apéndice debe ser eliminada
        assert_eq!(parsed.chunks.len(), 1);
        assert_eq!(parsed.chunks[0].title, "Resultados");
        assert!(
            parsed.chunks[0]
                .content
                .contains("Los datos muestran tendencias claras")
        );
    }

    #[test]
    fn test_clean_title_function() {
        let result = LaTexParser::clean_title(
            r"Análisis de \textit{Performance} en Sistemas \textsc{Distribuidos}",
        );
        assert_eq!(result, "Análisis de Performance en Sistemas Distribuidos");
    }
}
