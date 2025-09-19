# Project Brief: Data-factory

## Executive Summary

Data-factory is a data platform that can integrate, transform, and analyze data from various sources, providing batch processing and stream processing capabilities, supporting AI-powered workflows, and supporting flexible reource configurations. It mainly aims to provide data value for individuals so that reduce many unnecessary functions in practicle, like user roles and permissions.

**Primary Problem:** Individuals can't find a user-friendly data platform that can simplely integrate, transform, and analyze data from various sources 

**Target Market:** It is designed for individuals

**Key Value Proposition:** The only data platform that combines ETL with AI-powered automation, natural language interfaces, and collaborative features - making data transformation accessible to business users while providing data engineers with intelligent optimization and predictive capabilities.

**Key Differentiators:**
- AI-powered "Data Story Narrator" and relationship discovery
- Natural language to ETL conversion for business users  
- Predictive pipeline intelligence and automatic optimization
- Gamified data quality with collaborative workflows
- Data Time Machine for historical state manage/ment


**Modules:**
- data-terminal: It's user interface for data-factory, which is developed with Rust. It consists of three submodules: frontend, backend, and docsite, which is designed for user interaction and documentation. It is worth noting that the backend submoudle mainly provides RESTful APIs for frontend and specialize in general functionaites such as authentication, configuration and so on. 

- data-engine: It's the core data processing module, which is developed with Java. The difference between data-engine and the backend of data-terminal is that data-engine mainly focus on data tasks such as data integration, transformation, and sync. it's apparently benefit that data-engine is a independent module instead of a part of backend of data-terminal, which can makes it to isolate from other irrelated modules and utilize Java's big data ecosystem

- data-ai: It's the code ai-powered module, which is developed with Python. The difference between data-ai and the backend of data-terminal is that data-ai mainly focus on AI-powered workflows, such as tools, knowledge base, agents and so on. It's apparently benefit that data-ai is a independent module instead of a part of backend of data-terminal, which can makes it to isolate from other irrelated modules and utilize Python's ai ecosystem

- devops: It consist of devops tools and documents, which mainly specilizes in deployment and maintain of middlewares and services, and responsible for the reliability, performance and scalability of data-factory.



