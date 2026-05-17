# Streamforge Project Context

## Project Overview

Streamforge is a full-stack monorepo managed with **pnpm workspaces** and **Nx**. The project uses a modern technology stack separated into a frontend web application and a backend API service.

### Main Technologies
*   **Monorepo Management:** Nx (`nx`), pnpm workspaces.
*   **Frontend (`apps/web`):**
    *   **Core:** React 19, TypeScript, Vite 8.
    *   **Routing & State:** TanStack Router, TanStack Query, TanStack Form.
    *   **Styling & UI:** Tailwind CSS v4, Base UI, Shadcn, class-variance-authority, Tailwind Merge, clsx, Lucide React.
    *   **Testing:** Vitest, Testing Library.
    *   **Other Tools:** Zod for validation, Sonner for toasts, React Player.
*   **Backend (`apps/api`):**
    *   **Core:** Rust (Edition 2024).
    *   **Web Framework:** Axum.
    *   **Async Runtime:** Tokio.
    *   **Database:** PostgreSQL (via `sqlx`).
    *   **Cloud Storage:** AWS SDK for S3.
    *   **Logging:** Tracing.

### Architecture Notes
*   **Backend:** Adheres to Clean Architecture principles (indicated by internal skill guidelines under `apps/api/.agents/skills/clean-architecture`).
*   **Frontend:** Follows Vercel's React Best Practices and Composition Patterns (indicated by `apps/web/.agents/skills`). Uses modern React patterns with Server Components readiness and TanStack ecosystem.

## Building and Running

Since this is an Nx workspace, you should generally run commands via Nx to utilize caching and the dependency graph. Prefix commands with the package manager (`pnpm`).

### Global Commands (via Nx)
*   **Development Servers:** `pnpm nx run-many -t dev` (or `pnpm dev`)
*   **Build Everything:** `pnpm nx run-many -t build` (or `pnpm build`)
*   **Run All Tests:** `pnpm nx run-many -t test` (or `pnpm test`)
*   **Linting:** `pnpm nx run-many -t lint` (or `pnpm lint`)
*   **Typechecking:** `pnpm nx run-many -t typecheck` (or `pnpm typecheck`)
*   **Formatting:** `pnpm nx run-many -t format`

### Project-Specific Commands
To run tasks for a specific app (e.g., `web` or `api`), use the Nx `<target> <project>` syntax or `run`:
*   `pnpm nx dev web`
*   `pnpm nx build api`
*   `pnpm nx test web`

## Development Conventions

*   **Nx Usage:** 
    *   Always prefer running tasks through `nx` (e.g., `pnpm nx build`) rather than underlying tools directly.
    *   Never guess CLI flags for Nx; check `nx_docs` or `--help` if unsure.
*   **Frontend (`apps/web`):**
    *   Strict TypeScript typing is enforced (`tsc --noEmit` used for typechecking).
    *   Code formatting is enforced via Prettier and ESLint.
    *   Components use Tailwind CSS and often follow Shadcn UI/Radix UI patterns.
*   **Backend (`apps/api`):**
    *   Uses standard Rust tooling (`cargo`, `rustfmt`, `clippy`).
    *   Environment variables are likely managed via `.env` (using `dotenvy`).
    *   Database migrations are managed by `sqlx`.

*(Note: These conventions are inferred from configuration and dependencies. Check specific app directories for further isolated documentation.)*
