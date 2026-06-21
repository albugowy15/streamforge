# Repository Guidelines

Streamforge is an Nx-managed monorepo with two primary applications: a Rust API and a TanStack Start web app. Prefer workspace-level commands from the repository root so Nx can coordinate targets and caching.

## Product Overview

Streamforge is a small Netflix-like video platform where users upload videos, process them for streaming, and watch them in the web app. Build features around three product goals:

- Resumable video upload: users upload one source video at the highest available resolution, capped at 4K. Uploads must include video metadata and support resuming from the last completed progress point after network loss, browser interruption, or server error instead of restarting from byte zero.
- Video processing: after upload completes, the API should process the source video into HTTP Live Streaming (HLS) assets. Use FFmpeg for transcoding and packaging into the playlists and media chunks required by HLS.
- Streaming playback: users stream uploaded videos from generated HLS assets. Playback should support adaptive bitrate/resolution selection based on bandwidth, manual resolution choices from `360p`, `480p`, `720p`, `1080p`, `2k`, and `4k` up to the uploaded source resolution, play/pause, seeking, volume control, and optionally speed controls such as `0.5x`, `1x`, and `2x`.

Current video metadata is defined by API migrations in `apps/api/migrations`. The initial `videos` table stores `title`, `description`, `categories`, and `visibility`; update migrations when metadata requirements change.

<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

