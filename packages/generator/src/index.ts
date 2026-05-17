import { apiScope } from './scopes/api/index';

const [scope, action, ...args] = process.argv.slice(2);

if (!scope || !action) {
  console.error('Usage: streamforge-gen <scope> <action> [args...]');
  console.error('Available scopes: api');
  process.exit(1);
}

async function main() {
  switch (scope) {
    case 'api':
      await apiScope(action, args);
      break;
    default:
      console.error(`Error: Unknown scope '${scope}'`);
      process.exit(1);
  }
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
