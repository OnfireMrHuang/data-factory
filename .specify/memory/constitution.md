<!--
Sync Impact Report
==================
Version change: 0.0.0 → 1.0.0
Change type: MAJOR - Initial constitution creation with complete governance framework

Added sections:
- Core Principles (5 principles: Code Quality, Testing Standards, User Experience Consistency,
  Performance Requirements, Clean Architecture)
- Development Workflow
- Quality Gates
- Governance

Modified principles: N/A (initial creation)
Removed sections: N/A (initial creation)

Templates requiring updates:
✅ plan-template.md - Constitution Check section already present, aligns with principles
✅ spec-template.md - User scenarios and requirements sections align with principles
✅ tasks-template.md - Test-first approach and task organization align with principles
⚠ command files - No updates needed (no agent-specific references found)

Follow-up TODOs: None
-->

# Data Factory Constitution

## Core Principles

### I. Code Quality (NON-NEGOTIABLE)

All code MUST meet these quality standards before merging:

- **Type Safety**: Leverage Rust's type system fully; no unnecessary `unwrap()` or `expect()`
  in production code paths. Use proper error handling with `Result<T, E>` and `Option<T>`.
- **Compiler Warnings**: Zero warnings allowed. All `clippy::pedantic` and
  `clippy::correctness` warnings MUST be resolved or explicitly allowed with justification.
- **Code Reviews**: Every change requires review by at least one other developer.
  Reviewer MUST verify adherence to all constitution principles.
- **Documentation**: All public APIs MUST have doc comments following Rust conventions.
  Complex private functions MUST include inline comments explaining "why", not "what".
- **Formatting**: Code MUST be formatted with `rustfmt` using project defaults.
  No formatting debates allowed.

**Rationale**: High code quality reduces bugs, improves maintainability, and leverages
Rust's safety guarantees. Consistency enables faster onboarding and easier collaboration.

### II. Testing Standards (NON-NEGOTIABLE)

Test-Driven Development is mandatory for all features:

- **Test-First Approach**: Tests MUST be written before implementation. Tests MUST fail
  initially, demonstrating they test real functionality.
- **Red-Green-Refactor**: Strictly follow the TDD cycle:
  1. Write failing test (RED)
  2. Implement minimal code to pass (GREEN)
  3. Refactor while keeping tests green (REFACTOR)
- **Test Coverage Targets**:
  - Backend services: Minimum 80% coverage for business logic
  - Frontend components: Critical user journeys MUST have integration tests
  - All public APIs: MUST have contract tests
- **Test Categories**:
  - **Unit tests**: Test individual functions/methods in isolation (`cargo test`)
  - **Integration tests**: Test component interactions (`tests/integration/`)
  - **Contract tests**: Verify API contracts remain stable (`tests/contract/`)
- **Performance Tests**: Required for endpoints with performance requirements.
  Tests MUST fail when performance degrades beyond defined thresholds.
- **Test Independence**: Each test MUST be independently runnable. No shared state
  between tests. Tests MUST clean up resources (databases, files, etc.).

**Rationale**: Tests are executable specifications that prevent regressions, enable
confident refactoring, and serve as living documentation. Test-first ensures features
are testable by design.

### III. User Experience Consistency

Deliver a cohesive, predictable experience across all interfaces:

- **Component Library**: All UI components MUST use daisy-rsx (DaisyUI wrapper).
  No custom styling that deviates from design system without design approval.
- **Responsive Design**: All pages MUST be usable on desktop (1920x1080) and
  tablet (768px width) viewports. Mobile support (375px width) is OPTIONAL unless
  specified in feature requirements.
- **Error Handling**: User-facing errors MUST:
  - Use `dioxus-toast` for non-blocking notifications
  - Provide actionable error messages (what went wrong, what to do next)
  - Never expose internal error details (stack traces, database errors) to users
  - Log full error context server-side for debugging
- **Loading States**: All async operations MUST show loading indicators.
  No "frozen" UI during data fetching.
- **Accessibility**: All interactive elements MUST be keyboard accessible.
  Form inputs MUST have proper labels. Color MUST NOT be the only means of
  conveying information.
- **Performance Perception**: Initial page load MUST show content within 2 seconds.
  Interactions MUST provide feedback within 100ms.

**Rationale**: Consistent UX reduces cognitive load, improves user satisfaction, and
reduces support burden. Accessibility is both ethical and often legally required.

### IV. Performance Requirements

System MUST meet these performance standards:

- **Backend API Response Times**:
  - Simple queries (single record): p95 < 100ms
  - List queries (up to 100 records): p95 < 200ms
  - Complex queries (joins, aggregations): p95 < 500ms
  - Bulk operations: p95 < 2000ms
- **Frontend Performance**:
  - First Contentful Paint (FCP): < 1.5 seconds
  - Time to Interactive (TTI): < 3 seconds
  - WebAssembly bundle size: < 5MB compressed
- **Database Performance**:
  - All queries MUST use proper indexes (verify with `EXPLAIN`)
  - N+1 queries are FORBIDDEN (use eager loading/joins)
  - Connection pooling MUST be configured (min: 5, max: 20 connections)
- **Concurrency**:
  - API MUST handle minimum 100 concurrent requests without degradation
  - No blocking operations in request handlers (use async/await properly)
- **Resource Limits**:
  - Backend memory: < 500MB RSS under normal load
  - Frontend memory: < 100MB heap per active tab
