from sqlmodel import Session, select

from .connection import engine
from .models import Course, Enrollment, Student


def create_sample_data():
    """Crear datos de ejemplo para estudiantes, cursos y matrículas"""

    with Session(engine) as session:
        existing_students = session.exec(select(Student)).first()
        if existing_students:
            print("Los datos de ejemplo ya existen.")
            return

        students_data = [
            {
                "rut": "12345678-9",
                "name": "Juan Carlos Pérez González",
                "email": "juan.perez@estudiante.uct.cl",
                "register": "2021001",
            },
            {
                "rut": "98765432-1",
                "name": "María José González López",
                "email": "maria.gonzalez@estudiante.uct.cl",
                "register": "2021002",
            },
            {
                "rut": "11223344-5",
                "name": "Carlos Eduardo Rodríguez Silva",
                "email": "carlos.rodriguez@estudiante.uct.cl",
                "register": "2020001",
            },
            {
                "rut": "55667788-9",
                "name": "Ana Sofía Martínez Torres",
                "email": "ana.martinez@estudiante.uct.cl",
                "register": "2022001",
            },
            {
                "rut": "33445566-7",
                "name": "Diego Alejandro Sánchez Morales",
                "email": "diego.sanchez@estudiante.uct.cl",
                "register": "2021003",
            },
        ]

        courses_data = [
            {"year": 2024, "course_code": "INF2806"},
            {"year": 2024, "course_code": "INF2812"},
            {"year": 2024, "course_code": "MAT1204"},
            {"year": 2024, "course_code": "INF3807"},
            {"year": 2024, "course_code": "ING1013"},
        ]

        students = []
        for student_data in students_data:
            student = Student(**student_data)
            session.add(student)
            students.append(student)

        courses = []
        for course_data in courses_data:
            course = Course(**course_data)
            session.add(course)
            courses.append(course)

        session.commit()

        for student in students:
            session.refresh(student)
        for course in courses:
            session.refresh(course)

        enrollments_data = [
            {"student_id": students[0].id, "course_id": courses[0].id},
            {"student_id": students[0].id, "course_id": courses[3].id},
            {"student_id": students[1].id, "course_id": courses[1].id},
            {"student_id": students[1].id, "course_id": courses[2].id},
            {"student_id": students[2].id, "course_id": courses[3].id},
            {"student_id": students[2].id, "course_id": courses[0].id},
            {"student_id": students[2].id, "course_id": courses[4].id},
            {"student_id": students[3].id, "course_id": courses[2].id},
            {"student_id": students[3].id, "course_id": courses[4].id},
            {"student_id": students[4].id, "course_id": courses[1].id},
            {"student_id": students[4].id, "course_id": courses[0].id},
        ]

        for enrollment_data in enrollments_data:
            enrollment = Enrollment(**enrollment_data)
            session.add(enrollment)

        session.commit()
        print("Datos de ejemplo creados exitosamente!")
        print(f"   - {len(students)} estudiantes")
        print(f"   - {len(courses)} cursos")
        print(f"   - {len(enrollments_data)} matrículas")


if __name__ == "__main__":
    create_sample_data()
