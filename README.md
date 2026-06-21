# Streamforge

Streamforge is an Nx-managed monorepo for a video upload and streaming platform. It is built around three core capabilities:

1. Resumable file upload for large source videos.
2. Video processing with FFmpeg.
3. Live streaming based on the HTTP Live Streaming (HLS) protocol.

It has two applications:

- `apps/api`: a Rust backend that handles video metadata, resumable uploads, storage, and future HLS processing.
- `apps/web`: a TanStack Start frontend for uploading and streaming videos.

## Core Features

### Resumable File Upload

Users upload one source video along with its metadata. Because source files can be large, the API uses S3 multipart upload against RustFS so interrupted uploads can continue from the last completed part instead of starting again from byte zero.

The upload flow is designed for unstable networks and browser interruptions. The API exposes endpoints to create an upload session, send individual parts, inspect upload progress, complete the upload, and abort unfinished uploads.

### Video Processing With FFmpeg

After a source upload completes, the API will process the uploaded video with FFmpeg. The processing pipeline will generate streaming-ready outputs, including the manifests and media segments required for adaptive playback.

This work is intended to convert one uploaded source file into multiple delivery assets without asking the user to re-upload or manually transcode anything.

### Live Streaming With HLS

Streamforge serves playback through HTTP Live Streaming. HLS lets the client choose an appropriate rendition based on network conditions and switch resolution during playback when needed.

The target experience includes adaptive streaming, manual quality selection, seek support, play/pause, volume control, and optional playback-speed controls.

## Repository Layout

- `apps/api/` contains the Rust API, SQLx migrations, and backend-specific docs.
- `apps/web/` contains the TanStack Start app, routes, components, and public assets.
- `packages/generator/` contains the local Nx generator used by `pnpm generate`.
- Root files such as `package.json`, `nx.json`, and `docker-compose.yaml` define workspace scripts and local infrastructure.

## Workspace Commands

Run tasks from the repository root so Nx can coordinate caching and project dependencies:

```bash
pnpm dev        # run dev targets across the workspace
pnpm build      # build all projects with a build target
pnpm test       # run workspace tests
pnpm lint       # run workspace lint targets
pnpm typecheck  # run workspace type checks
pnpm start      # run start targets
pnpm generate   # build and run the local generator
```

To run one app directly with Nx:

```bash
pnpm nx run @streamforge/api:dev
pnpm nx run @streamforge/web:dev
```

If Nx fails with plugin-worker connection errors in a sandboxed environment, disable plugin isolation for that command:

```bash
NX_ISOLATE_PLUGINS=false pnpm nx run @streamforge/web:dev
```

## Local Infrastructure

The project is designed to run locally with Docker:

```bash
docker compose up -d postgres redis rustfs
docker compose up -d
```

- `postgres` stores application data.
- `redis` supports cache or background-work workloads.
- `rustfs` provides S3-compatible object storage for uploaded video files.
- `api` and `web` are the application containers.

## Configuration

Each app owns its own `.env` file at the app root:

- `apps/api/.env`
- `apps/web/.env`

Copy from the example file in the same app directory before running locally. Do not commit real secrets.

## Project Docs

- API guide: [apps/api/README.md](apps/api/README.md)
- Web guide: [apps/web/README.md](apps/web/README.md)
- Contributor rules: [AGENTS.md](AGENTS.md)
