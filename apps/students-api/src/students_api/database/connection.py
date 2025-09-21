import os
from collections.abc import Generator

from sqlmodel import Session, SQLModel, create_engine

# URL de conexión a PostgreSQL - se puede obtener de variables de entorno
DATABASE_URL = os.getenv("DATABASE_URL")

# Crear el engine de SQLAlchemy
engine = create_engine(DATABASE_URL, echo=True)


def create_db_and_tables():
    """Crear las tablas en la base de datos"""
    SQLModel.metadata.create_all(engine)


def get_session() -> Generator[Session, None, None]:
    """Dependency para obtener sesión de base de datos"""
    with Session(engine) as session:
        yield session
