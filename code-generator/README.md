# AutoGPT

An AI-powered code generation system that converts user requirements into Rust web server implementations.

## Overview

This system uses GPT-4 to understand user requirements and generate web server code based on a predefined template. It employs multiple AI agents to handle different aspects of the development process.

## Project Structure

```
src/
├── ai_functions/     # AI function definitions for code generation
├── apis/            # OpenAI API integration
├── helpers/         # Utility functions
├── models/          # Core models and agents
│   ├── agent_basic/    # Basic agent traits
│   ├── agents/         # Specialized agents
│   ├── agents_manager/ # Agent coordination
│   └── general/        # Common types
└── main.rs          # Entry point
```

## Quick Start

### Prerequisites

- Rust (latest stable)
- OpenAI API Key
- Cargo

### Installation

1. Set up environment variables

```bash
touch .env
# Add your OpenAI API key to .env
# OPEN_AI_KEY=your-api-key
# OPEN_AI_ORG=your-org-id
```

2. Run

```bash
cargo run
```

## How It Works

The system uses three specialized AI agents:

1. Project Manager

   - Coordinates the overall process
   - Analyzes user requirements
   - Manages other agents

2. Solution Architect

   - Analyzes project scope
   - Determines required features
   - Plans API endpoints

3. Backend Developer
   - Generates actual code implementation
   - Performs unit testing
   - Fixes bugs

### Code Generation Process

1. User inputs project requirements
2. System analyzes requirements and determines project scope
3. Generates web server code based on template
4. Performs automated testing
5. Fixes any detected issues

## Security

- All generated code requires manual review
- Basic security measures are implemented by default
- Production deployments should undergo security audit
