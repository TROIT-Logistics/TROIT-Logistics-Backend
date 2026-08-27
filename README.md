# TROIT Logistics — Backend MVP Manual & Demo Walkthrough

Welcome to the **TROIT Logistics** MVP backend codebase! This repository houses the REST API server for TROIT Logistics, an e-commerce trust and logistics platform starting with a controlled pilot in Port Harcourt, Nigeria.

This document serves as the **authoritative developer guide and presentation walkthrough manual** for tomorrow's MVP demo.

---

## 1. Core MVP Vertical Slice Story

The MVP backend demonstrates the complete end-to-end trust and logistics transaction lifecycle:

```text
SELLER Registers / Logins
   ↓
SELLER Adds Product (Initial Status: PENDING)
   ↓
PRODUCT VERIFIED (Demo Helper Endpoint PATCH /api/v1/products/:id/verify)
   ↓
BUYER Sees Verified Product (GET /api/v1/products)
   ↓
BUYER Places Order (Status: CONFIRMED, Payment: PROTECTED)
   ↓
PICKUP INSPECTION Recorded (Condition verified, Status: PASSED)
   ↓
ORDER Transitions: READY_FOR_PICKUP → OUT_FOR_DELIVERY → DELIVERED
   ↓
BUYER Confirms Delivery (POST /api/v1/orders/:id/confirm-delivery)
   ↓
ORDER COMPLETED & Protected Payment RELEASED
```

---

## 2. Technology Stack

* **Language**: Rust (2021 edition)
* **Web Framework**: Axum (v0.7+)
* **Async Runtime**: Tokio (v1.0+)
* **Database**: PostgreSQL (v16+)
* **Database Driver & Migrations**: SQLx (v0.8+ with `runtime-tokio-rustls`, `postgres`)
* **Security & Auth**: Argon2id (`argon2`), JWT (`jsonwebtoken`)
* **Serialization & Validation**: Serde (`serde`), Validator (`validator`)
* **Logging & Observability**: Tracing (`tracing`, `tracing-subscriber`)

---

## 3. Quick Start & Database Setup

### Prerequisites
* Rust toolchain (v1.75.0 or higher)
* Docker & Docker Compose

### Setup Commands

1. **Start PostgreSQL Container**:
   ```bash
   docker compose up -d
   ```

2. **Configure Environment Variables**:
   ```bash
   cp .env.example .env
   ```

3. **Run the Backend Server**:
   ```bash
   cargo run
   ```
   *The server will start on `http://0.0.0.0:8000` and automatically execute all database migrations.*

4. **Verify Health Endpoint**:
   ```bash
   curl http://localhost:8000/health
   ```

---

## 4. Development Seed Mechanism

To quickly set up demo data for tomorrow's presentation, run the seed endpoint:

```bash
curl -X POST http://localhost:8000/api/v1/seed
```

### Pre-configured Demo Credentials
* 🏪 **Demo Seller**: `seller@demo.troit` / Password: `DemoPass123!`
* 🛒 **Demo Buyer**: `buyer@demo.troit` / Password: `DemoPass123!`

---

## 5. Step-by-Step MVP Presentation Curl Walkthrough

### Step 1: Register Seller
```bash
curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "seller1@troitlogistics.com",
    "password": "Password123!",
    "full_name": "Port Harcourt Electronics Store",
    "phone_number": "+2348012345678",
    "role": "seller"
  }'
```

### Step 2: Login Seller & Get Token
```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "seller1@troitlogistics.com",
    "password": "Password123!"
  }'
```
*Save the returned `token` as `$SELLER_TOKEN`.*

### Step 3: Seller Adds Product
```bash
curl -X POST http://localhost:8000/api/v1/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SELLER_TOKEN" \
  -d '{
    "name": "iPhone 14 Pro 256GB Deep Purple",
    "description": "Inspected Grade A like-new condition.",
    "price": 650000.00,
    "condition": "Grade A",
    "stock": 5
  }'
```
*Save the returned product `id` as `$PRODUCT_ID`.*

### Step 4: Verify Product (Demo Action)
```bash
curl -X PATCH http://localhost:8000/api/v1/products/$PRODUCT_ID/verify \
  -H "Content-Type: application/json" \
  -d '{
    "verification_status": "VERIFIED"
  }'
```

### Step 5: Register & Login Buyer
```bash
curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "buyer1@troitlogistics.com",
    "password": "Password123!",
    "full_name": "Amaka Okorie",
    "role": "buyer"
  }'
```
*Login buyer and save token as `$BUYER_TOKEN`.*

### Step 6: Buyer Views Verified Products
```bash
curl http://localhost:8000/api/v1/products
```

### Step 7: Buyer Places Order
```bash
curl -X POST http://localhost:8000/api/v1/orders \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $BUYER_TOKEN" \
  -d '{
    "product_id": "'"$PRODUCT_ID"'",
    "quantity": 1
  }'
```
*Order returns `status: CONFIRMED` and `payment_status: PROTECTED`.*
*Save returned order `id` as `$ORDER_ID`.*

### Step 8: Record Pickup Inspection
```bash
curl -X POST http://localhost:8000/api/v1/orders/$ORDER_ID/pickup-inspection \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SELLER_TOKEN" \
  -d '{
    "condition": "Physical condition verified Grade A by Rider. No scratches.",
    "notes": "Serial number matched.",
    "inspection_status": "PASSED"
  }'
```
*Order status transitions to `READY_FOR_PICKUP`.*

### Step 9: Order Transition: Out for Delivery → Delivered
```bash
curl -X PATCH http://localhost:8000/api/v1/orders/$ORDER_ID/status \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SELLER_TOKEN" \
  -d '{ "status": "OUT_FOR_DELIVERY" }'

curl -X PATCH http://localhost:8000/api/v1/orders/$ORDER_ID/status \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SELLER_TOKEN" \
  -d '{ "status": "DELIVERED" }'
```

### Step 10: Buyer Confirms Delivery
```bash
curl -X POST http://localhost:8000/api/v1/orders/$ORDER_ID/confirm-delivery \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $BUYER_TOKEN"
```
*Order status becomes `COMPLETED` and payment status transitions to `RELEASED`!*

---

## 6. Development & Quality Commands

```bash
cargo fmt          # Code formatting
cargo check        # Compilation check
cargo clippy -- -D warnings  # Linter inspection
cargo test         # Run test suite
```