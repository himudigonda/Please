import http from 'node:http';

export function createServer() {
  return http.createServer((_, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ status: 'ok', service: 'node-web' }));
  });
}

if (process.env.RUN_SERVER === '1') {
  const server = createServer();
  server.listen(8092, () => {
    console.log('listening on :8092');
  });
}
