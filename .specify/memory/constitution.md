<!--
Sync Impact Report
==================
Version change: 1.0.0 → 1.1.0
Change type: MINOR - Updated to reflect Data Factory multi-module architecture and migration from BMAD-METHOD to SpecKit

Modified principles:
- Clean Architecture → Updated to reflect multi-module structure (data-terminal, data-engine, data-ai)
- Added specific architectural constraints for Java and Python modules
- Updated runtime guidance section to reference migration from BMAD-METHOD

Added sections:
- Multi-Module Architecture Principles (new principle VI)
- Documentation Standards (new principle VII)
- SpecKit Workflow guidance (replacing BMAD-METHOD references)

Removed sections: N/A

Templates requiring updates:
✅ plan-template.md - Already reviewed, aligns with updated principles
✅ spec-template.md - Already reviewed, aligns with updated principles
✅ tasks-template.md - Already reviewed, aligns with updated principles
✅ command files - Updated to remove BMAD-METHOD references

Follow-up TODOs:
- Consider adding observability/monitoring principles for data-engine workflows in future MINOR version
- Review Java-specific quality standards for data-engine module (potential future addition)
- Review Python-specific quality standards for data-ai module (potential future addition)
-->

# Data Factory Constitution

## Core Principles

### I. Code Quality (NON-NEGOTIABLE)

All code MUST meet these quality standards before merging:

- **Type Safety**:
  - **Rust (data-terminal)**: Leverage Rust's type system fully; no unnecessary `unwrap()` or `expect()` in production code paths. Use proper error handling with `Result<T, E>` and `Option<T>`.
  - **Java (data-engine)**: Use Java's strong typing; avoid raw types, prefer `Optional<T>` over null.
  - **Python (data-ai)**: Use type hints for all public APIs; validate with `mypy` strict mode.
- **Compiler/Linter Warnings**: Zero warnings allowed.
  - **Rust**: All `clippy::pedantic` and `clippy::correctness` warnings MUST be resolved or explicitly allowed with justification.
  - **Java**: All SonarLint/Checkstyle warnings MUST be resolved.
  - **Python**: All `pylint` and `flake8` warnings MUST be resolved.
- **Code Reviews**: Every change requires review by at least one other developer. Reviewer MUST verify adherence to all constitution principles.
- **Documentation**:
  - All public APIs MUST have doc comments following language conventions (Rust doc comments, Javadoc, Python docstrings).
  - Complex private functions MUST include inline comments explaining "why", not "what".
- **Formatting**:
  - **Rust**: Code MUST be formatted with `rustfmt` using project defaults.
  - **Java**: Code MUST be formatted with Google Java Format.
  - **Python**: Code MUST be formatted with `black` and imports sorted with `isort`.
  - No formatting debates allowed.

**Rationale**: High code quality reduces bugs, improves maintainability, and leverages each language's safety guarantees. Consistency enables faster onboarding and easier collaboration across modules.

### II. Testing Standards (NON-NEGOTIABLE)

Test-Driven Development is mandatory for all features:

- **Test-First Approach**: Tests MUST be written before implementation. Tests MUST fail initially, demonstrating they test real functionality.
- **Red-Green-Refactor**: Strictly follow the TDD cycle:
  1. Write failing test (RED)
  2. Implement minimal code to pass (GREEN)
  3. Refactor while keeping tests green (REFACTOR)
- **Test Coverage Targets**:
  - Backend services (all modules): Minimum 80% coverage for business logic
  - Frontend components: Critical user journeys MUST have integration tests
  - All public APIs: MUST have contract tests
  - Data pipeline nodes: MUST have integration tests with real database connections
- **Test Categories**:
  - **Unit tests**: Test individual functions/methods in isolation
  - **Integration tests**: Test component interactions and module boundaries
  - **Contract tests**: Verify API contracts remain stable between modules
  - **End-to-end tests**: Validate complete data workflows across all modules
- **Performance Tests**: Required for endpoints and data pipelines with performance requirements. Tests MUST fail when performance degrades beyond defined thresholds.
- **Test Independence**: Each test MUST be independently runnable. No shared state between tests. Tests MUST clean up resources (databases, files, containers, etc.).

**Rationale**: Tests are executable specifications that prevent regressions, enable confident refactoring, and serve as living documentation. Test-first ensures features are testable by design. Multi-module architecture requires strict contract testing.

### III. User Experience Consistency

Deliver a cohesive, predictable experience across all interfaces:

