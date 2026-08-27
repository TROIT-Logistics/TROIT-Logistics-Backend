# TROIT Logistics — Backend Engineering Onboarding & Developer Guide

Welcome to the **TROIT Logistics** backend codebase! This repository houses the REST API server foundation for TROIT Logistics, an e-commerce trust and logistics platform starting with a controlled pilot in Port Harcourt, Nigeria.

This document serves as the **authoritative onboarding manual, architectural specification, and coding standards guide** for all backend engineers and interns. Read this document thoroughly before creating feature branches or writing backend code.

---

## 1. Product Overview & Context

Nigerian online commerce suffers from severe trust, verification, and logistics gaps: financial scams on social commerce, product mismatch, counterfeit goods, and last-mile delivery failures.

**TROIT Logistics** closes this trust gap by embedding physical verification and accountability into every stage of a transaction:
1. **Seller KYC**: Verification of seller identity prior to platform access.
2. **Physical Seller Inspection**: Field agents inspect physical stores and operations in person.
3. **Product Verification & Verified Inventory**: Goods are inspected for authenticity, condition, and graded (**Grade A, B, C**).
4. **Buyer KYC**: Buyer identification for transaction safety and dispute resolution.
5. **Controlled Logistics**: In-house dispatch fleet for end-to-end delivery tracking.
6. **Pickup Inspection**: Riders verify item condition and capture photo evidence before an item leaves the seller.
7. **Protected Transactions (Escrow)**: Funds remain held until delivery confirmation or dispute resolution.
8. **Seller Probation & Trust Progression**: New sellers undergo close monitoring for their **first 5 transactions**. Trust is earned through performance, never purchased via VIP status.

### Supported User Roles
The platform architecture supports 5 core user roles:
* 🛒 `buyer`: Product discovery, verified inventory browsing, protected payment, delivery tracking, disputes.
* 🏪 `seller`: Registration, KYC onboarding, listing verified inventory, order tracking, trust progression.
* 🛵 `rider`: Assigned pickups, on-site condition checklist & photo evidence logging, delivery confirmation.
* 📋 `field_agent`: Physical store verification, inventory quality grading, inspection reporting.
* 📊 `admin`: Operational monitoring, KYC approvals, trust score management, dispute resolution, pilot KPIs.

---

## 2. Environment Foundation vs. Intern Scope

To ensure architectural clarity, understand what is already provided in this setup and what you are expected to build:

### Provided Foundation (Set up by Mentors/Team)
* ✅ Rust workspace structure, Axum web framework, and Tokio async runtime setup.
* ✅ PostgreSQL connection pool configuration and automatic SQLx migrations pipeline (`src/db/mod.rs`).
* ✅ Centralized error handling returning standardized JSON responses (`src/errors/mod.rs`).
* ✅ Type-safe environment variable reader (`src/config/mod.rs`).
* ✅ Structured logging subscriber using `tracing` and `tracing-subscriber`.
* ✅ Argon2id password hashing and JWT token generation/verification architecture foundation (`src/auth/service.rs`).
* ✅ Infrastructure health check endpoint `GET /health` (`src/routes/mod.rs`).
* ✅ Unit and integration testing setup (`tests/`).
* ✅ Zero-config local PostgreSQL setup via Docker Compose (`docker-compose.yml`).

### Scope to be Built by Interns (Your Assignments)
* 🚀 Feature domain models, database migrations, and repository queries.
* 🚀 Business service handlers for assigned user stories (Seller KYC, Product catalogue, Escrow, Rider dispatch, Disputes).
* 🚀 Input validation rules and role-based authorization checks.
* 🚀 Integration tests for feature endpoints.

---

## 3. Technology Stack

* **Language**: Rust (2021 edition)
* **Web Framework**: Axum (v0.7+)
* **Async Runtime**: Tokio (v1.0+)
* **Database**: PostgreSQL (v16+)
* **Database Driver & Migrations**: SQLx (v0.8+ with `runtime-tokio-rustls`, `postgres`)
* **Security & Auth**: Argon2id (`argon2`), JWT (`jsonwebtoken`)
* **Serialization & Validation**: Serde (`serde`), Validator (`validator`)
* **Logging & Observability**: Tracing (`tracing`, `tracing-subscriber`)
* **Containerization**: Docker & Docker Compose

---

## 4. Getting Started & Installation

