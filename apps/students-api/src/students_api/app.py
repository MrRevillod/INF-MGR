from fastapi import Depends, FastAPI, HTTPException
from sqlmodel import Session, select

from .database.connection import create_db_and_tables, get_session
from .database.models import Course, Enrollment, Student
from .database.seed import create_sample_data
from .security import verify_api_key

app = FastAPI(
    title="Students API",
    description="API for managing students, courses and enrollments",
    version="1.0.0",
)


@app.on_event("startup")
def on_startup():
    """Ejecutar al iniciar la aplicación"""
    create_db_and_tables()
    create_sample_data()


@app.get("/health")
async def health():
    return {"status": "ok", "database": "connected"}


@app.get("/uct/courses/{course_code}/{year}/student")
def get_students_by_course_and_year(
    course_code: str,
    year: int,
    session: Session = Depends(get_session),
    _: str = Depends(verify_api_key)  # Solo para validación, no necesitamos el valor
):
    """Obtener estudiantes matriculados en un curso específico de un año específico"""

    course = session.exec(
        select(Course).where(Course.course_code == course_code)
    ).first()

    if not course:
        raise HTTPException(status_code=404, detail="Course not found")

    if course.year != year:
        raise HTTPException(
            status_code=400,
            detail=(
                f"Course year ({course.year}) does not match requested year ({year})"
            ),
        )

    enrollments = session.exec(
        select(Enrollment).where(Enrollment.course_id == course.id)
    ).all()

    if not enrollments:
        return {
            "course_code": course.course_code,
            "year": year,
            "total_students": 0,
            "students": [],
        }

    student_ids = [enrollment.student_id for enrollment in enrollments]
    students = session.exec(select(Student).where(Student.id.in_(student_ids))).all()

    return {
        "course_code": course.course_code,
        "year": year,
        "total_students": len(students),
        "students": students,
    }