# General Guidelines for working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first - it has patterns for querying projects, targets, and dependencies
- When running tasks (for example build, lint, test, e2e, etc.), always prefer running the task through `nx` (i.e. `nx run`, `nx run-many`, `nx affected`) instead of using the underlying tooling directly
- Prefix nx commands with the workspace's package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to the Nx MCP server and its tools, use them to help the user
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - always check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin configuration, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` skill handles generator discovery internally - don't call nx_docs just to look up generator syntax

<!-- nx configuration end-->

## Project Structure & Module Organization

- `apps/api/` contains the Rust backend application. Source lives in `apps/api/src`, SQLx migrations live in `apps/api/migrations`, and integration tests live in `apps/api/tests`.
- `apps/web/` contains the TanStack Start frontend. App source lives in `apps/web/src`, routes are in `apps/web/src/routes`, shared UI components are in `apps/web/src/components`, and static assets are in `apps/web/public`.
- `packages/generator/` contains the local TypeScript generator used by `pnpm generate`.
- Root files such as `package.json`, `nx.json`, and `tsconfig.base.json` define workspace scripts, Nx target behavior, and shared TypeScript configuration.

## Applications

### API (`apps/api`)

The API is a Rust 2024 project using Axum, Tokio, SQLx/PostgreSQL, S3-compatible storage, tracing, and OpenAPI tooling. Keep domain code organized under `apps/api/src`, add database changes as migrations in `apps/api/migrations`, and place end-to-end coverage in `apps/api/tests`.

Video uploads use S3 multipart upload against RustFS for resumability:

- `POST /api/v1/videos` stores video metadata and starts a multipart upload. The response includes `video_id`, `upload_id`, `bucket`, `object_key`, and `recommended_part_size_bytes`.
- `PUT /api/v1/videos/{id}/parts/{part_number}?upload_id=...` uploads one raw video chunk. Part numbers must be in the S3 range `1..=10000`; the API currently recommends 8 MiB chunks and allows request bodies up to 64 MiB.
- `GET /api/v1/videos/{id}/upload-status?upload_id=...` lists uploaded parts and total uploaded bytes so the web app can resume after interruption.
- `POST /api/v1/videos/{id}/complete-upload` completes the multipart upload. If the request omits parts, the API lists uploaded parts from S3 and completes with those.
- `DELETE /api/v1/videos/{id}/upload?upload_id=...` aborts an unfinished upload.

Run API-specific tasks through Nx, for example:

```bash
pnpm nx run @streamforge/api:dev
pnpm nx run @streamforge/api:test
pnpm nx run @streamforge/api:lint
pnpm nx run @streamforge/api:build
```

### Web (`apps/web`)

The web app is built with TanStack Start, TanStack Router, React, Vite, Tailwind CSS, shadcn-style UI components, Vitest, ESLint, and Prettier. File-based routes belong in `apps/web/src/routes`; do not hand-edit `apps/web/src/routeTree.gen.ts` unless the framework requires it.

Run web-specific tasks through Nx:

```bash
pnpm nx run @streamforge/web:dev
pnpm nx run @streamforge/web:test
pnpm nx run @streamforge/web:typecheck
pnpm nx run @streamforge/web:build
```

## Build, Test, and Development Commands

Use root scripts when you want to run the same target across the monorepo:

```bash
pnpm dev        # run dev targets for available projects
pnpm build      # build all projects with build targets
pnpm test       # run all test targets
pnpm lint       # run all lint targets
pnpm format     # format projects with format targets
pnpm typecheck  # typecheck projects with typecheck targets
pnpm start      # run start targets where configured
pnpm preview    # run preview targets where configured
pnpm generate   # build and run the local generator
```

## Local Docker Infrastructure

The repository includes `docker-compose.yaml` for running the full local stack with Docker. Use it from the workspace root:

```bash
docker compose up -d postgres redis rustfs  # start local dependencies only
docker compose up -d                        # start dependencies plus api and web
docker compose down                         # stop containers, keep named volumes
```

Compose defines two app containers:

- `api` builds `apps/api/Dockerfile`, loads environment variables from `apps/api/.env`, exposes the Rust API on port `5000`, and depends on `postgres` and `rustfs`.
- `web` builds `apps/web/Dockerfile`, exposes the TanStack Start app on port `3000`, and joins the frontend network.

Compose also defines these supporting services:

- `postgres` runs PostgreSQL 18 on port `5432` with database `streamforge` and default local credentials `postgres`/`postgres`. Data persists in the `postgres_data` volume.
- `redis` runs Redis 8.6.3 on port `6379` with append-free snapshot persistence configured by `redis-server --save 60 1`. Data persists in `redis_data`.
- `rustfs` provides S3-compatible object storage for local uploads. The S3 API is on port `9000`, the console is on port `9001`, and local credentials are `rustfsadmin`/`rustfsadmin`. Data persists in `rustfs_data`.

Networking is split into `backend` and `frontend` bridge networks. The API can reach backend dependencies by service name from inside Compose, while the web container only joins the frontend network.

## Coding Style & Naming Conventions

- Use Rust formatting for API code: `pnpm nx run @streamforge/api:format`.
- Treat Rust Clippy warnings as errors; the API lint target runs `cargo clippy -- -D warnings`.
- Use TypeScript, React components, and existing UI primitives in `apps/web/src/components/ui` for frontend work.
- Use Prettier and ESLint for web code. Prefer descriptive kebab-case filenames for route and component files, matching existing patterns such as `image-preview.tsx` and `file-uploader.tsx`.

## Testing Guidelines

- API tests use Rust’s built-in test framework plus `testcontainers` for integration coverage. Add integration tests under `apps/api/tests` and keep focused unit tests near the code they exercise.
- Web tests use Vitest and Testing Library. Keep test files near the feature they cover using conventional names such as `*.test.ts` or `*.test.tsx`.
- Before opening a pull request, run the smallest relevant Nx target first, then use `pnpm test`, `pnpm lint`, and `pnpm build` when the change affects multiple projects.

## Commit & Pull Request Guidelines

Recent history follows Conventional Commits with scopes, for example `feat(api): ...`, `test(api): ...`, `refactor(api): ...`, and `chore(api): ...`. Keep commits scoped to one concern and use the app or package name as the scope when possible.

Pull requests should include a concise description, linked issue when applicable, test results, migration notes for API/database changes, and screenshots or recordings for visible web UI changes.

## Security & Configuration Tips

- Each app owns its own environment file at the root of that app directory. Use `apps/api/.env` for the API and `apps/web/.env` for the web app.
- Before running an app, copy values from that app's checked-in env example file and fill in the required variables. Prefer `apps/<app>/.env.example` when present; the API currently documents its required variables in `apps/api/example.env`.
- For Docker-based local API runs, set `DATABASE_URL`, `S3_REGION`, `S3_ACCESS_KEY_ID`, `S3_SECRET_ACCESS_KEY`, `S3_ENDPOINT_URL`, and `S3_BUCKET` in `apps/api/.env` to match the Compose services; do not prefix every command with env variables when the app `.env` can carry them.
- Never commit app `.env` files or real secrets.
- Do not commit local upload verification media such as `apps/api/sample_video.mp4` or downloaded object-storage copies unless explicitly requested.
- Keep generated artifacts and build outputs out of reviews unless they are intentionally part of the change.
