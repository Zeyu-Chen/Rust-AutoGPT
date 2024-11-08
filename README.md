# Multi-Agent Code Generator

[![Rust](https://img.shields.io/badge/Rust-1.8-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Actix_Web-4.3-000000?logo=rust&logoColor=white)](https://actix.rs/)
[![OpenAI](https://img.shields.io/badge/OpenAI-v1-412991?logo=openai&logoColor=white)](https://openai.com/)
[![Tokio](https://img.shields.io/badge/Tokio-1.28-0BA8E9?logo=rust&logoColor=white)](https://tokio.rs/)

A multi-agent code generator system powered by OpenAI that automatically converts user requirements into a complete Rust web server implementation.

## Core Features

- 🤖 Multi-Agent Collaboration
  - Project Manager: Project coordination and requirements analysis
  - Solution Architect: System design and API planning
  - Backend Developer: Code generation and testing
- 🏗️ Automated Code Generation
  - Template-based generation
  - CRUD operations
  - User authentication
  - External API integration
- 🧪 Automated Testing & Fixing
  - Unit testing
  - Bug detection
  - Auto-fixing
  - API endpoint validation

## Project Structure

```
/
├── autogpt/                 # AI Code Generator
│   ├── src/
│   │   ├── ai_functions/    # AI function definitions
│   │   ├── apis/            # OpenAI API integration
│   │   ├── helpers/         # Utility functions
│   │   ├── models/          # Agent models
│   │   └── main.rs          # Entry point
│   ├── schemas/             # API schemas
│   └── Cargo.toml
│
└── web-server/              # Server Template
    ├── src/
    │   ├── main.rs          # Generated server code
    │   └── code_template.rs # Base template
    ├── database.json        # Persistence
    └── Cargo.toml
```

## Quick Start

### Prerequisites

- Rust (latest stable)
- OpenAI API Key
- Cargo

### Installation

1. Clone the repository

```bash
git clone <repository-url>
```

2. Set up environment variables

```bash
cd autogpt
touch .env
# Add your OpenAI API key to .env
# OPEN_AI_KEY=your-api-key
# OPEN_AI_ORG=your-org-id
```

3. Run

```bash
cargo run
```

## How It Works

1. User inputs requirements
2. Managing Agent analyzes requirements and coordinates work
3. Solution Architect designs system architecture
4. Backend Developer generates code
5. System automatically tests and fixes issues

## Web Server Template

The web-server template provides:

- Actix-web framework integration
- CORS support
- JSON persistence
- Thread-safe state management
- Basic data structures
- Database operations
- User management
- External API integration

### Template Customization

The base template (`web-server/src/code_template.rs`) can be modified to:

- Add new base functionality
- Modify default implementations
- Include additional dependencies

## Security

- Generated code requires manual review
- Basic security measures implemented by default
- Production deployments should undergo security audit
