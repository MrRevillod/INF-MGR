import uuid

from sqlmodel import Field, SQLModel


class Student(SQLModel, table=True):
    """Modelo de Estudiante"""

    __tablename__ = "students"

    id: str | None = Field(default_factory=lambda: str(uuid.uuid4()), primary_key=True)
    rut: str = Field(unique=True, index=True)
    name: str
    email: str = Field(unique=True)
    register: str


class Course(SQLModel, table=True):
    """Modelo de Curso"""

    __tablename__ = "courses"

    id: str | None = Field(default_factory=lambda: str(uuid.uuid4()), primary_key=True)
    year: int
    course_code: str = Field(unique=True, index=True)


class Enrollment(SQLModel, table=True):
    """Modelo de Matrícula - Tabla intermedia entre Student y Course"""

    __tablename__ = "enrollments"

    id: str | None = Field(default_factory=lambda: str(uuid.uuid4()), primary_key=True)
    student_id: str = Field(foreign_key="students.id")
    course_id: str = Field(foreign_key="courses.id")