- **Monitoring**: All performance-critical paths MUST be instrumented with metrics.
  Performance regressions MUST be detected in CI before merging.

**Rationale**: Performance is a feature. Slow systems frustrate users and limit scale.
Defining clear targets enables objective measurement and prevents gradual degradation.

### V. Clean Architecture

Maintain clear separation of concerns and dependency flow:

- **Layer Structure** (Backend):
  - **Routes** (`routes/`): HTTP handlers only. Parse requests, call services, format responses.
  - **Services** (`services/`): Business logic. Orchestrate repositories, implement domain rules.
  - **Repositories** (`repositories/`): Data access only. Translate domain models to/from database.
  - **Models** (`models/`): Data structures. No business logic, only data representation.
  - **Utils** (`utils/`): Stateless helper functions. No dependencies on other layers.
- **Layer Structure** (Frontend):
  - **Pages** (`pages/`): Route components. Compose smaller components, manage page state.
  - **Components** (`components/`): Reusable UI. Self-contained, receive props, emit events.
  - **Models** (`models/`): Data structures matching backend DTOs.
  - **Utils** (`utils/`): Client helpers (validation, HTTP, cookies). No UI logic.
- **Dependency Rules**:
  - Outer layers MAY depend on inner layers (routes → services → repositories)
  - Inner layers MUST NOT depend on outer layers (repositories cannot import routes)
  - Same-level dependencies are FORBIDDEN (services cannot directly call other services
    without dependency injection)
- **Dependency Injection**: Use `Shaku` for backend DI. All service dependencies MUST be
  injected, not constructed directly. Enables testing and decoupling.
- **No God Objects**: Classes/modules with >500 lines MUST be reviewed for single
  responsibility violations. Refactor into smaller, focused units.

**Rationale**: Clean architecture makes code testable, maintainable, and changeable.
Clear boundaries enable parallel development and reduce coupling.

## Development Workflow

### Feature Development Process

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
6. **Review**: Submit PR with link to spec and plan. Reviewer MUST verify all
   constitution principles are met.
7. **Validation**: Run `/speckit.analyze` to verify spec/plan/tasks consistency
   before merging.

### Branching Strategy

- **Main branch**: Always production-ready. Protected. Requires PR approval.
- **Feature branches**: Named `[###-feature-name]` matching spec folder.
- **Hotfix branches**: Named `hotfix/description` for urgent production fixes.
- **No direct commits**: All changes via PR, even for documentation.

### Commit Standards

- **Format**: `<type>(<scope>): <description>` (Conventional Commits)
- **Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `chore`
- **Scope**: Component/module name (e.g., `auth`, `datasource`, `frontend`)
- **Description**: Imperative mood, lowercase, no period, <72 chars
- **Example**: `feat(datasource): add MySQL connection pooling support`

## Quality Gates

All gates MUST pass before merging to main:

### Code Quality Gates

- [ ] `cargo fmt -- --check` passes (formatting)
- [ ] `cargo clippy -- -D warnings` passes (linting)
- [ ] `cargo check` passes (compilation)
- [ ] No `TODO` or `FIXME` comments without associated tracking issues

### Testing Gates

- [ ] All tests pass: `cargo test`
- [ ] Test coverage meets minimums (80% for services)
- [ ] New features have integration tests for user journeys
- [ ] Performance tests pass (if applicable to feature)
- [ ] Manual testing completed for UI changes (screenshot/recording in PR)

### Architecture Gates

- [ ] Follows clean architecture layer rules (no dependency violations)
- [ ] No circular dependencies between modules
- [ ] Dependency injection used for service dependencies
- [ ] Error handling uses `Result<T, E>` (no panics in production paths)

### Documentation Gates

- [ ] Public APIs have doc comments
- [ ] README updated if user-facing changes
- [ ] CHANGELOG.md updated with user-visible changes
- [ ] Migration guide provided if breaking changes

### Performance Gates

- [ ] No performance regressions detected (if metrics available)
- [ ] Database queries reviewed for indexes and N+1 issues
- [ ] Bundle size within limits (frontend changes)
- [ ] Load testing completed for high-traffic features

## Governance

### Authority and Amendments

- **Constitution Authority**: This constitution supersedes all other development practices,
  coding standards, or verbal agreements. When in doubt, constitution takes precedence.
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

- **PR Reviews**: All pull requests MUST be reviewed for constitutional compliance.
  Reviewers use checklist from Quality Gates section.
- **Violation Handling**:
  - Non-compliant code MUST be flagged in review with specific principle reference
  - Author MUST address violations before approval
  - Repeated violations trigger architecture review meeting
- **Complexity Justification**: Violations of simplicity principles (e.g., adding 4th
  project, complex patterns) MUST be documented in plan.md Complexity Tracking table
  with explicit justification and rejected alternatives.
- **Regular Audits**: Monthly review of merged PRs for constitutional adherence.
  Systemic violations trigger constitution amendment discussion.

### Runtime Guidance

For day-to-day development guidance beyond governance, see:
- Backend: `data-terminal/CLAUDE.md` (architecture patterns, common tasks)
- Frontend: `data-terminal/frontend/CLAUDE.md` (component patterns, styling)

These guides provide practical examples and shortcuts but MUST NOT contradict
constitution principles. If conflict exists, constitution wins.

**Version**: 1.0.0 | **Ratified**: 2025-10-16 | **Last Amended**: 2025-10-16
