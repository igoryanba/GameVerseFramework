import { execFileSync } from 'node:child_process';
import assert from 'node:assert/strict';
const metadata = JSON.parse(execFileSync('cargo',['metadata','--locked','--format-version','1'],{encoding:'utf8',maxBuffer:16*1024*1024}));
const nodes = new Map(metadata.resolve.nodes.map(n=>[n.id,n]));
const packages = new Map(metadata.packages.map(p=>[p.id,p]));
for(const name of ['gameverse-protocol','gameverse-runtime','gameverse-transport','gameverse-server','gameverse-client']) {
  const root = metadata.packages.find(p=>p.name===name);
  assert(root,`Missing ${name}`);
  const seen = new Set();
  function visit(id) {
    if(seen.has(id)) return; seen.add(id);
    const dependency = packages.get(id);
    assert(!['gameverse-core','gameverse-compat-fivem'].includes(dependency.name),`${name} depends on ${dependency.name}`);
    for(const d of nodes.get(id).deps) {
      if(d.dep_kinds.some(k=>k.kind!=='dev')) visit(d.pkg);
    }
  }
  visit(root.id);
}
console.log('M0 production dependency boundary passed');
