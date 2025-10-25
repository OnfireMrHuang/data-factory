# Feature Specification: Collect Task Module

**Feature Branch**: `001-collect-task-module`
**Created**: 2025-10-25
**Status**: Draft
**Input**: User description: "Implement collect task module with full and incremental collection modes for database and API datasources"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Full Collection from Database Datasource (Priority: P1)

As a data engineer, I want to configure a full collection task from a database datasource (MySQL/PostgreSQL) to a relational database resource, so that I can extract, transform, and load complete datasets into my data warehouse.

**Why this priority**: This is the most common data collection scenario and represents the core ETL functionality. It delivers immediate value by enabling basic data pipeline operations.

**Independent Test**: Can be fully tested by creating a collection task that selects tables/fields from a source database, applies SQL transformations, and writes to a target database. Delivers a working data pipeline that can be executed and verified independently.

**Acceptance Scenarios**:

1. **Given** user is on the collect task page, **When** user selects "Full Collection" mode and chooses a database datasource (MySQL/PostgreSQL), **Then** system displays relational_database as the only available resource type
2. **Given** user has selected a database datasource and relational_database resource, **When** user opens the left configuration area, **Then** system displays available tables from the datasource
3. **Given** user selects a table from the datasource, **When** user expands the table, **Then** system displays all available fields from that table
4. **Given** user has selected fields, **When** user views the result area, **Then** system displays the selected tables and fields visually
5. **Given** user is in the right configuration area, **When** user chooses "generate target table schema", **Then** system auto-generates a table schema based on the selected source table/fields mapping
6. **Given** user has generated a target table schema, **When** user edits the schema, **Then** system allows modifications to field names, types, and constraints
7. **Given** user has finalized the target schema, **When** user clicks "apply", **Then** system creates the target table in the resource
8. **Given** user is in the middle configuration area, **When** user writes SQL in the transform box, **Then** system allows SQL transformation logic to be defined
9. **Given** user has configured all collection rules, **When** user clicks "confirm", **Then** system saves the collection task configuration
10. **Given** user has saved the collection task, **When** user clicks "apply", **Then** system publishes the task to the data pipeline for scheduling

---

### User Story 2 - Full Collection from API Datasource (Priority: P2)

As a data engineer, I want to configure a full collection task from an API datasource to either a relational database or file system resource, so that I can periodically fetch data from external APIs and store it for analysis.

**Why this priority**: API data collection is increasingly common for integrating third-party data sources. It extends the platform's capabilities beyond traditional databases.

**Independent Test**: Can be tested independently by configuring a query API datasource with a schedule, transforming JSON responses to table/file schemas, and storing results. Delivers API integration capability without dependencies on other stories.

**Acceptance Scenarios**:

1. **Given** user is on the collect task page, **When** user selects "Full Collection" mode and chooses a Query API datasource, **Then** system displays relational_database and file_system as available resource types
2. **Given** user has selected a Query API datasource, **When** user opens the left configuration area, **Then** system displays query schedule configuration options
3. **Given** user is configuring query schedule, **When** user sets schedule parameters (interval, cron expression), **Then** system accepts the schedule configuration
4. **Given** user is in the left configuration area, **When** user configures cursor update strategy, **Then** system allows defining how to track the last fetched position for incremental queries
5. **Given** user is in the right configuration area, **When** user selects a resource, **Then** system allows choosing an existing table/file or creating a new one
6. **Given** user has selected/created a target table/file, **When** user returns to middle configuration area, **Then** system displays a text box for JSON schema transformation
7. **Given** user receives JSON data from API, **When** user defines transformation rules in the text box, **Then** system allows mapping JSON schema to target table schema or file schema
8. **Given** user has configured all collection rules, **When** user clicks "confirm" and then "apply", **Then** system saves and publishes the API collection task to the data pipeline

---

### User Story 3 - Incremental Collection from Database Datasource (Priority: P3)

As a data engineer, I want to configure an incremental collection task from a database datasource (MySQL/PostgreSQL) to a queue resource, so that I can capture real-time database changes (CDC) and stream them to message queues for event-driven processing.

**Why this priority**: Change Data Capture is an advanced feature needed for real-time data pipelines. While valuable, it's not required for basic batch ETL workflows.

**Independent Test**: Can be tested independently by selecting source tables for CDC, defining queue topics and message schemas, and applying filter rules. Delivers real-time data streaming capability as a standalone feature.

**Acceptance Scenarios**:

1. **Given** user is on the collect task page, **When** user selects "Incremental Collection" mode and chooses a database datasource (MySQL/PostgreSQL), **Then** system displays queue as the only available resource type
2. **Given** user has selected a database datasource and queue resource, **When** user opens the left configuration area, **Then** system displays available tables from the datasource for CDC
3. **Given** user selects collection tables, **When** user navigates to the right configuration area, **Then** system allows defining the receive topic name
4. **Given** user is defining the receive topic, **When** user configures the message schema, **Then** system allows specifying the structure of CDC messages
5. **Given** user is in the middle configuration area, **When** user defines filter rules in the text box, **Then** system allows filtering specific messages based on conditions
6. **Given** user has defined filter rules, **When** user adds message field transformations, **Then** system allows adding computed fields or enriching messages
7. **Given** user has configured all incremental collection rules, **When** user clicks "confirm" and then "apply", **Then** system saves and publishes the CDC task to the data pipeline

