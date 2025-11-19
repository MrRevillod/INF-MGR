# Sistema de gestión de prácticas y tesis universitarias

Este repositorio contiene el código fuente de un sistema de gestión de prácticas y tesis universitarias. El sistema está diseñado para facilitar la administración de las prácticas y tesis de los estudiantes, permitiendo a los administradores gestionar las solicitudes, asignaciones y seguimientos de manera eficiente.

[![Documentación con Deepwiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/MrRevillod/INF-MGR)


## Diagramas del sistema

### Entidad relación (ERD)
![Diagrama ERD](https://raw.githubusercontent.com/MrRevillod/INF-MGR/refs/heads/development/.diagrams/erd.png)

### Cola de eventos (Event Queue)

```mermaid
---
config:
  theme: mc
  look: classic
---
graph LR
    subgraph "Application Layer"
        SA[Service A]
        SB[Service B]
    end
    subgraph "Event Queue System"
        EQ[Event Queue<br/>Publisher]
        CH{Message Channel<br/>mpsc}
        ES[Event Subscriber<br/>Consumer]
    end
    subgraph "Event Handlers"
        M[Mailer]
        P[Printer]
    end
    subgraph "Parallel Processing"
        T1[Task 1]
        T2[Task 2]
        T3[Task N]
    end
    SA -->|publish event| EQ
    SB -->|publish event| EQ
    EQ --> CH
    CH --> ES
    ES --> T1
    ES --> T2
    ES --> T3
    T1 --> M
    T2 --> P
    T3 --> M
    classDef service fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef queue fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef handler fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef task fill:#f1f8e9,stroke:#33691e,stroke-width:1px
    class SA,SB service
    class EQ,CH,ES queue
    class M,P handler
    class T1,T2,T3 task

```

## Instalación y ejecución

Para instalar y ejecutar el sistema, sigue estos pasos de ahora:

1. Clona el repositorio:
   ```bash
   git clone
   ```

2. Instala docker y docker compose.

3. Navega al directorio del proyecto:
   ```bash
   cd INF-MGR/
   ```
4. Crea una copia del archivo `.env.example` y renómbralo a `.env`:
   ```bash
   cp .env.example .env
   ```
   Asegúrate de configurar las variables de entorno en el archivo `.env` según tus necesidades y preferencias.

5. Construye y levanta los contenedores:
   ```bash
    docker compose up
    ```

6. Accede al sistema a través de tu navegador en `http://localhost/`.

## Configuración del entorno de desarrollo

Para configurar el entorno de desarrollo, asegúrate de tener instaladas las siguientes herramientas:

- Node.js LTS [https://nodejs.org/en](https://nodejs.org/en)
- Docker y Docker Compose [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/)
- Rust stable [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Just [https://github.com/casey/just](https://github.com/casey/just)

- Instalar dependencias locales de Node.js:
   ```bash
   cd ruta-al-repositorio/ && just install
   ```

4. Crea una copia del archivo `.env.example` y renómbralo a `.env`:
   ```bash
   cp .env.example .env
   ```
   Asegúrate de configurar las variables de entorno en el archivo `.env` según tus necesidades.
