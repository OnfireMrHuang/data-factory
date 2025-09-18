# Data-Factory ETL Pipeline Builder Enhancement PRD

## Intro Project Analysis and Context

### Enhancement Vision Summary

**Path A: ETL Pipeline Builder Enhancement** with specific focus on:

**Core Enhancement**: Visual DAG workflow designer with drag-and-drop data transformation nodes, specifically enabling MySQL-to-Data Warehouse ETL pipelines with full/incremental data loading capabilities.

**User Journey Transformation:**

**CURRENT STATE** (Limited Management Interface):
1. User configures projects, resources, datasources manually
2. User manually coordinates with data-engine for processing
3. No visual workflow creation
4. No automated data warehouse loading

**DESIRED STATE** (Enhanced ETL Platform):
1. **Data Processing Focus**: Users focus solely on data transformation logic and workflows
2. **MySQL → Data Warehouse Pipeline**: Seamless full/incremental data loading from MySQL to data warehouse
3. **Visual DAG Designer**: Drag-and-drop interface for creating data transformation workflows
4. **AI-Assisted Development**: data-agent provides intelligent recommendations and optimizations
5. **Configuration-Driven**: data-terminal generates configurations that data-engine executes
6. **Query-Ready Output**: Transformed data available for other applications' analysis needs

**Architecture Integration Model:**
- **data-terminal**: User interface for resource config, data processing config, schedule config
- **data-engine**: Reads data-terminal configurations, executes dataflow processing
- **data-agent**: AI engine providing tools, knowledge base, and workflow optimization

**Target Users**: Technical power users (data engineers, senior analysts) who need advanced ETL configuration capabilities

**Key Success Metrics:**
- Users can create MySQL→Data Warehouse pipelines visually
- Full and incremental loading capabilities working seamlessly  
- Data transformations produce query-ready datasets for downstream applications
- AI assistance reduces pipeline creation time and improves optimization