---

### User Story 4 - Incremental Collection from API Datasource (Priority: P4)

As a data engineer, I want to configure an incremental collection task from a Subscription API datasource to a queue resource, so that I can receive real-time webhook events from external APIs and route them to message queues.

**Why this priority**: Webhook/subscription API handling is the least common scenario and builds on top of other collection capabilities. It's valuable for specific integration use cases but not critical for MVP.

**Independent Test**: Can be tested independently by configuring a subscription API datasource, defining queue topics, applying filter rules, and validating webhook message routing. Delivers webhook integration as a standalone capability.

**Acceptance Scenarios**:

1. **Given** user is on the collect task page, **When** user selects "Incremental Collection" mode and chooses a Subscription API datasource, **Then** system displays queue as the only available resource type
2. **Given** user has selected a Subscription API datasource and queue resource, **When** user opens the left configuration area, **Then** system displays filter rule configuration
3. **Given** user is in the left configuration area, **When** user defines filter rules, **Then** system allows filtering incoming webhook messages based on conditions
4. **Given** user is in the left configuration area, **When** user adds message field transformations, **Then** system allows adding or modifying message fields
5. **Given** user is in the right configuration area, **When** user defines the receive topic, **Then** system allows specifying the queue topic name
6. **Given** user is defining the receive topic, **When** user configures the message schema, **Then** system allows specifying the expected structure of webhook messages
7. **Given** user has configured all subscription API collection rules, **When** user clicks "confirm" and then "apply", **Then** system saves and publishes the webhook task to the data pipeline

---

### Edge Cases

- What happens when a selected datasource is deleted or becomes unavailable before the collection task is applied?
- How does the system handle schema mismatches when auto-generating target table schemas from source tables with incompatible data types?
- What happens if a user tries to select an incompatible resource type for a given datasource/collection mode combination?
- How does the system handle SQL syntax errors in the transformation box for database full collection?
- What happens when JSON schema transformation fails due to invalid or unexpected API response formats?
- How does the system handle queue resource connectivity issues during incremental collection?
- What happens if a user tries to create a target table that already exists with a different schema?
- How does the system validate and enforce cursor update strategies for API query scheduling?
- What happens when filter rules for incremental collection are invalid or cause all messages to be dropped?
- How does the system handle applying a collection task when the data pipeline service is unavailable?

## Requirements *(mandatory)*

### Functional Requirements

#### Collection Mode Selection

- **FR-001**: System MUST allow users to select between "Full Collection" and "Incremental Collection" modes as the first step in creating a collect task
- **FR-002**: System MUST enforce mode-specific datasource and resource type constraints based on the selected collection mode

#### Datasource and Resource Selection

- **FR-003**: System MUST allow users to select from previously configured datasources (added from the datasource page)
- **FR-004**: System MUST allow users to select from previously configured resources (added from the public resource page)
- **FR-005**: System MUST support MySQL and PostgreSQL database datasources for both Full and Incremental collection modes
- **FR-006**: System MUST only allow Query API datasources for Full Collection mode
- **FR-007**: System MUST only allow Subscription API datasources for Incremental Collection mode
- **FR-008**: System MUST only allow relational_database and file_system resource types for Full Collection mode
- **FR-009**: System MUST only allow queue resource type for Incremental Collection mode
- **FR-010**: System MUST prevent invalid datasource/resource type combinations and display clear validation messages

#### Full Collection - Database Datasource Configuration

- **FR-011**: System MUST restrict resource type to relational_database only when Full Collection mode is selected with a database datasource
- **FR-012**: System MUST display available tables from the selected datasource in the left configuration area
- **FR-013**: System MUST allow users to select one or more tables from the datasource
- **FR-014**: System MUST display available fields for selected tables
- **FR-015**: System MUST allow users to select specific fields from tables
- **FR-016**: System MUST display selected tables and fields visually in a result area within the left configuration
- **FR-017**: System MUST provide an option to auto-generate target table schema based on source table/field mapping in the right configuration area
- **FR-018**: System MUST allow users to select an existing target table from the resource in the right configuration area
- **FR-019**: System MUST allow users to edit generated or selected table schemas (field names, types, constraints)
- **FR-020**: System MUST allow users to apply the target table schema creation/modification to the resource
- **FR-021**: System MUST provide a SQL transformation text box in the middle configuration area
- **FR-022**: System MUST allow users to write and edit SQL transformation logic

#### Full Collection - API Datasource Configuration

