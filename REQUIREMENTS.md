# Requisitos del sistema de gestión de prácticas y tesis universitarias

## Descripción del sistema

La Dirección de la carrera de Ingeniería Civil en Informática de una universidad requiere una plataforma web destinada a la gestión integral de las prácticas iniciales y profesionales de sus estudiantes.

El propósito es desarrollar un sistema que facilite la comunicación, el acceso a la información y la gestión documental entre los distintos actores involucrados en el proceso: estudiantes, profesor encargado de la asignatura, jefe de carrera, secretarias y coordinador de prácticas. Se busca centralizar las interacciones en una sola plataforma, optimizando el flujo de trabajo, mejorando la organización y ofreciendo una experiencia más intuitiva y eficiente. La generación automática de documentos y el seguimiento claro de cada etapa permitirán que todos los usuarios participen del proceso de manera ordenada y efectiva.

## Flujo actual del proceso de práctica

1. **Inscripción**: El estudiante inscribe el ramo correspondiente a la práctica (inicial o profesional).

2. **Asignación docente**: Se designa un profesor encargado de la asignatura de práctica.

3. **Inducción al proceso**: El coordinador de prácticas realiza una sesión introductoria con el estudiante, donde se explican las etapas del proceso y se entregan los siguientes documentos:

    - Autorización de práctica
    - Formato de bitácora semanal
    - Formato de informe de práctica
    - Formulario de inscripción de práctica
    - Pauta de evaluación para la empresa

4. **Entrega de documentos iniciales**: El estudiante debe completar y enviar los documentos requeridos dentro del plazo establecido por el coordinador. Esta entrega se realiza vía correo electrónico, con copia a:

    - Secretaría
    - Coordinador de prácticas
    - Profesor encargado de la asignatura

5. **Bitácoras semanales**: Una vez iniciada la práctica, el estudiante debe redactar una bitácora semanal donde se detallen las actividades realizadas (idealmente de lunes a viernes). Esta bitácora debe enviarse semanalmente al coordinador de prácticas y al profesor de la asignatura.

6. **Informe final de práctica**: Al finalizar la práctica, el estudiante dispone de un plazo de dos semanas para redactar y entregar el informe final al coordinador de prácticas.

7. **Evaluación académica**: El coordinador recibe y revisa todos los documentos (bitácoras e informe final), evaluando el desempeño del estudiante de acuerdo con las rúbricas establecidas.

8. **Evaluación por parte de la empresa**: El tutor o encargado del estudiante en la empresa debe completar la pauta de evaluación entregada al inicio del proceso. Esta evaluación debe ser enviada directamente a la secretaría y al coordinador de prácticas.

9. **Cierre del proceso**: Una vez recopiladas todas las evaluaciones, el coordinador de prácticas informa la nota final al profesor encargado, quien se encarga de ingresarla oficialmente en el portal del estudiante, dando por finalizada la asignatura.

## Mejora planteada

Se desarrollará una plataforma web con distintos perfiles de acceso, diseñados según los roles involucrados en el proceso de práctica:

- Estudiante en práctica
- Docente encargado de la asignatura
- Coordinador de prácticas
- Secretaría
- Administración

Cada perfil contará con funcionalidades específicas, limitadas según el tipo de usuario, lo que permitirá transformar el proceso en una experiencia más clara e intuitiva, tanto para los estudiantes como para el personal académico y administrativo de la carrera.

A partir de esta base, el nuevo flujo de trabajo sería el siguiente:

1. Al momento de la inscripción de asignaturas, el encargado de la plataforma generará automáticamente los perfiles de los estudiantes en práctica a partir de la lista de inscritos. Una vez iniciado el semestre, los estudiantes recibirán en su correo institucional una introducción al proceso junto con sus credenciales de acceso.

2. Los estudiantes accederán a una vista web personalizada donde se les da la bienvenida a la asignatura y se les explica el funcionamiento del sistema. Además, contarán con una sección de recursos donde podrán consultar las rúbricas de evaluación y otros documentos relevantes.

3. Simultáneamente, el docente encargado será incorporado al sistema con su respectivo perfil y un curso asociado que incluye el listado de estudiantes inscritos. Desde allí podrá acceder a las bitácoras semanales, informes de práctica y demás documentos entregados por los estudiantes.

4. El coordinador de prácticas tendrá acceso administrativo a los cursos asignados, con privilegios de edición. Podrá revisar y calificar los documentos entregados por los estudiantes. El cálculo de la nota final se realizará automáticamente, tomando en cuenta los porcentajes definidos para cada componente de la asignatura.

5. El estudiante podrá notificar el inicio de su proceso de práctica completando un formulario con los datos de la empresa. A partir de esta información, el sistema generará automáticamente los documentos oficiales correspondientes y enviará notificaciones por correo electrónico tanto a los responsables internos como a la empresa, incluyendo las instrucciones del proceso.

6. Estas instrucciones contendrán un enlace válido para que la empresa complete la evaluación del estudiante. Este mecanismo permite establecer una comunicación directa entre los actores, sin depender del intercambio manual de correos.

7. La bitácora semanal dejará de ser un archivo descargable y pasará a gestionarse completamente en línea. El estudiante podrá redactar y editar su bitácora directamente en la plataforma, mediante una interfaz sencilla que permitirá ingresar las actividades desarrolladas día a día. Esta bitácora será privada y solo será visible para el estudiante, su profesor guía, el coordinador de prácticas, la secretaría y la administración. Otros estudiantes no tendrán acceso a ella.

8. Al finalizar la práctica, el estudiante deberá subir a la plataforma el informe final en formato PDF. Adicionalmente, deberá adjuntar un enlace en modo solo lectura al documento original si lo está desarrollando en una plataforma externa como Overleaf, Word Online o Google Docs. Esto permitirá al equipo docente verificar la autoría y evolución del documento si es necesario.

9. El sistema gestionará el seguimiento mediante un sistema de etiquetas o estados visibles en el perfil de cada estudiante, tales como: \emph{No iniciado}, \emph{En curso}, \emph{Informe pendiente}, \emph{Evaluación en revisión}, \emph{Completado}, entre otros. Esto facilitará el monitoreo del progreso de cada práctica tanto para los docentes como para la administración.

10. Una vez completado el proceso, las tres evaluaciones finales estarán disponibles en la plataforma. El sistema calculará el promedio automáticamente y notificará al docente encargado sobre la nueva actualización del estado del estudiante, para que proceda a registrar la nota final en el portal académico correspondiente.

11. Todos los registros del proceso —incluyendo bitácoras, informes, evaluaciones y comunicaciones— quedarán almacenados de forma permanente. El historial de cada estudiante será accesible incluso después del cierre del semestre, lo que permitirá realizar seguimientos, auditorías y respaldos en cualquier momento futuro.