### Prerequisites
* Rust toolchain (v1.75.0 or higher): Install via [rustup.rs](https://rustup.rs)
* Docker & Docker Compose: Install via [Docker Desktop](https://www.docker.com/)

### Setup Instructions

1. **Clone the repository**:
   ```bash
   git clone https://github.com/TROIT-Logistics/TROIT-Logistics-Backend.git
   cd TROIT-Logistics-Backend
   ```

2. **Configure Environment Variables**:
   Copy `.env.example` to create your local `.env` file:
   ```bash
   cp .env.example .env
   ```

3. **Start Local PostgreSQL Database**:
   ```bash
   docker compose up -d
   ```
   *This starts a local PostgreSQL instance on port `5432` with database `troit_logistics`.*

4. **Run the Backend Server**:
   ```bash
   cargo run
   ```
   *The server will start on `http://0.0.0.0:8000` and automatically execute pending database migrations.*

5. **Verify Server Health**:
   ```bash
   curl http://localhost:8000/health
   ```
   Expected Response:
   ```json
   {
     "service": "TROIT Logistics Backend API",
     "status": "ok",
     "version": "0.1.0"
   }
   ```

---

## 5. Available Development Commands

| Command | Purpose |
| :--- | :--- |
| `cargo run` | Builds and starts local backend development server |
| `cargo check` | Fast compilation check without outputting binary |
| `cargo test` | Runs all unit and integration tests |
| `cargo fmt` | Formats all Rust source files according to Rust style guide |
| `cargo clippy -- -D warnings` | Runs Rust linter and fails on any warnings |
| `docker compose up -d` | Starts local PostgreSQL container in background |
| `docker compose down` | Stops local PostgreSQL container |

---

## 6. Directory & Project Architecture

```text
TROIT-Logistics-Backend/
│
├── src/
│   ├── main.rs                 # Server startup, tracing, CORS & TCP listener initialization
│   ├── lib.rs                  # Library exports for integration testing
│   │
│   ├── config/                 # Strongly-typed environment configuration (AppConfig)
│   ├── db/                     # SQLx PostgreSQL pool creation & auto-migrations
│   │
│   ├── auth/                   # Authentication module
│   │   ├── mod.rs
│   │   ├── handlers.rs         # REST Handlers (register, login, logout, me)
│   │   ├── models.rs           # Request/Response DTOs
│   │   └── service.rs          # Argon2id hashing & JWT token handling
│   │
│   ├── middleware/             # Axum auth verification & tracing middleware
│   ├── routes/                 # Router hierarchy (/health, /api/v1/auth, placeholder domain routes)
│   ├── errors/                 # Centralized AppError enum & Axum IntoResponse implementation
│   ├── models/                 # User & UserRole domain models ('buyer', 'seller', 'rider', 'field_agent', 'admin')
│   ├── services/               # Domain business service layer traits & exports
│   └── utils/                  # Input validation & helper utilities
│
├── migrations/                 # Version-controlled SQLx database migration files
│   └── 20260827000001_create_users_table.sql
│
├── tests/                      # Integration & unit test suite
│   ├── health_test.rs
│   └── auth_test.rs
│
├── .env.example                # Safe example environment configuration
├── .gitignore                  # Git ignore rules
├── Cargo.toml                  # Cargo dependencies manifest
├── docker-compose.yml          # Local PostgreSQL 16 container setup
└── README.md                   # Authoritative backend manual
```

---

## 7. Database Migrations Guide

All database schema modifications MUST go through version-controlled migrations in the `migrations/` directory.

### Migration Naming Convention
Migration files must start with a UTC timestamp `YYYYMMDDHHMMSS` followed by a descriptive name:

```text
migrations/
├── 20260827000001_create_users_table.sql
├── 20260827000002_create_seller_kyc_table.sql
└── 20260827000003_create_orders_table.sql
```

### Migration Rules
1. 🛑 **NEVER** edit an existing migration file that has already been pushed to `develop` or `main`.
2. 🛑 **NEVER** manually alter PostgreSQL tables without a migration script.
3. ✅ **ALWAYS** test migrations locally by restarting the application or using `sqlx migrate run`.

---

## 8. API Routing Structure & Versioning

All application endpoints follow strict API versioning under `/api/v1/`:

| Method | Endpoint | Description | Auth Required |
| :--- | :--- | :--- | :--- |
| `GET` | `/health` | Server infrastructure health check | No |
| `POST` | `/api/v1/auth/register` | Register new user account | No |
| `POST` | `/api/v1/auth/login` | Authenticate email/password & get JWT | No |
| `POST` | `/api/v1/auth/logout` | Client logout request | No |
| `GET` | `/api/v1/auth/me` | Fetch authenticated user profile | Yes (`Bearer <token>`) |

---

## 9. Error Handling Standard

The backend enforces a centralized error response format. All errors implement Axum's `IntoResponse` and return consistent JSON:

```json
{
  "success": false,
  "message": "Human-readable error explanation"
}
```

### Security Rule for Errors
* 🔒 Database connection strings, SQL query errors, stack traces, and internal secrets must **NEVER** be returned to the client in production HTTP responses.
* Internal errors are logged on the server using `tracing::error!` while returning a safe `500 Internal Server Error` message to the user.

---

## 10. Security & Secret Management Rules

1. 🔒 **Never Commit `.env`**: `.env` is listed in `.gitignore`. Never use `git add -f .env`.
2. 🔒 **Password Hashing**: Passwords must be hashed using Argon2id with random salt (`AuthService::hash_password`). Plain-text passwords must never touch the database.
3. 🔒 **No Secrets in Source Code**: Database passwords, JWT secret keys, and third-party API keys must be loaded from environment variables via `AppConfig::from_env()`.
4. 🔒 **Parameterized SQL Queries**: Always use SQLx query parameters (`$1`, `$2`) to prevent SQL injection vulnerabilities.

---

## 11. Git Workflow & Branching Standard

Interns MUST NEVER work or push directly to the `main` or `develop` branches.

### Branch Naming Scheme
```text
main
  └── develop
        ├── feature/<description>
        ├── fix/<description>
        ├── refactor/<description>
        ├── docs/<description>
        └── chore/<description>
```

### Examples
* `feature/seller-kyc-api`
* `feature/product-verification-schema`
* `fix/db-connection-timeout`
* `docs/update-backend-readme`

---

## 12. Conventional Commit Standards

All commit messages MUST follow the Conventional Commits specification.

### Allowed Prefix Types
* `feat:` A new API endpoint or backend feature
* `fix:` A bug fix in existing code
* `docs:` Documentation changes only
* `refactor:` Code change that neither fixes a bug nor adds a feature
* `test:` Adding or updating unit/integration tests
* `chore:` Updating dependencies or build scripts

### Examples
```bash
git commit -m "feat: implement seller kyc registration endpoint"
git commit -m "fix: handle unique constraint error on user registration"
git commit -m "docs: add database setup instructions"
```

❌ **Prohibited commit messages**: `update`, `changes`, `fixed`, `wip`, `final`, `stuff`, `testing`.

---

## 13. Definition of Done Checklist

A backend task is considered **DONE** only when all applicable criteria are met:

```text
[ ] Feature follows agreed modular Rust architecture
[ ] Code compiles cleanly (`cargo check`)
[ ] Code is formatted (`cargo fmt --check`)
[ ] Rust linter passes without warnings (`cargo clippy -- -D warnings`)
[ ] Unit and integration tests pass (`cargo test`)
[ ] API input validation implemented
[ ] Database migration added where required
[ ] No secrets committed or logged
[ ] No unnecessary unwrap() or expect() in production request paths
[ ] API error responses follow the standard JSON error format
[ ] Documentation updated
[ ] PR description completed with test evidence attached
[ ] Mentor/Peer review feedback addressed and approved
```

---

## 14. 13-Step Developer Workflow

Follow this step-by-step workflow for every assigned backend task:

```text
 1. Read Assigned Backend Task / User Story
        ↓
 2. Review Relevant PRD Section
        ↓
 3. Check Frontend API Contract Requirements
        ↓
 4. Check Database Schema & Migration Requirements
        ↓
 5. Ask Mentor if Business Logic is Unclear
        ↓
 6. Create Local Feature Branch (e.g., feature/seller-kyc-api)
        ↓
 7. Write Migration & Implement Handlers, Models & Services
        ↓
 8. Write Unit & Integration Tests in tests/
        ↓
 9. Run Local Verification (cargo fmt && cargo check && cargo clippy && cargo test)
        ↓
10. Open Pull Request to develop Branch
        ↓
11. Participate in Code Review
        ↓
12. Address Reviewer Feedback & Push Fixes
        ↓
13. Merge to develop
```

> 💡 **Important Rule**: If product requirements or API contracts are ambiguous or missing, **do NOT invent business rules**. Immediately seek clarification from your mentor or product team.

---

## 15. Troubleshooting & FAQs

* **PostgreSQL connection refused (`Error: ConnectionRefused`)**: Ensure Docker container is running via `docker compose up -d` and port `5432` is not blocked by another PostgreSQL service.
* **SQLx compilation error on query! macro**: SQLx offline data can be updated using `cargo sqlx prepare` if offline checking is enabled.
* **Axum route compilation error**: Ensure handler signatures match Axum's expected extractor parameters (`State`, `Json`, `Extension`, etc.).