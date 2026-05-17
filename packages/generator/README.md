# Streamforge Generator

A centralized CLI tool for scaffolding and code generation across the Streamforge monorepo.

## Usage

The generator is accessible from the workspace root via `pnpm`:

```bash
pnpm generate <scope> <action> [arguments]
```

## Available Scopes

### API (`api`)

Used for scaffolding components within the Rust backend.

#### Actions:
*   **`init-module`**: Scaffolds a complete module directory structure following Clean Architecture principles.
    *   **Argument**: `<module_name>` (Required)
    *   **Example**: `pnpm generate api init-module authors`

## Technical Details

*   **Runtime**: Node.js
*   **Build Tool**: [tsdown](https://github.com/ts-down/tsdown) (Rolldown-based bundler)
*   **Language**: TypeScript (with `Bundler` module resolution for clean imports)

## Adding New Generators

1.  Create or update the relevant scope directory in `src/scopes/<scope_name>`.
2.  Implement your action logic as a modular function.
3.  Register the action in the scope's `index.ts`.
4.  Export the scope in the main `src/index.ts`.
