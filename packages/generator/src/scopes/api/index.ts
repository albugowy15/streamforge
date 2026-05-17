import { initModule } from './init-module';

export async function apiScope(action: string, args: string[]) {
  switch (action) {
    case 'init-module':
      await initModule(args);
      break;
    default:
      console.error(`Error: Unknown action '${action}' for scope 'api'`);
      process.exit(1);
  }
}
