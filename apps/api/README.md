# Streamforge API

Streamforge API is the Rust backend for a small Netflix-like app where users upload and stream videos. The API owns video metadata, resumable source-video uploads, object storage access, and future video processing/streaming workflows.

## Tech Stack

- Rust 2024, Axum, Tokio
- PostgreSQL with SQLx migrations
- RustFS as local S3-compatible object storage through `aws-sdk-s3`
- Utoipa Swagger UI and Redoc for API documentation
- Nx targets for monorepo orchestration

## Architecture

The API follows a layered module pattern:

```text
router -> controller -> service -> repository
```

Current source layout:

```text
apps/api/
├── migrations/       # SQLx migrations
├── src/
│   ├── config.rs     # Required environment variables
│   ├── lib.rs        # App composition, routing, middleware
│   ├── main.rs       # Binary entrypoint and database migrations
│   ├── modules/
│   │   └── videos/   # Video metadata and resumable upload API
│   └── storage/      # PostgreSQL and S3/RustFS clients
└── tests/            # E2E tests; not required for first upload phase
```

## Implementation Progress

### Implemented

- Health endpoint.
- Video metadata creation with `title`, `description`, `categories`, and `visibility`.
- Resumable source-video upload using S3 multipart upload against RustFS.
- Upload status lookup by S3 `upload_id` for frontend resume support.
- Upload completion and abort endpoints.
- Automatic S3 bucket creation for the configured `S3_BUCKET`.
- 8 MiB recommended part size and 64 MiB API body limit for upload chunks.
- Unit tests for DTO validation and video service upload behavior.

### To Do

- Enforce source video constraints, including max 4K resolution and supported file types.
- Persist upload/object state beyond S3 multipart state, including upload lifecycle and original object key.
- Add FFmpeg-based processing jobs to generate HLS playlists and media chunks.
- Add streaming endpoints for HLS manifests and segments.
- Add adaptive streaming metadata for 360p, 480p, 720p, 1080p, 2K, and 4K.
- Add authentication/authorization and per-user video ownership.
- Add E2E coverage for the upload lifecycle when Docker is available.

## Local Configuration

Each app owns its env file at the app root. For the API, use:

```bash
cp apps/api/example.env apps/api/.env
```

Required variables:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/streamforge
S3_REGION=us-east-1
S3_ACCESS_KEY_ID=rustfsadmin
S3_SECRET_ACCESS_KEY=rustfsadmin
S3_ENDPOINT_URL=http://localhost:9000
S3_BUCKET=streamforge-videos
```

Start local dependencies:

```bash
docker compose up -d postgres rustfs
```

Run the API from the workspace root:

```bash
pnpm nx run @streamforge/api:dev
```

## Commands

```bash
pnpm nx run @streamforge/api:dev     # run API locally
pnpm nx run @streamforge/api:build   # release build
pnpm nx run @streamforge/api:test    # cargo test
pnpm nx run @streamforge/api:lint    # cargo clippy -- -D warnings
pnpm nx run @streamforge/api:format  # cargo fmt
```

## REST API

Base URL for local development:

```text
http://localhost:5000/api/v1
```

Generated docs are also available when the API is running:

- Swagger UI: `http://localhost:5000/swagger-ui`
- Redoc: `http://localhost:5000/redoc`

### Health

```bash
curl -i http://localhost:5000/health
```

Response:

```http
HTTP/1.1 200 OK
```

### Create Video and Start Upload

Stores metadata and starts an S3 multipart upload.

```bash
curl -X POST http://localhost:5000/api/v1/videos \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Sample upload video for resumable upload test",
    "description": "This description is intentionally long enough to satisfy the API validation rules for creating a video upload session.",
    "visibility": "public",
    "categories": ["action", "comedy"],
    "file_name": "sample_video.mp4",
    "content_type": "video/mp4"
  }'
```

Response:

```json
{
  "data": {
    "video_id": "019eea41-c0cb-73d6-b07b-adbe7eb94055",
    "upload_id": "multipart-upload-id",
    "bucket": "streamforge-videos",
    "object_key": "videos/019eea41-c0cb-73d6-b07b-adbe7eb94055/source",
    "recommended_part_size_bytes": 8388608
  }
}
```

### Upload a Part

Upload raw bytes for one part. Part numbers must be between `1` and `10000`.

```bash
curl -X PUT \
  'http://localhost:5000/api/v1/videos/{video_id}/parts/1?upload_id={upload_id}' \
  -H 'Content-Type: application/octet-stream' \
  --data-binary '@/path/to/part-0001'
```

Response:

```json
{
  "data": {
    "video_id": "019eea41-c0cb-73d6-b07b-adbe7eb94055",
    "upload_id": "multipart-upload-id",
    "part_number": 1,
    "etag": "\"7e33582bf7f1622a1133be3ce600c6db\"",
    "size_bytes": 8388608
  }
}
```

### Check Upload Status

Use this after an interruption to determine which parts already reached object storage.

```bash
curl 'http://localhost:5000/api/v1/videos/{video_id}/upload-status?upload_id={upload_id}'
```

Response:

```json
{
  "data": {
    "video_id": "019eea41-c0cb-73d6-b07b-adbe7eb94055",
    "upload_id": "multipart-upload-id",
    "object_key": "videos/019eea41-c0cb-73d6-b07b-adbe7eb94055/source",
    "uploaded_parts": [
      {
        "part_number": 1,
        "etag": "\"7e33582bf7f1622a1133be3ce600c6db\"",
        "size_bytes": 8388608
      }
    ],
    "uploaded_bytes": 8388608,
    "next_part_number": 2
  }
}
```

### Complete Upload

Complete the multipart upload after all parts are uploaded. You can provide the uploaded part list explicitly:

```bash
curl -X POST http://localhost:5000/api/v1/videos/{video_id}/complete-upload \
  -H 'Content-Type: application/json' \
  -d '{
    "upload_id": "multipart-upload-id",
    "parts": [
      {
        "part_number": 1,
        "etag": "\"7e33582bf7f1622a1133be3ce600c6db\""
      }
    ]
  }'
```

If `parts` is omitted, the API lists parts from S3 and completes with those.

Response:

```json
{
  "data": {
    "video_id": "019eea41-c0cb-73d6-b07b-adbe7eb94055",
    "upload_id": "multipart-upload-id",
    "bucket": "streamforge-videos",
    "object_key": "videos/019eea41-c0cb-73d6-b07b-adbe7eb94055/source",
    "etag": "\"6274c24083c616befe2c724394e04785-2\""
  }
}
```

### Abort Upload

Abort an unfinished multipart upload.

```bash
curl -X DELETE \
  'http://localhost:5000/api/v1/videos/{video_id}/upload?upload_id={upload_id}'
```

Response:

```json
{
  "data": {
    "video_id": "019eea41-c0cb-73d6-b07b-adbe7eb94055",
    "upload_id": "multipart-upload-id",
    "object_key": "videos/019eea41-c0cb-73d6-b07b-adbe7eb94055/source",
    "aborted": true
  }
}
```

## Manual Upload Verification

The resumable upload flow has been verified with `apps/api/sample_video.mp4` using `curl`. The object downloaded from RustFS matched the source file byte-for-byte by SHA-256.
