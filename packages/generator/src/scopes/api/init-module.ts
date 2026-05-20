import { FileSystem } from "@effect/platform/FileSystem";
import { Path } from "@effect/platform/Path";
import { Terminal } from "@effect/platform/Terminal";
import { Effect } from "effect";
import * as Templates from "./templates";

export const initModule = (moduleName: string) =>
  Effect.gen(function* () {
    const fs = yield* FileSystem;
    const path = yield* Path;
    const terminal = yield* Terminal;

    const toSnakeCase = (str: string) => str.toLowerCase();
    const toPascalCase = (str: string) =>
      str.replace(/(^|_)([a-z])/g, (_, __, c) => c.toUpperCase());
    const toSingular = (str: string) => str.replace(/s$/, "");

    const snakeModule = toSnakeCase(moduleName);
    const snakeSingular = toSingular(snakeModule);
    const pascalModule = toPascalCase(snakeModule);
    const pascalSingular = toPascalCase(snakeSingular);

    const vars: Templates.TemplateVars = {
      snakeModule,
      snakeSingular,
      pascalModule,
      pascalSingular,
    };

    const findRoot = (dir: string): Effect.Effect<string, Error, FileSystem> =>
      Effect.gen(function* () {
        const workspacePath = path.join(dir, "pnpm-workspace.yaml");
        const exists = yield* fs.exists(workspacePath);
        if (exists) {
          return dir;
        }
        const parent = path.dirname(dir);
        if (parent === dir) {
          return yield* Effect.fail(
            new Error("Could not find workspace root (pnpm-workspace.yaml)")
          );
        }
        return yield* findRoot(parent);
      });

    const cwd = yield* Effect.sync(() => process.cwd());
    const rootDir = yield* findRoot(cwd);
    const apiSrcDir = path.join(rootDir, "apps", "api", "src");
    const moduleDir = path.join(apiSrcDir, "modules", snakeModule);

    const dirExists = yield* fs.exists(moduleDir);
    if (dirExists) {
      return yield* Effect.fail(
        new Error(`Module directory already exists: ${moduleDir}`)
      );
    }

    yield* terminal.display(
      `Scaffolding module: ${snakeModule} in ${moduleDir}\n`
    );

    yield* fs.makeDirectory(moduleDir, { recursive: true });

    const files = {
      "mod.rs": Templates.modTemplate(vars),
      "models.rs": Templates.modelsTemplate(vars),
      "repository.rs": Templates.repositoryTemplate(vars),
      "service.rs": Templates.serviceTemplate(vars),
      "controller.rs": Templates.controllerTemplate(vars),
      "router.rs": Templates.routerTemplate(vars),
    };

    for (const [filename, content] of Object.entries(files)) {
      yield* fs.writeFileString(path.join(moduleDir, filename), content);
    }

    yield* terminal.display("Done!\n");
    yield* terminal.display("Remember to:\n");
    yield* terminal.display(
      "1. Register the module in apps/api/src/modules/mod.rs\n"
    );
    yield* terminal.display(
      "2. Add the service to AppState in apps/api/src/state.rs\n"
    );
    yield* terminal.display(
      "3. Initialize the service and merge the router in apps/api/src/main.rs\n"
    );
  });
