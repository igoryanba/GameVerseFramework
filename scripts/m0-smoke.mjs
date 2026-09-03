// Separate-process acceptance run. No game installation or external server required.
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createWriteStream } from 'node:fs';
import { resolve, join } from 'node:path';
import { once } from 'node:events';
import { tmpdir } from 'node:os';
import assert from 'node:assert/strict';

const arg = (name, fallback) => {
  const index = process.argv.indexOf(name);
  return index < 0 ? fallback : process.argv[index + 1];
};
const seconds = Number(arg('--seconds', '600'));
assert(Number.isInteger(seconds) && seconds >= 6, '--seconds must be an integer >= 6');
const output = resolve(arg('--output', join(tmpdir(), `gameverse-m0-${Date.now()}`)));
await mkdir(output, { recursive: true });
const bin = resolve(arg('--bin', join(process.env.CARGO_TARGET_DIR ?? 'target', 'debug')));
const exe = name => join(bin, name + (process.platform === 'win32' ? '.exe' : ''));
const cert = join(output, 'localhost.der');
const key = join(output, 'localhost.key');
const children = new Set();
function start(name, args, label) {
  const child = spawn(exe(name), args, { windowsHide: true, stdio: ['ignore','pipe','pipe'] });
  children.add(child);
  const stdout = createWriteStream(join(output, `${label}.stdout.log`));
  const stderr = createWriteStream(join(output, `${label}.stderr.log`));
  child.stdout.pipe(stdout); child.stderr.pipe(stderr);
  const done = new Promise((resolve, reject) => {
    child.once('error', reject);
    child.once('close', async code => {
      children.delete(child);
      // Pipe finish completes before reports/logs are consumed below.
      await Promise.all([stdout.writableFinished ? null : once(stdout, 'finish'), stderr.writableFinished ? null : once(stderr, 'finish')]);
      code === 0 ? resolve() : reject(new Error(`${label} exited ${code}; see ${output}`));
    });
  });
  done.catch(() => {}); // Observed by the orchestration below; no unhandled rejection race.
  return { child, done };
}
const watchdog = setTimeout(() => {
  for (const child of children) child.kill();
}, (seconds + 30) * 1000);
try {
  await start('gameverse-server', ['--init-identity','--cert',cert,'--key',key], 'identity').done;
  const server = start('gameverse-server', ['--bind','127.0.0.1:0','--cert',cert,'--key',key,'--duration',String(seconds+3)], 'server');
  const address = await new Promise((resolve,reject) => {
    let text = '';
    const timer = setTimeout(() => reject(new Error('server readiness timeout')), 5000);
    server.child.stdout.on('data', chunk => {
      text += chunk;
      const end = text.indexOf('\n');
      if (end >= 0) {
        try { const value = JSON.parse(text.slice(0,end)); if(value.event === 'ready') { clearTimeout(timer); resolve(value.address); } }
        catch(error) { clearTimeout(timer); reject(error); }
      }
    });
    server.done.catch(error => {clearTimeout(timer); reject(error);});
  });
  const common = ['--server',address,'--cert',cert,'--duration',String(seconds),'--move-seconds','2'];
  const a = start('gameverse-client', [...common,'--dx','1','--dy','0','--reconnect-after',String(Math.floor(seconds/2)),'--report',join(output,'client-a.json')], 'client-a');
  const b = start('gameverse-client', [...common,'--dx','0','--dy','1','--report',join(output,'client-b.json')], 'client-b');
  await Promise.all([a.done,b.done,server.done]);
  const first = JSON.parse(await readFile(join(output,'client-a.json'),'utf8'));
  const second = JSON.parse(await readFile(join(output,'client-b.json'),'utf8'));
  const lines = (await readFile(join(output,'server.stdout.log'),'utf8')).trim().split(/\r?\n/).map(x=>JSON.parse(x));
  const shutdown = lines.at(-1);
  assert(first.clean_shutdown && second.clean_shutdown);
  assert.equal(first.sessions.length,2);
  assert.notEqual(first.sessions[0],first.sessions[1]);
  assert.equal(second.sessions.length,1);
  assert.equal(first.convergence_state.entities.length,2);
  assert.deepEqual(first.convergence_state.entities,second.convergence_state.entities);
  assert(first.snapshots >= seconds * 5 && second.snapshots >= seconds * 5);
  assert.equal(shutdown.event,'shutdown');
  assert.equal(shutdown.players,0);
  assert.equal(shutdown.accepted_sessions,3);
  assert.equal(shutdown.disconnects,3);
  assert(shutdown.max_input_depth <= 128);
  const report = { passed:true, seconds, clients:[first,second], server:shutdown, node:process.version, platform:process.platform };
  await writeFile(join(output,'acceptance.json'),JSON.stringify(report,null,2));
  console.log(JSON.stringify({passed:true, output, seconds, server:shutdown}));
} finally {
  clearTimeout(watchdog);
  for (const child of children) child.kill();
}
