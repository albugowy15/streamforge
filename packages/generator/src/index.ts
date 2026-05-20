import { Command } from "@effect/cli";
import { NodeContext, NodeRuntime } from "@effect/platform-node";
import { Effect } from "effect";
import { apiCommand } from "./scopes/api/index";

const rootCommand = Command.make("streamforge-gen").pipe(
  Command.withSubcommands([apiCommand])
);

const program = Command.run(rootCommand, {
  name: "Streamforge Generator",
  version: "0.1.0",
});

Effect.suspend(() => program(process.argv)).pipe(
  Effect.provide(NodeContext.layer),
  NodeRuntime.runMain
);
