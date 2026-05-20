import { Command, Args } from "@effect/cli";
import { initModule } from "./init-module";

export const apiCommand = Command.make("api").pipe(
  Command.withSubcommands([
    Command.make("init-module", {
      moduleName: Args.text({ name: "module_name" }),
    }).pipe(Command.withHandler(({ moduleName }) => initModule(moduleName))),
  ])
);
