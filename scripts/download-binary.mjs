#!/usr/bin/env node
import { chmodSync, createWriteStream, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { pipeline } from 'node:stream/promises';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const binariesDir = join(__dirname, '..', 'binaries');

function getBinaryInfo() {
  const platform = process.platform;
  const arch = process.arch;
  const version = process.env.npm_package_version || '0.1.0';

  let target;
  let filename;

  if (platform === 'win32') {
    target = 'x86_64-pc-windows-gnu';
    filename = `edge-v${version}-${target}.exe`;
  } else if (platform === 'darwin') {
    target = arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    filename = `edge-v${version}-${target}`;
  } else if (platform === 'linux') {
    target = arch === 'arm64' ? 'aarch64-unknown-linux-musl' : 'x86_64-unknown-linux-musl';
    filename = `edge-v${version}-${target}`;
  } else {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  return { target, filename };
}

async function downloadBinary() {
  const { filename } = getBinaryInfo();
  const version = process.env.npm_package_version || '0.1.0';
  const url = `https://github.com/edgetrade/mcp/releases/download/v${version}/${filename}`;

  console.log(`Downloading edge binary for ${process.platform}-${process.arch}...`);
  console.log(`URL: ${url}`);

  mkdirSync(binariesDir, { recursive: true });

  const outputPath = join(binariesDir, filename);

  const res = await fetch(url, { redirect: 'follow' });
  if (!res.ok) {
    throw new Error(`Download failed: ${res.status} ${res.statusText}`);
  }

  const fileStream = createWriteStream(outputPath);
  await pipeline(res.body, fileStream);

  if (process.platform !== 'win32') {
    chmodSync(outputPath, 0o755);
  }

  console.log('✓ Binary downloaded successfully');
}

downloadBinary().catch((error) => {
  console.error('Failed to download edge binary:', error.message);
  console.error('\nYou can manually install using:');
  console.error('  cargo install edge-trade');
  process.exit(1);
});
