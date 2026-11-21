# DOKU Server HTTP API

Standalone Axum microservice wrapping `doku-sdk` for payment operations.

## Endpoints

### 1. Health Probe
- `GET /health` or `GET /api/v1/health`
- Response: `200 OK`
  ```json
  {
    "status": "ok",
    "service": "doku-server",
    "version": "0.1.0",
    "sandbox": true,
    "mock": false
  }
  ```

### 2. Initiate Checkout
- `POST /api/v1/checkout`
- Request:
  ```json
  {
    "invoice_number": "INV-12345",
    "amount": 50000,
    "currency": "IDR",
    "callback_url": "http://localhost:3000/track?inv=INV-12345",
    "auto_redirect": true,
    "payment_due_date": 60,
    "customer_name": "Anton",
    "customer_email": "anton@example.com",
    "customer_phone": "08123456789",
    "line_items": [
      {
        "name": "50 Diamonds",
        "price": 50000,
        "quantity": 1
      }
    ]
  }
  ```
- Response: `200 OK`
  ```json
  {
    "success": true,
    "invoice_number": "INV-12345",
    "payment_url": "https://api-sandbox.doku.com/checkout/link/...",
    "token": "token-xyz"
  }
  ```

### 3. Check Order Status
- `GET /api/v1/orders/{invoice}/status`
- Response: `200 OK`
  ```json
  {
    "success": true,
    "invoice_number": "INV-12345",
    "status": "SUCCESS",
    "amount": 50000,
    "date": "2026-11-21T10:00:00Z"
  }
  ```

### 4. Verify Webhook Notification
- `POST /api/v1/notifications/verify`
- Request:
  ```json
  {
    "client_id": "MCH-0001-DEMO",
    "request_id": "req-uuid",
    "timestamp": "2026-11-21T10:00:00Z",
    "signature": "HMACSHA256=...",
    "path": "/api/v1/payments/doku/notification",
    "body": "{\"order\":{\"invoice_number\":\"INV-12345\",\"amount\":50000},\"transaction\":{\"status\":\"SUCCESS\"}}"
  }
  ```
- Response: `200 OK`
  ```json
  {
    "valid": true,
    "invoice_number": "INV-12345",
    "status": "SUCCESS",
    "amount": 50000,
    "date": "..."
  }
  ```
