import fs from 'fs';
import path from 'path';

export async function initModule(args: string[]) {
  const moduleName = args[0];

  if (!moduleName) {
    console.error('Error: Module name is required for init-module action.');
    console.error('Usage: streamforge-gen api init-module <module_name>');
    process.exit(1);
  }

  const toSnakeCase = (str: string) => str.toLowerCase();
  const toPascalCase = (str: string) => str.replace(/(^|_)([a-z])/g, (_, __, c) => c.toUpperCase());
  const toSingular = (str: string) => str.replace(/s$/, '');

  const snakeModule = toSnakeCase(moduleName);
  const snakeSingular = toSingular(snakeModule);
  const pascalModule = toPascalCase(snakeModule);
  const pascalSingular = toPascalCase(snakeSingular);

  // In an Nx monorepo, we can assume the command is run from the root
  // or we can find the root. For now, let's assume it targets apps/api relative to the current working directory if run from root.
  // Better: search for the root directory by looking for pnpm-workspace.yaml
  
  const findRoot = (dir: string): string => {
    if (fs.existsSync(path.join(dir, 'pnpm-workspace.yaml'))) {
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) {
      throw new Error('Could not find workspace root (pnpm-workspace.yaml)');
    }
    return findRoot(parent);
  };

  const rootDir = findRoot(process.cwd());
  const apiSrcDir = path.join(rootDir, 'apps', 'api', 'src');
  const moduleDir = path.join(apiSrcDir, 'modules', snakeModule);

  if (fs.existsSync(moduleDir)) {
    console.error(`Error: Module directory already exists: ${moduleDir}`);
    process.exit(1);
  }

  console.log(`Scaffolding module: ${snakeModule} in ${moduleDir}`);

  fs.mkdirSync(moduleDir, { recursive: true });

  const files = {
    'mod.rs': `mod controller;
mod models;
mod repository;
mod router;
mod service;

pub use {
    repository::Postgres${pascalModule}Repository, router::${pascalModule}Router,
    service::${pascalModule}Service,
};`,

    'models.rs': `use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ${pascalSingular} {
    pub id: Option<i64>,
    pub name: String,
}

impl ${pascalSingular} {
    pub fn new(id: Option<i64>, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create${pascalSingular}Request {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ${pascalSingular}Response {
    pub id: i64,
    pub name: String,
}

impl From<${pascalSingular}> for ${pascalSingular}Response {
    fn from(item: ${pascalSingular}) -> Self {
        Self {
            id: item.id.unwrap_or(0),
            name: item.name,
        }
    }
}`,

    'repository.rs': `use std::sync::Arc;
use async_trait::async_trait;
use crate::storage::PostgresDatabase;
use super::models::${pascalSingular};

#[async_trait]
pub trait ${pascalModule}Repository: Send + Sync {
    async fn create(&self, item: ${pascalSingular}) -> Result<${pascalSingular}, String>;
    async fn list(&self) -> Result<Vec<${pascalSingular}>, String>;
}

pub struct Postgres${pascalModule}Repository {
    db: Arc<PostgresDatabase>,
}

impl Postgres${pascalModule}Repository {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ${pascalModule}Repository for Postgres${pascalModule}Repository {
    async fn create(&self, item: ${pascalSingular}) -> Result<${pascalSingular}, String> {
        let mut created = item;
        created.id = Some(1);
        Ok(created)
    }

    async fn list(&self) -> Result<Vec<${pascalSingular}>, String> {
        Ok(vec![${pascalSingular}::new(Some(1), "Scaffolded Item".to_string())])
    }
}`,

    'service.rs': `use std::sync::Arc;
use super::models::{ ${pascalSingular}, ${pascalSingular}Response, Create${pascalSingular}Request };
use super::repository::${pascalModule}Repository;

pub struct ${pascalModule}Service {
    repository: Arc<dyn ${pascalModule}Repository>,
}

impl ${pascalModule}Service {
    pub fn new(repository: Arc<dyn ${pascalModule}Repository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, req: Create${pascalSingular}Request) -> Result<${pascalSingular}Response, String> {
        let item = ${pascalSingular}::new(None, req.name);
        let created = self.repository.create(item).await?;
        Ok(${pascalSingular}Response::from(created))
    }

    pub async fn list(&self) -> Result<Vec<${pascalSingular}Response>, String> {
        let items = self.repository.list().await?;
        Ok(items.into_iter().map(${pascalSingular}Response::from).collect())
    }
}`,

    'controller.rs': `use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode};
use crate::state::AppState;
use super::models::{Create${pascalSingular}Request, ${pascalSingular}Response};

pub async fn create_${snakeSingular}_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Create${pascalSingular}Request>,
) -> Result<(StatusCode, Json<${pascalSingular}Response>), (StatusCode, String)> {
    state
        .${snakeModule}_service
        .create(payload)
        .await
        .map(|res| (StatusCode::CREATED, Json(res)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn list_${snakeModule}_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<${pascalSingular}Response>>, (StatusCode, String)> {
    state
        .${snakeModule}_service
        .list()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}`,

    'router.rs': `use std::sync::Arc;
use axum::{Router, routing::post};
use crate::state::AppState;
use super::controller::{create_${snakeSingular}_handler, list_${snakeModule}_handler};

pub struct ${pascalModule}Router;

impl ${pascalModule}Router {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new()
            .route("/${snakeModule}", post(create_${snakeSingular}_handler).get(list_${snakeModule}_handler))
    }
}`
  };

  for (const [filename, content] of Object.entries(files)) {
    fs.writeFileSync(path.join(moduleDir, filename), content);
  }

  console.log('Done!');
  console.log('Remember to:');
  console.log('1. Register the module in apps/api/src/modules/mod.rs');
  console.log('2. Add the service to AppState in apps/api/src/state.rs');
  console.log('3. Initialize the service and merge the router in apps/api/src/main.rs');
}
