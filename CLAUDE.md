# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PoWCAPTCHA is a Proof-of-Work CAPTCHA system designed to protect web applications from DoS attacks by requiring users to perform computational work. The system consists of three main components:

- **pow-captcha-core**: Core PoW library with SHA-2 based challenge/solution validation
- **pow-captcha-server**: gRPC backend service with PostgreSQL storage for session management
- **web-component**: TypeScript/Lit-based web component for frontend integration

## Development Commands

### Backend (Rust)
```bash
# Build the entire workspace
cargo build

# Run the server (requires PostgreSQL)
cd pow-captcha-server
cargo run --bin server

# Run tests
cargo test

# Check for compilation errors
cargo check
```

### Frontend (TypeScript/Web Component)
```bash
cd web-component

# Install dependencies
yarn install

# Development server with hot reload
yarn start

# Build for production
yarn build

# Run tests
yarn test

# Lint and format
yarn lint
yarn format

# Storybook for component development
yarn storybook
```

### Database Setup
The server requires PostgreSQL with these environment variables:
```bash
POWCAPTCHA_SERVICE_DATABASE_URL="postgres://user:password@localhost/dbname"
POWCAPTCHA_SERVICE_ADDRESS="[::]:8080"  # Optional, defaults to this value
```

Run migrations with sqlx:
```bash
sqlx migrate run --database-url "postgres://user:password@localhost/dbname"
```

## Architecture

### Clean Architecture Pattern
The server follows Domain-Driven Design with clear separation:

- **Domain Layer** (`pow-captcha-server/src/domain/`): Core business entities
  - `ExamSession`: Represents a PoW challenge session
  - `WorkProofToken`: Represents validated proof tokens

- **Application Layer** (`pow-captcha-server/src/application/`): Business logic
  - `CaptchaService`: Orchestrates challenge generation and validation

- **Infrastructure Layer** (`pow-captcha-server/src/infrastructure/`): External concerns
  - `PostgresExamSessionRepository`: Database operations for sessions
  - `PostgresWorkProofTokenRepository`: Database operations for tokens
  - `GrpcService`: gRPC API implementation

### gRPC Protocol
The system uses Protocol Buffers with two services:
- `PoWcaptchaFrontendService`: For web components to get challenges and submit solutions
- `PoWcaptchaBackendService`: For application backends to validate tokens

Protobuf definitions are in `contracts/pow_captcha/v1/` and code generation is handled by the `volo-gen` crate.

### Web Component Integration
The frontend component (`web-component/src/PowCaptcha.ts`) is built with Lit and provides:
- Automatic challenge fetching and solving using Web Workers (`pow.worker.ts`)
- Token buffering to minimize user wait times
- Connect-Web for gRPC communication with the backend

## Key Implementation Details

### PoW Algorithm
Uses SHA-256 with difficulty-based target (leading zeros in hash). The core algorithm is in `pow-captcha-core/src/lib.rs`.

### Database Schema
Two main tables in `migrations/20250821103507_create_exam_sessions_table.sql`:
- `exam_sessions`: Stores active challenges with expiration
- `work_proof_tokens`: Stores single-use validation tokens

Both tables use `UNLOGGED` for performance and have indexes on frequently queried fields.

### Configuration
- Backend: Environment variables with `config` crate (see `pow-captcha-server/src/bin/server.rs`)
- Frontend: Package.json scripts for all development tasks
- Workspace: Cargo workspace with shared dependencies in root `Cargo.toml`

## Testing
- Rust: Standard `cargo test` with unit tests throughout the codebase
- TypeScript: Web Test Runner (`@web/test-runner`) with test files in `web-component/test/`
- Integration: Storybook for component testing and documentation