- **FR-023**: System MUST allow relational_database or file_system resource types when Full Collection mode is selected with a Query API datasource
- **FR-024**: System MUST provide query schedule configuration options in the left configuration area (interval, cron expression, start time)
- **FR-025**: System MUST allow users to define cursor update strategy for tracking the last fetched position in paginated or incremental API queries
- **FR-026**: System MUST allow users to select an existing table/file from the resource in the right configuration area
- **FR-027**: System MUST allow users to create a new table/file in the resource in the right configuration area
- **FR-028**: System MUST provide a JSON schema transformation text box in the middle configuration area
- **FR-029**: System MUST allow users to define transformation rules to map JSON schema to target table schema or file schema

#### Incremental Collection - Database Datasource Configuration

- **FR-030**: System MUST restrict resource type to queue only when Incremental Collection mode is selected with a database datasource
- **FR-031**: System MUST display available tables from the datasource in the left configuration area for CDC selection
- **FR-032**: System MUST allow users to select one or more tables for change data capture
- **FR-033**: System MUST allow users to define the receive topic name in the right configuration area
- **FR-034**: System MUST allow users to configure the message schema for CDC events in the right configuration area
- **FR-035**: System MUST provide a filter rule text box in the middle configuration area
- **FR-036**: System MUST allow users to define filter rules to selectively capture database changes (e.g., filter by operation type, field values)
- **FR-037**: System MUST allow users to add or transform message fields in the filter configuration

#### Incremental Collection - API Datasource Configuration

- **FR-038**: System MUST restrict resource type to queue only when Incremental Collection mode is selected with a Subscription API datasource
- **FR-039**: System MUST provide filter rule configuration in the left configuration area
- **FR-040**: System MUST allow users to define filter rules to selectively process incoming webhook messages
- **FR-041**: System MUST allow users to add or transform message fields in the left configuration area
- **FR-042**: System MUST allow users to define the receive topic name in the right configuration area
- **FR-043**: System MUST allow users to configure the message schema for webhook events in the right configuration area

#### Task Lifecycle Management

- **FR-044**: System MUST provide a "Confirm" button to save the collection task configuration
- **FR-045**: System MUST validate all required configuration fields before allowing task save
- **FR-046**: System MUST provide an "Apply" button to publish the saved collection task to the data pipeline
- **FR-047**: System MUST communicate with the data pipeline service to register the collection task for scheduling and execution
- **FR-048**: System MUST display task status (Draft, Saved, Applied, Running, Failed) to the user
- **FR-049**: System MUST allow users to edit saved tasks before applying them
- **FR-050**: System MUST prevent editing of applied tasks that are already running in the pipeline

### Key Entities

- **CollectTask**: Represents a data collection task configuration. Attributes include task ID, task name, collection mode (Full/Incremental), datasource reference, resource reference, configuration rules (specific to mode and datasource type), status (Draft/Saved/Applied/Running/Failed), created timestamp, updated timestamp, applied timestamp.

- **Datasource**: Represents a data source from which data is collected. Attributes include datasource ID, datasource name, datasource type (MySQL, PostgreSQL, Query API, Subscription API), datasource category (Database, API), connection details, status (Active/Inactive).

- **Resource**: Represents a target destination where collected data is stored. Attributes include resource ID, resource name, resource type (relational_database, file_system, queue), connection details, status (Active/Inactive).

- **CollectionRule**: Represents the specific configuration for a collection task, varying by mode and datasource type. For Full-Database: selected tables, selected fields, SQL transformation, target table schema. For Full-API: query schedule, cursor update strategy, JSON transformation, target table/file. For Incremental-Database: CDC tables, filter rules, message schema, topic name. For Incremental-API: filter rules, message schema, topic name.

- **TargetSchema**: Represents the schema of the target table or file. Attributes include field definitions (name, type, constraints), schema type (table/file), schema format (for files: JSON, CSV, Parquet).

- **FilterRule**: Represents filtering and transformation logic for incremental collection. Attributes include filter conditions (field-based predicates), field transformations (add/modify fields), evaluation order.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can create and configure a full collection task from database datasource to relational_database resource in under 5 minutes
- **SC-002**: Users can create and configure an API collection task with query scheduling in under 7 minutes
- **SC-003**: System successfully validates datasource/resource type combinations and prevents invalid configurations with clear error messages in 100% of cases
- **SC-004**: 90% of users successfully create their first collection task without requiring external help or documentation
- **SC-005**: Collection tasks are successfully published to the data pipeline within 3 seconds of clicking the "Apply" button
- **SC-006**: Users can generate target table schemas from source tables with 95% accuracy (field types correctly mapped)
- **SC-007**: System supports configuring collection tasks for at least 100 different datasources and resources without performance degradation
- **SC-008**: Task configuration is auto-saved every 30 seconds to prevent data loss in case of browser crashes
- **SC-009**: Users can edit and re-apply saved collection tasks within 2 minutes
- **SC-010**: System provides real-time validation feedback (within 500ms) for configuration errors such as invalid SQL syntax or malformed JSON transformation rules
