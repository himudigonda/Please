import { mkdir, cp } from 'node:fs/promises';

await mkdir('dist', { recursive: true });
await cp('src', 'dist/src', { recursive: true });
console.log('build complete');
