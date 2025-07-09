# Rusty Chat

A modern, high-performance chat application built with Rust and Actix Web, featuring PostgreSQL database integration and a modular architecture.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Prerequisites](#prerequisites)
4. [Installation & Setup](#installation--setup)
5. [Project Structure](#project-structure)
6. [Configuration](#configuration)
7. [Database Setup](#database-setup)
8. [API Documentation](#api-documentation)
9. [Development Guide](#development-guide)
10. [Contributing](#contributing)
12. [Deployment](#deployment)
13. [Troubleshooting](#troubleshooting)

## Project Overview

Rusty Chat is a scalable real-time chat application that demonstrates modern Rust web development practices. It features:

- **High Performance**: Built with Actix Web for maximum throughput
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Database Integration**: PostgreSQL with SQLx for async database operations
- **Modular Architecture**: Clean separation of concerns for maintainability
- **Configuration Management**: Environment-based configuration system
- **Migration Support**: Database schema versioning with SQLx migrations

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Client/UI     │───▶│   Actix Web      │───▶│   PostgreSQL    │
│                 │    │   (HTTP Server)  │    │   (Database)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │   Application    │
                    │   Modules        │
                    └──────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Handlers   │    │   Models    │    │   Routes    │
│             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │   Database       │
                    │   Connection     │
                    └──────────────────┘
```

## Prerequisites

- **Rust** 1.70 or higher
- **PostgreSQL** 15 or higher
- **Git** for version control
- **Docker** (optional, for containerized development)

### 1. Clone the Repository

```bash
git clone https://github.com/morelucks/rusty-chat.git
cd rusty-chat
```

### 2. Environment Setup

Create environment configuration files:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
# Application Environment
APP_ENV=development

# Database URLs
LOCAL_DATABASE_URL=postgresql://rusty_root:password@localhost:5432/rustychat
PROD_DATABASE_URL=postgresql://username:password@prod-host:5432/rustychat_prod

# Server Configuration
APP__SERVER__HOST=127.0.0.1
APP__SERVER__PORT=8080

# Database Pool Configuration
APP__DATABASE__MAX_CONNECTIONS=20
APP__DATABASE__MIN_CONNECTIONS=5
APP__DATABASE__CONNECTION_TIMEOUT=30
APP__DATABASE__IDLE_TIMEOUT=600

# Logging
RUST_LOG=debug
RUST_BACKTRACE=1
```

### 3. Database Setup

#### Create Database and User

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE rustychat;

# Create user
CREATE USER rusty_root WITH ENCRYPTED PASSWORD 'password';

# Grant permissions
GRANT ALL PRIVILEGES ON DATABASE rustychat TO rusty_root;
GRANT USAGE ON SCHEMA public TO rusty_root;
GRANT CREATE ON SCHEMA public TO rusty_root;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO rusty_root;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO rusty_root;

# Set default privileges
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO rusty_root;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO rusty_root;

\q
```

### 4. Build and Run

```bash
# Build the project
cargo build

# Run the application
cargo run

# For development with auto-reload
cargo install --locked watchexec-cli
watchexec -e rs -r cargo run 
```

The server will start on `http://127.0.0.1:8080`

## Project Structure

```
rusty-chat/
├── 📁 migrations/                  # Database migrations
│   └── 📄 001_initial_schema.sql
├── 📁 src/
│   ├── 📄 main.rs                 # Application entry point
│   ├── 📄 lib.rs                  # Library exports
│   ├── 📁 config/                 # Configuration management
│   │   ├── 📄 mod.rs             # Module exports
│   │   └── 📄 settings.rs        # Application settings
│   ├── 📁 database/               # Database layer
│   │   ├── 📄 mod.rs             # Database module exports
│   │   └── 📄 connection.rs      # Connection pool & migrations
│   ├── 📁 handlers/               # Request handlers
│   │   ├── 📄 mod.rs             # Handler exports
│   │   └── 📄 users.rs           # User-related handlers
│   ├── 📁 models/                 # Data models
│   │   ├── 📄 mod.rs             # Model exports
│   │   ├── 📄 user.rs            # User model
│   │   └── 📄 message.rs         # Message model
│   ├── 📁 routes/                 # Route configuration
│   │   ├── 📄 mod.rs             # Route exports
│   │   └── 📄 api.rs             # API routes
│   └── 📁 utils/                  # Utility functions
│       ├── 📄 mod.rs             # Utility exports
│       └── 📄 helpers.rs         # Helper functions
├── 📁 target/                     # Build artifacts
├── 📄 .env                        # Environment variables
├── 📄 .env.example               # Environment template
├── 📄 .gitignore                 # Git ignore rules
├── 📄 Cargo.toml                 # Rust dependencies
├── 📄 Cargo.lock                 # Dependency lock file
└── 📄 README.md                  # This file
```

### Module Responsibilities

| Module | Purpose | Key Files |
|--------|---------|-----------|
| `config` | Application configuration management | `settings.rs` |
| `database` | Database connections and migrations | `connection.rs` |
| `handlers` | HTTP request processing | `users.rs`, `messages.rs` |
| `models` | Data structures and database operations | `user.rs`, `message.rs` |
| `routes` | URL routing and endpoint configuration | `api.rs` |
| `utils` | Shared utilities and helper functions | `helpers.rs` |

## Configuration

### Environment Variables

The application uses environment-based configuration with the following structure:

```env
# Application Environment (development/production)
APP_ENV=development

# Database Configuration
LOCAL_DATABASE_URL=postgresql://user:pass@localhost:5432/dbname
PROD_DATABASE_URL=postgresql://user:pass@host:5432/dbname

# Server Configuration
APP__SERVER__HOST=127.0.0.1
APP__SERVER__PORT=8080

# Database Pool Settings
APP__DATABASE__MAX_CONNECTIONS=20
APP__DATABASE__MIN_CONNECTIONS=5
APP__DATABASE__CONNECTION_TIMEOUT=30
APP__DATABASE__IDLE_TIMEOUT=600
```

### Configuration Loading

The application loads configuration through the `AppConfig::from_env()` method:

1. **Environment Detection**: Reads `APP_ENV` to determine environment
2. **Database URL Selection**: Chooses appropriate database URL based on environment
3. **Default Values**: Sets sensible defaults for all configuration options
4. **Validation**: Validates configuration values (e.g., min_connections ≤ max_connections)
5. **Environment Override**: Allows overriding defaults with environment variables

## Database Setup

### Schema Overview

The application uses PostgreSQL with the following core tables:

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_messages_user_id ON messages(user_id);
CREATE INDEX idx_messages_created_at ON messages(created_at);
```

### Migration Management

```bash
# Create a new migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Connection Pool Configuration

The application uses SQLx connection pooling with these settings:

- **Max Connections**: 20 (configurable)
- **Min Connections**: 5 (configurable)
- **Connection Timeout**: 30 seconds
- **Idle Timeout**: 600 seconds

## API Documentation

### Base URL

```
http://127.0.0.1:8080/api/v1
```

### Endpoints

#### Users

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/users` | Get all users | `200 OK` with user list |
| `POST` | `/users` | Create new user | `201 Created` with user data |
| `GET` | `/users/{id}` | Get user by ID | `200 OK` with user data |
| `PUT` | `/users/{id}` | Update user | `200 OK` with updated user |
| `DELETE` | `/users/{id}` | Delete user | `204 No Content` |

#### Messages

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/messages` | Get recent messages | `200 OK` with message list |
| `POST` | `/messages` | Create new message | `201 Created` with message data |
| `GET` | `/messages/{id}` | Get message by ID | `200 OK` with message data |
| `DELETE` | `/messages/{id}` | Delete message | `204 No Content` |

### Response Format

All API responses follow this structure:

```json
{
    "success": true,
    "data": {
        // Response data here
    },
    "message": "Optional message",
    "timestamp": "2024-01-01T12:00:00Z"
}
```

### Error Responses

```json
{
    "success": false,
    "error": {
        "code": "ERROR_CODE",
        "message": "Human readable error message"
    },
    "timestamp": "2024-01-01T12:00:00Z"
}
```

## Development Guide

### Adding New Features

Follow these steps to add new features while maintaining the modular structure:

#### 1. Create Model (if needed)

```bash
# Create new model file
touch src/models/your_model.rs
```

Add your model to `src/models/mod.rs`:

```rust
pub mod user;
pub mod message;
pub mod your_model;  // Add this line
```

#### 2. Create Handler

```bash
# Create handler file
touch src/handlers/your_handler.rs
```

Add handler to `src/handlers/mod.rs`:

```rust
pub mod users;
pub mod your_handler;  // Add this line
```

#### 3. Add Routes

Update `src/routes/api.rs`:

```rust
use crate::handlers;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(/* existing routes */),
    )
    .service(
        web::scope("/your-endpoint")  // Add new scope
            .service(/* your routes */),
    );
}
```

#### 4. Update Database (if needed)

```bash
# Create migration
sqlx migrate add add_your_table

# Edit the migration file
# Run migration
sqlx migrate run
```

### Code Style Guidelines

#### 1. File Organization

- **One main struct per file**
- **Related functions grouped together**
- **Clear module boundaries**
- **Consistent naming conventions**

#### 2. Error Handling

```rust
// Use Result<T, E> for fallible operations
pub async fn create_user(pool: &DbPool, user: CreateUser) -> Result<User, sqlx::Error> {
    // Implementation
}

// Use proper error logging
.map_err(|e| {
    error!("Failed to create user: {}", e);
    actix_web::error::ErrorInternalServerError("Failed to create user")
})?
```

#### 3. Database Operations

```rust
// Use sqlx::query_as! for type-safe queries
let users = sqlx::query_as!(
    User,
    "SELECT id, username, email, created_at, updated_at FROM users WHERE active = $1",
    true
)
.fetch_all(pool)
.await?;
```

#### 4. Handler Structure

```rust
use actix_web::{web, HttpResponse, Result};
use tracing::error;

pub async fn your_handler(
    pool: web::Data<DbPool>,
    // other parameters
) -> Result<HttpResponse> {
    // 1. Extract and validate parameters
    // 2. Call model methods
    // 3. Handle errors appropriately
    // 4. Return properly formatted response
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(data)))
}
```

## Contributing

We welcome contributions! Please follow these guidelines:

### 1. Fork and Clone

```bash
git clone https://github.com/morelucks/rusty-chat.git
cd rusty-chat
```

### 2. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Follow Code Standards

- **Run tests**: `cargo test`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`
- **Check build**: `cargo build`

### 4. Commit Guidelines

Use conventional commits:

```bash
git commit -m "feat: add user authentication"
git commit -m "fix: resolve database connection issue"
git commit -m "docs: update API documentation"
```

### 5. Submit Pull Request

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Ensure all tests pass**
4. **Update CHANGELOG.md**
5. **Submit PR** with clear description

### Code Review Process

1. **Automated checks** must pass
2. **At least one reviewer** approval required
3. **No merge conflicts**
4. **Documentation updated** if needed

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_user_creation

# Run tests with output
cargo test -- --nocapture

# Run tests in specific module
cargo test handlers::users
```

### Test Database Setup

```bash
# Create test database
createdb rustychat_test

# Set test environment
export TEST_DATABASE_URL=postgresql://rusty_root:password@localhost:5432/rustychat_test

# Run migrations for test database
sqlx migrate run --database-url $TEST_DATABASE_URL
```

## Deployment

### Production Environment

#### 1. Environment Configuration

```env
APP_ENV=production
PROD_DATABASE_URL=postgresql://prod_user:secure_password@prod-host:5432/rustychat_prod
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=8080
RUST_LOG=info
```

#### 2. Database Setup

```sql
-- Create production database
CREATE DATABASE rustychat_prod;

-- Create production user
CREATE USER prod_user WITH ENCRYPTED PASSWORD 'secure_password';

-- Grant minimal required permissions
GRANT CONNECT ON DATABASE rustychat_prod TO prod_user;
GRANT USAGE ON SCHEMA public TO prod_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO prod_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO prod_user;
```

#### 3. Build for Production

```bash
# Build optimized binary
cargo build --release

# The binary will be in target/release/rusty-chat
```

#### 4. Systemd Service (Linux)

```ini
# /etc/systemd/system/rusty-chat.service
[Unit]
Description=Rusty Chat Application
After=network.target postgresql.service

[Service]
Type=simple
User=rusty-chat
Group=rusty-chat
WorkingDirectory=/opt/rusty-chat
ExecStart=/opt/rusty-chat/rusty-chat
Restart=always
RestartSec=5
Environment=APP_ENV=production
EnvironmentFile=/opt/rusty-chat/.env

[Install]
WantedBy=multi-user.target
```

## Troubleshooting

### Common Issues

#### 1. Database Connection Failed

```
Error: Failed to create database pool: connection failed
```

**Solutions:**
- Check PostgreSQL is running: `pg_isready`
- Verify database URL in `.env`
- Check database user permissions
- Ensure database exists

#### 2. Migration Permission Denied

```
Error: permission denied for schema public
```

**Solutions:**
```sql
-- Grant schema permissions
GRANT USAGE ON SCHEMA public TO your_user;
GRANT CREATE ON SCHEMA public TO your_user;
```

#### 3. Port Already in Use

```
Error: Address already in use (os error 48)
```

**Solutions:**
- Change port in `.env`: `APP__SERVER__PORT=8081`
- Kill process using port: `lsof -ti:8080 | xargs kill`

#### 4. Environment Configuration

```
Error: APP_ENV: environment variable not found
```

**Solutions:**
- Ensure `.env` file exists
- Check environment variable names
- Verify `dotenv` is loaded in `main.rs`

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

### Health Check

```bash
# Check if server is running
curl http://localhost:8080/

# Check database connection
curl http://localhost:8080/api/v1/health
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: Check this README and inline code documentation
- **Issues**: Create an issue on GitHub
- **Discussions**: Use GitHub Discussions for questions
- **Email**: [manbankat@gmail.com](mailto:manbankat@gmail.com)

---

**Happy coding! 🦀**