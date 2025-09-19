# Epic Description

**Existing System Context:**

- Current relevant functionality: Data-Factory has basic ETL framework with data-engine modules (API, service, DAO, web) and data-terminal configuration interface
- Technology stack: Java/Spring Boot backend (v2.2.10), Maven modules, TailwindCSS frontend with DaisyUI
- Integration points: Existing data-engine execution engine, data-terminal configuration UI, MySQL connectivity infrastructure

**Enhancement Details:**

- What's being added/changed: Core pipeline task types - datasource config, collection tasks (batch/stream), SQL transformation (scheduled/realtime), data sync to external MySQL
- How it integrates: Extends data-engine-service with task management, enhances data-terminal UI with task configuration forms, leverages existing data-engine-dao for database connectivity
- Success criteria: Users can create and execute individual MySQL pipeline tasks independently
