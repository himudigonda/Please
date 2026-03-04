import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from './server.js';

test('server responds with ok', async () => {
  const server = createServer();
  await new Promise((resolve) => server.listen(0, resolve));
  const { port } = server.address();
  const res = await fetch(`http://127.0.0.1:${port}`);
  const json = await res.json();
  assert.equal(json.status, 'ok');
  server.close();
});
