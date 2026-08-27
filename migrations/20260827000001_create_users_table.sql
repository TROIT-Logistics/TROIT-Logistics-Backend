-- Initial Database Migration for TROIT Logistics
-- Sets up pgcrypto/uuid extension, user_role enum, and users table foundation

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Define valid user roles matching the PRD specification
CREATE TYPE user_role AS ENUM (
    'buyer',
    'seller',
    'rider',
    'field_agent',
    'admin'
);

-- Core users table foundation
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    phone_number VARCHAR(50),
    role user_role NOT NULL DEFAULT 'buyer',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index on email for fast authentication lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Index on role for authorization queries
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
