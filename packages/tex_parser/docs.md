# Documentación: Parser LaTeX para Detección de Plagios

Este documento explica cómo el parser procesa diferentes elementos LaTeX y por qué toma cada decisión.

## Estrategia General

### Elementos que se ELIMINAN COMPLETAMENTE
Elementos que no aportan al análisis de plagio textual y solo añaden ruido.

### Elementos que se NORMALIZAN
Elementos con valor contextual pero que deben estandarizarse para comparación efectiva.

### Elementos que se PRESERVAN
Contenido académico principal que es relevante para detección de similitud.

## Casos Específicos de Procesamiento

## Procesamiento Jerárquico

### Niveles Soportados

**Nivel 1:** `\section{}`
**Nivel 2:** `\subsection{}`
**Nivel 3:** `\subsubsection{}`

### Algoritmo de Parsing

1. **Detección de secciones** con regex patterns
2. **Extracción de títulos** con parsing balanceado de llaves
3. **Procesamiento recursivo** del contenido
4. **Normalización estratégica** de elementos LaTeX
5. **Generación de chunks** con metadatos jerárquicos

## Manejo de Elementos LaTeX

### ELIMINACIÓN COMPLETA

#### Tablas
**Entrada:**
```latex
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
```
**Resultado:** Eliminado completamente

**Razón:** Las tablas contienen datos estructurados específicos. El plagio de tablas requiere análisis especializado diferente al texto narrativo.

#### Figuras e Imágenes
**Entrada:**
```latex
\begin{figure}[h]
    \centering
    \includegraphics[width=0.8\textwidth]{grafico.png}
    \caption{Evolución temporal de la variable X}
    \label{fig:evolucion}
\end{figure}
```
**Resultado:** Eliminado completamente

**Razón:** Las imágenes no pueden ser analizadas como texto. Los captions suelen ser descriptivos muy específicos y no representan plagio conceptual.

#### Ecuaciones en Bloque
**Entrada:**
```latex
\begin{equation}
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
\label{eq:gaussian}
\end{equation}
```
**Resultado:** Eliminado completamente

**Razón:** Las fórmulas matemáticas son conocimiento establecido o demostraciones técnicas que no constituyen plagio textual académico.

#### Código Fuente
**Entrada:**
```latex
\begin{lstlisting}[language=Python]
def calcular_promedio(lista):
    return sum(lista) / len(lista)
\end{lstlisting}
```
**Resultado:** Eliminado completamente

**Razón:** El código requiere análisis de plagio especializado con herramientas diferentes.

#### Bibliografía y Appendices
**Entrada:**
```latex
\section{Referencias}
\bibliography{referencias}

\appendix
\section{Código Completo}
```
**Resultado:** Secciones ignoradas completamente

**Razón:** Las referencias son citas requeridas, no contenido original. Los apéndices suelen contener material auxiliar no central al análisis.

### NORMALIZACIÓN A TEXTO ESTÁNDAR

#### Referencias Bibliográficas
**Entrada:**
```latex
Según el estudio de \cite{garcia2019} y como mencionan \citep{lopez2020, martinez2021}...
```
**Resultado:**
```
Según el estudio de referencia bibliográfica y como mencionan referencia bibliográfica...
```

**Razón:** Preserva el flujo argumentativo pero elimina referencias específicas que varían entre documentos sin afectar el contenido conceptual.

#### Referencias Cruzadas
**Entrada:**
```latex
Como se observa en la Tabla \ref{tab:resultados} y la Ecuación \eqref{eq:principal}...
```
**Resultado:**
```
Como se observa en la referencia y la referencia...
```

**Razón:** Mantiene la estructura argumentativa sin elementos específicos del documento que no afectan la similitud conceptual.

#### URLs y Enlaces Web
**Entrada:**
```latex
Consultamos \url{https://ejemplo.com/datos} y el sitio \href{http://test.org}{oficial}...
```
**Resultado:**
```
Consultamos enlace web y el sitio enlace web...
```

**Razón:** Las URLs específicas varían pero el hecho de referenciar fuentes web es conceptualmente similar.

#### Ecuaciones Inline
**Entrada:**
```latex
La relación entre variables es $y = mx + b$ donde m representa la pendiente.
```
**Resultado:**
```
La relación entre variables es fórmula matemática donde m representa la pendiente.
```

**Razón:** Preserva el contexto matemático sin la fórmula específica, enfocándose en la explicación conceptual.

#### Formato de Texto
**Entrada:**
```latex
El concepto de \textbf{machine learning} es \textit{fundamental} en \textsc{IA moderna}.
```
**Resultado:**
```
El concepto de machine learning es fundamental en IA moderna.
```

**Razón:** El contenido semántico es lo relevante, no el formato visual específico.

#### Listas y Enumeraciones
**Entrada:**
```latex
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
```
**Resultado:**
```
Los pasos son:
• Recolectar datos primarios
• Aplicar técnicas de análisis
• Estadística descriptiva
• Inferencia estadística
• Interpretar resultados
```

**Razón:** Convierte a formato estándar preservando la información estructural pero eliminando variaciones de formato específicas.

### PRESERVACIÓN COMPLETA

#### Párrafos Académicos Principales
**Entrada:**
```latex
La metodología cualitativa permite una comprensión profunda de los fenómenos estudiados. 
A través de entrevistas semiestructuradas, se obtiene información rica en matices que 
los métodos cuantitativos no pueden capturar completamente.
```
**Resultado:** Preservado exactamente igual

**Razón:** Este es el contenido académico central que debe ser analizado para detección de plagio.

#### Argumentaciones y Análisis
**Entrada:**
```latex
Los resultados sugieren una correlación significativa entre las variables analizadas. 
Esta relación implica que las intervenciones preventivas pueden ser más efectivas 
cuando se implementan durante las primeras etapas del proceso.
```
**Resultado:** Preservado exactamente igual

**Razón:** Las argumentaciones académicas son el núcleo del análisis de plagio conceptual.

#### Descripciones Metodológicas
**Entrada:**
```latex
Se utilizó un diseño experimental de tipo transversal con una muestra de 150 participantes 
seleccionados mediante muestreo aleatorio estratificado según edad y género.
```
**Resultado:** Preservado exactamente igual

**Razón:** Las descripciones metodológicas contienen el conocimiento específico del investigador.

## Casos Especiales

### Títulos con Comandos LaTeX
**Entrada:**
```latex
\section{Análisis de \textit{Performance} en Sistemas \textsc{Distribuidos}}
```
**Resultado:** Título preservado como "Análisis de Performance en Sistemas Distribuidos"

**Razón:** Los títulos deben preservar su contenido semántico pero sin comandos de formato.

### Secciones Vacías
**Entrada:**
```latex
\subsection{}
El contenido aparece inmediatamente después sin título explícito.
```
**Resultado:** Título se convierte en "Sección sin título"

**Razón:** Se debe mantener la estructura jerárquica incluso sin título explícito.

### Contenido Mixto Complejo
**Entrada:**
```latex
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
```

**Resultado:**
```
Título: Resultados
Contenido: El análisis reveló patrones interesantes referencia bibliográfica. Los datos se presentan en la referencia:

Estos hallazgos confirman la hipótesis inicial sobre la relación entre variables.
```

**Razón:** Se preserva la narrativa académica eliminando elementos técnicos específicos.

### Caracteres Especiales y Acentos
**Entrada:**
```latex
La investigación se realizó en colaboración con universidades de México, 
España y otros países de América Latina.
```
**Resultado:** Preservado exactamente igual incluyendo acentos

**Razón:** Los caracteres especiales y acentos son parte del contenido semántico académico.
