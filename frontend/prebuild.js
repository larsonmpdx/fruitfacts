// clone some files from the plant database into our /public/ folder so that they can be served

const path = require('node:path');
const fs = require('fs');

const SOURCE_DIR = '../plant_database/references/';
const DEST_DIR = './public/data/';

const filter = (src) => {
  const ext = path.extname(src);
  return fs.statSync(src).isDirectory() || ext === '.jpg' || ext === '.json5';
};

console.log(`updating ${DEST_DIR}`);
fs.rmSync(DEST_DIR, { force: true, recursive: true });
fs.cpSync(SOURCE_DIR, DEST_DIR, { filter, recursive: true });
console.log(`finished updating ${DEST_DIR}`);
