#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 12pt,
  lang: "es"
)

#let uct_yellow = rgb("#F2B705")
#let uct_blue = rgb("#00487C")

#let hr = grid(
  columns: (1fr, 1fr),
  align: (left, right),
  line(length: 100%, stroke: 0.5pt + uct_blue ),
  line(length: 100%, stroke: 0.5pt + uct_yellow)
)

#set page(
  margin: (
    x: 2cm,
    y: 2cm
  ),
  header: [
    #grid(
      columns: (1fr, auto),
      align: (left + horizon, right + horizon),
      [*Escuela de Ingeniería en Informática*],
      image("logo.png", scaling: "smooth", width: 18em)
    )
    #hr
    #v(-5em)
  ],
  footer: [
    #v(-1em)
    #align(center)[
      www.uctemuco.cl
    ]
    #v(-0.5em)
    #hr
  ]
)

#v(6em)

#align(center)[
  *AUTORIZACIÓN DE PRÁCTICA DE LA EMPRESA*
]

#v(1.5em)
#align(left)[
  Sr. {{ career_manager }}\
  Carrera de {{ career_name }}\
  Universidad Católica de Temuco
]

Autorizamos a: {{ student_name }}, alumno(a) regular de la carrera {{ career_name }} de la Universidad Católica de Temuco, para que efectúe su *{{ course_name }}* en nuestra empresa o institución:

#v(1em)

#let tab1 = {
  table(
    columns: (1fr, 1fr),
    rows: (2em, 2em, 2em, 2em, 2em, 2em, 4em, 4em),
    align: start,

    [Empresa / Institución], [ {{ enterprise_name }} ],
    [Dirección donde se realizará], [ {{ location }} ],
    [Departamento / Unidad ], [  ],
    [Fono/Anexo ], [ ],
    [Fecha de inicio (Tentativa)], [{{ start_date }}],
    [Fecha de termino (Tentativa)], [{{ end_date }}],
    [Actividades a realizar], [],
    [Beneficios acordados], []
  )
}

#figure(
    tab1,
    kind: table,

) <tab:tab1>
La coordinación de esta Práctica en la empresa/institución estará bajo la supervisión:

#let tab2 = {
  table(
    columns: (1fr, 1fr),
    rows: (2em, 2em, 2em, 2em),
    align: start,
    [Nombre del funcionario], [ {{ supervisor_name }} ],
    [Cargo], [  ],
    [Correo electrónico], [ #raw("{{ supervisor_email }}") ],
    [Teléfono de contacto], [ ],
  )
}
#figure(
    tab2,
    kind: table,
) <tab:tab2>

#v(5em)

#grid(
  columns: (1fr, 1fr),
  align: (left, right),
  align(center)[
    #line(length: 80%, stroke: 0.5pt)\
    #v(-2em)
    Fecha
  ],
  align(center)[
    #line(length: 80%, stroke: 0.5pt)\
    #v(-2em)
    Firma y timbre del supervisor
  ],
)