- **Component Library**: All UI components MUST use daisy-rsx (DaisyUI wrapper). No custom styling that deviates from design system without design approval.
- **Responsive Design**: All pages MUST be usable on desktop (1920x1080) and tablet (768px width) viewports. Mobile support (375px width) is OPTIONAL unless specified in feature requirements.
- **Error Handling**: User-facing errors MUST:
  - Use `dioxus-toast` for non-blocking notifications
  - Provide actionable error messages (what went wrong, what to do next)
  - Never expose internal error details (stack traces, database errors) to users
  - Log full error context server-side for debugging
- **Loading States**: All async operations MUST show loading indicators. No "frozen" UI during data fetching or pipeline execution.
- **Accessibility**: All interactive elements MUST be keyboard accessible. Form inputs MUST have proper labels. Color MUST NOT be the only means of conveying information.
- **Performance Perception**: Initial page load MUST show content within 2 seconds. Interactions MUST provide feedback within 100ms.

**Rationale**: Consistent UX reduces cognitive load, improves user satisfaction, and reduces support burden. Accessibility is both ethical and often legally required. Data platform users need clear feedback during long-running operations.

### IV. Performance Requirements

System MUST meet these performance standards:

- **Backend API Response Times** (data-terminal):
  - Simple queries (single record): p95 < 100ms
  - List queries (up to 100 records): p95 < 200ms
  - Complex queries (joins, aggregations): p95 < 500ms
  - Bulk operations: p95 < 2000ms
- **Data Processing Performance** (data-engine):
  - Batch ETL jobs: Process minimum 10,000 records/second
  - Real-time streaming: Latency p95 < 1 second
  - Pipeline scheduling overhead: < 100ms per task
- **Frontend Performance**:
  - First Contentful Paint (FCP): < 1.5 seconds
  - Time to Interactive (TTI): < 3 seconds
  - WebAssembly bundle size: < 5MB compressed
- **Database Performance**:
  - All queries MUST use proper indexes (verify with `EXPLAIN`)
  - N+1 queries are FORBIDDEN (use eager loading/joins)
  - Connection pooling MUST be configured (min: 5, max: 20 connections per module)
- **Concurrency**:
  - API MUST handle minimum 100 concurrent requests without degradation
  - No blocking operations in request handlers (use async/await properly)
  - Pipeline execution MUST support parallel task execution
- **Resource Limits**:
  - data-terminal backend memory: < 500MB RSS under normal load
  - data-engine memory: < 2GB RSS under normal load
  - data-ai memory: < 1GB RSS under normal load
  - Frontend memory: < 100MB heap per active tab
- **Monitoring**: All performance-critical paths MUST be instrumented with metrics. Performance regressions MUST be detected in CI before merging.

**Rationale**: Performance is a feature. Slow systems frustrate users and limit scale. Data processing platforms require strict performance guarantees to handle large datasets efficiently.

### V. Clean Architecture

Maintain clear separation of concerns and dependency flow:

- **Layer Structure** (data-terminal Backend - Rust):
  - **Routes** (`routes/`): HTTP handlers only. Parse requests, call services, format responses.
  - **Services** (`services/`): Business logic. Orchestrate repositories, implement domain rules.
  - **Repositories** (`repositories/`): Data access only. Translate domain models to/from database.
  - **Models** (`models/`): Data structures. No business logic, only data representation.
  - **Utils** (`utils/`): Stateless helper functions. No dependencies on other layers.
- **Layer Structure** (data-terminal Frontend - Rust/Dioxus):
  - **Pages** (`pages/`): Route components. Compose smaller components, manage page state.
  - **Components** (`components/`): Reusable UI. Self-contained, receive props, emit events.
  - **Models** (`models/`): Data structures matching backend DTOs.
  - **Utils** (`utils/`): Client helpers (validation, HTTP, cookies). No UI logic.
- **Dependency Rules**:
  - Outer layers MAY depend on inner layers (routes → services → repositories)
  - Inner layers MUST NOT depend on outer layers (repositories cannot import routes)
  - Same-level dependencies are FORBIDDEN (services cannot directly call other services without dependency injection)
- **Dependency Injection**:
  - **Rust**: Use `Shaku` for backend DI. All service dependencies MUST be injected, not constructed directly.
  - **Java**: Use Spring Framework DI. All beans MUST be interface-based.
  - **Python**: Use dependency injection pattern (e.g., `dependency-injector` or manual DI).
- **No God Objects**: Classes/modules with >500 lines MUST be reviewed for single responsibility violations. Refactor into smaller, focused units.

