set dotenv-load
set dotenv-path := "backend/.env"

dev-backend:
    cd backend && cargo watch -x run

dev-frontend:
    cd frontend && npm run dev

build:
    cd backend && cargo build --release
    cd frontend && npm run build