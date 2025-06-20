# README.md

// filepath: README.md

# SRS Template

A full-stack web application template using **SvelteKit** (frontend), **Rust** (backend), and **PostgreSQL** (database). This project is structured as a monorepo for easy development and deployment using Docker and Docker Compose.

---

## Stack

- **Frontend:** [SvelteKit](https://kit.svelte.dev/) ([apps/frontend](apps/frontend))
- **Backend:** [Rust](https://www.rust-lang.org/) ([apps/backend](apps/backend))
- **Database:** [PostgreSQL](https://www.postgresql.org/)

---

## Project Structure

```
.
├── apps/
│   ├── backend/      # Rust backend service
│   └── frontend/     # SvelteKit frontend app
├── docker-compose.yml
├── README.md
└── ...
```

---

## Getting Started

### Prerequisites

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

---

### Development

1. **Clone the repository:**

   ```sh
   git clone <repo-url>
   cd SRS-template
   ```

2. **Start all services:**

   ```sh
   docker-compose up --build
   ```

   - Frontend: [http://localhost:5173](http://localhost:5173)
   - Backend: [http://localhost:8080](http://localhost:8080)
   - PostgreSQL: [localhost:5432](http://localhost:5432)

3. **Environment Variables:**
   - Backend: Configure in [`apps/backend/.env`](apps/backend/.env)
   - Database credentials are set in [`docker-compose.yml`](docker-compose.yml)

---

## Useful Commands

- **Build and run all services:**  
  `docker-compose up --build`

- **Stop all services:**  
  `docker-compose down`

- **Run frontend locally (outside Docker):**

  ```sh
  cd apps/frontend
  npm install
  npm run dev
  ```

- **Run backend locally (outside Docker):**
  ```sh
  cd apps/backend
  cargo run
  ```

---

## Notes

- The backend Rust service connects to PostgreSQL using credentials from the `.env` file.
- The frontend is built with SvelteKit and uses Vite for development.
- All services are containerized for easy deployment.

---

## License

MIT
