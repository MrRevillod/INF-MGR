# Roadmap de Desarrollo - INF-MGR Server

## Estado Actual ✅
- [x] Arquitectura base implementada
- [x] Módulos Users, Courses, Inscriptions
- [x] Base de datos con migraciones
- [x] Validaciones de DTOs
- [x] Configuración Docker

## Fase 1: Autenticación y Seguridad (Semanas 1-2)

### Sprint 1.1: Sistema de Auth
- [ ] Implementar endpoints de login/logout
- [ ] JWT middleware y refresh tokens
- [ ] Hash de passwords en Users service
- [ ] Middleware de autorización por roles

### Sprint 1.2: Seguridad
- [ ] Rate limiting
- [ ] CORS refinado por ambiente
- [ ] Input sanitization
- [ ] Error handling global

## Fase 2: Módulo de Prácticas (Semanas 3-4)

### Sprint 2.1: Practices CRUD
- [ ] Crear módulo practices completo
- [ ] Endpoints para gestión de prácticas
- [ ] Validaciones de empresa y fechas
- [ ] Integración con inscriptions

### Sprint 2.2: Company Integration
- [ ] Sistema de notificaciones a empresas
- [ ] Formularios de evaluación externa
- [ ] Tracking de comunicaciones

## Fase 3: Reportes y Bitácoras (Semanas 5-7)

### Sprint 3.1: Reports Module
- [ ] Sistema de bitácoras semanales
- [ ] Upload de informes finales
- [ ] Templates de documentos
- [ ] Versionado de reportes

### Sprint 3.2: Document Generation
- [ ] Generación automática de PDFs
- [ ] Sistema de templates con Tera
- [ ] Email notifications con attachments

## Fase 4: Workflow y Estados (Semanas 8-9)

### Sprint 4.1: State Management
- [ ] Sistema de estados de práctica
- [ ] Transiciones automáticas
- [ ] Deadlines y recordatorios

### Sprint 4.2: Evaluation System
- [ ] Cálculo automático de notas
- [ ] Dashboard de progreso
- [ ] Exportación de datos

## Backlog Futuro
- [ ] Dashboard analytics
- [ ] File management avanzado
- [ ] API versioning
- [ ] Cache layer con Redis
- [ ] Background jobs
- [ ] API documentation con OpenAPI