**Rationale**: Clean architecture makes code testable, maintainable, and changeable. Clear boundaries enable parallel development and reduce coupling.

### VI. Multi-Module Architecture Principles

Data Factory consists of multiple specialized modules that MUST maintain clear boundaries:

- **Module Separation**:
  - **data-terminal** (Rust): User interface and general platform management (authentication, configuration, metadata).
  - **data-engine** (Java): Core data processing, ETL pipelines, batch/streaming workflows. Leverages Java big data ecosystem.
  - **data-ai** (Python): AI-powered features (agents, knowledge base, tools). Leverages Python AI ecosystem.
  - **devops**: Deployment, infrastructure, middleware management, observability.
- **Inter-Module Communication**:
  - Modules MUST communicate via well-defined REST APIs or message queues (no direct database access across modules).
  - API contracts MUST be versioned and documented (OpenAPI/Swagger).
  - Breaking changes require migration guides and deprecation periods.
- **Module Independence**:
  - Each module MUST be deployable independently.
  - Modules MUST NOT share databases (except through read-only replicas if needed).
  - Shared libraries are FORBIDDEN unless absolutely necessary; prefer API communication.
- **Technology Choices**:
  - Module technology stack MUST align with ecosystem strengths (Java for big data, Python for AI, Rust for performance/safety).
  - New dependencies MUST be justified based on module purpose.
  - Avoid technology duplication (e.g., don't implement ETL in Rust if data-engine handles it).

**Rationale**: Multi-module architecture enables technology specialization, independent deployment, and parallel team development. Clear boundaries prevent coupling and allow each module to leverage its ecosystem's strengths.

### VII. Documentation Standards

Maintain comprehensive, up-to-date documentation:

- **Code Documentation**:
  - All public APIs MUST have doc comments (Rust doc, Javadoc, Python docstrings).
  - Complex algorithms MUST include inline comments explaining logic.
  - Configuration files MUST include inline comments for non-obvious settings.
- **Architecture Documentation** (docs/architecture/):
  - MUST reflect current implementation reality (no aspirational documentation).
  - MUST be updated when architectural changes are made.
  - MUST include diagrams for complex interactions.
- **Feature Documentation** (SpecKit specs):
  - Features MUST be specified before implementation using `/speckit.specify`.
  - Specs MUST include user stories, acceptance criteria, and success metrics.
  - Specs MUST be linked in PRs for traceability.
- **Migration Documentation**:
  - Breaking changes MUST include migration guides.
  - Deprecated features MUST document timeline and alternatives.
  - Database schema changes MUST include migration scripts.
- **User Documentation**:
  - User-facing features MUST have usage documentation.
  - API endpoints MUST be documented in OpenAPI/Swagger.
  - Configuration options MUST be documented with examples.

**Rationale**: Good documentation reduces onboarding time, prevents knowledge silos, and enables asynchronous collaboration. Data platforms require clear documentation for complex workflows and integrations.

## Development Workflow

### SpecKit-Based Development Process

Data Factory uses SpecKit framework (migrated from BMAD-METHOD) for AI-assisted feature development:

1. **Specification**: Create feature spec using `/speckit.specify` command.
   Spec MUST include user stories, acceptance criteria, and success metrics.
2. **Planning**: Generate implementation plan using `/speckit.plan` command.
   Plan MUST pass Constitution Check before proceeding.
3. **Clarification**: Run `/speckit.clarify` if requirements are underspecified.
   Resolve ambiguities before writing code.
4. **Task Generation**: Create task list using `/speckit.tasks` command.
   Tasks MUST be organized by user story priority.
5. **Implementation**: Execute tasks using `/speckit.implement` or manually.
   Follow test-first approach strictly.
6. **Review**: Submit PR with link to spec and plan. Reviewer MUST verify all constitution principles are met.
7. **Validation**: Run `/speckit.analyze` to verify spec/plan/tasks consistency before merging.

### Branching Strategy

- **Main branch**: Always production-ready. Protected. Requires PR approval.
- **Feature branches**: Named `feature/###-feature-name` matching spec folder.
- **Hotfix branches**: Named `hotfix/description` for urgent production fixes.
- **No direct commits**: All changes via PR, even for documentation.

### Commit Standards

- **Format**: `<type>(<scope>): <description>` (Conventional Commits)
- **Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `chore`
- **Scope**: Module/component name (e.g., `terminal/auth`, `engine/etl`, `ai/agents`)
- **Description**: Imperative mood, lowercase, no period, <72 chars
- **Examples**:
  - `feat(terminal/datasource): add MySQL connection pooling support`
  - `fix(engine/pipeline): resolve DAG cycle detection bug`
  - `docs(architecture): update multi-module communication diagram`

## Quality Gates

All gates MUST pass before merging to main:

### Code Quality Gates

- [ ] Formatting passes (Rust: `cargo fmt -- --check`, Java: Google Java Format, Python: `black --check`)
- [ ] Linting passes (Rust: `cargo clippy -- -D warnings`, Java: SonarLint, Python: `pylint && flake8`)
- [ ] Type checking passes (Python: `mypy --strict`)
- [ ] Compilation passes (Rust: `cargo check`, Java: `mvn compile`)
- [ ] No `TODO` or `FIXME` comments without associated tracking issues

### Testing Gates

- [ ] All tests pass (Rust: `cargo test`, Java: `mvn test`, Python: `pytest`)
- [ ] Test coverage meets minimums (80% for services)
- [ ] New features have integration tests for user journeys
- [ ] Contract tests pass for inter-module communication
- [ ] Performance tests pass (if applicable to feature)
- [ ] Manual testing completed for UI changes (screenshot/recording in PR)

### Architecture Gates

- [ ] Follows clean architecture layer rules (no dependency violations)
- [ ] No circular dependencies between modules
- [ ] Dependency injection used for service dependencies
- [ ] Error handling uses proper types (`Result<T, E>`, `Optional`, custom exceptions)
- [ ] Inter-module communication uses versioned APIs only
- [ ] No direct database access across module boundaries

### Documentation Gates

- [ ] Public APIs have doc comments
- [ ] README updated if user-facing changes
- [ ] CHANGELOG.md updated with user-visible changes
- [ ] Architecture docs updated if structural changes
- [ ] Migration guide provided if breaking changes
- [ ] SpecKit spec linked in PR description

### Performance Gates

- [ ] No performance regressions detected (if metrics available)
- [ ] Database queries reviewed for indexes and N+1 issues
- [ ] Bundle size within limits (frontend changes)
- [ ] Load testing completed for high-traffic features
- [ ] Pipeline performance benchmarks pass (data-engine changes)

## Governance

### Authority and Amendments

- **Constitution Authority**: This constitution supersedes all other development practices, coding standards, or verbal agreements. When in doubt, constitution takes precedence.
- **Amendment Process**:
  1. Propose amendment via GitHub issue with justification
  2. Discuss with team (minimum 3 business days for feedback)
  3. Vote requires 2/3 majority of active contributors
  4. Update constitution version following semantic versioning
  5. Update all dependent templates and documentation
  6. Announce changes to team with migration guide if needed
- **Version Semantics**:
  - **MAJOR**: Breaking changes (principle removal, incompatible governance changes)
  - **MINOR**: Additive changes (new principles, expanded guidance)
  - **PATCH**: Clarifications, typo fixes, non-semantic refinements

### Compliance and Enforcement

- **PR Reviews**: All pull requests MUST be reviewed for constitutional compliance. Reviewers use checklist from Quality Gates section.
- **Violation Handling**:
  - Non-compliant code MUST be flagged in review with specific principle reference
  - Author MUST address violations before approval
  - Repeated violations trigger architecture review meeting
- **Complexity Justification**: Violations of simplicity principles (e.g., adding 4th module, complex patterns) MUST be documented in plan.md Complexity Tracking table with explicit justification and rejected alternatives.
- **Regular Audits**: Monthly review of merged PRs for constitutional adherence. Systemic violations trigger constitution amendment discussion.

### Runtime Guidance and Migration Notes

For day-to-day development guidance beyond governance, see:
- **data-terminal**: `data-terminal/CLAUDE.md` (Rust architecture patterns, common tasks)
- **Architecture**: `docs/architecture/` (brownfield analysis, current state documentation)
- **SpecKit**: `.specify/templates/` (workflow templates replacing BMAD-METHOD)

**Migration from BMAD-METHOD**:
- This project previously used BMAD-METHOD framework for documentation and workflow management.
- All BMAD-METHOD documentation has been migrated to SpecKit structure under `.specify/` directory.
- Historical documentation remains in `docs/` for reference but new features MUST use SpecKit workflows.
- BMAD-METHOD epics and stories remain in `docs/epic-*` for historical context.

These guides provide practical examples and shortcuts but MUST NOT contradict constitution principles. If conflict exists, constitution wins.

**Version**: 1.1.0 | **Ratified**: 2025-10-16 | **Last Amended**: 2025-10-25
