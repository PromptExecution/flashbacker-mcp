#!/usr/bin/env node
/**
 * Test Poly-Proxy Opinion Enforcement
 * Verifies that flashbacker-mcp enforces codified opinions correctly
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

const tests = [
  {
    name: 'Node.js version check on startup',
    test: async () => {
      // Just start the server and check stderr for version message
      const proc = spawn(process.execPath, ['./flashbacker-mcp']);

      return new Promise((resolve) => {
        let stderr = '';

        proc.stderr.on('data', (data) => {
          stderr += data.toString();
        });

        setTimeout(() => {
          proc.kill();

          const hasVersionCheck = stderr.includes('Node.js') &&
                                  (stderr.includes('✅') || stderr.includes('💡') || stderr.includes('⚠️'));

          if (hasVersionCheck) {
            console.log('✅ PASS: Node.js version check appears on startup');
            console.log(`   Output: ${stderr.split('\n')[1]}`);
            resolve(true);
          } else {
            console.log('❌ FAIL: No Node.js version check found');
            console.log(`   Stderr: ${stderr}`);
            resolve(false);
          }
        }, 1000);
      });
    }
  },
  {
    name: 'Init without --mcp flag shows suggestion',
    test: async () => {
      const request = {
        jsonrpc: '2.0',
        id: 1,
        method: 'tools/call',
        params: {
          name: 'flashback_init',
          arguments: { mcp: false }
        }
      };

      const proc = spawn(process.execPath, ['./flashbacker-mcp'], {
        cwd: '/tmp/test-flashback'
      });

      return new Promise((resolve) => {
        let stdout = '';

        proc.stdout.on('data', (data) => {
          stdout += data.toString();
        });

        proc.stdin.write(JSON.stringify(request) + '\n');

        setTimeout(() => {
          proc.kill();

          const hasSuggestion = stdout.includes('💡') && stdout.includes('--mcp');

          if (hasSuggestion) {
            console.log('✅ PASS: Suggestion to use --mcp flag shown');
            resolve(true);
          } else {
            console.log('❌ FAIL: No --mcp suggestion found');
            console.log(`   Output: ${stdout}`);
            resolve(false);
          }
        }, 2000);
      });
    }
  },
  {
    name: 'Commands before init are blocked',
    test: async () => {
      // Create temp directory without .claude
      const tempDir = '/tmp/test-flashback-uninit-' + Date.now();
      fs.mkdirSync(tempDir, { recursive: true });

      const request = {
        jsonrpc: '2.0',
        id: 1,
        method: 'tools/call',
        params: {
          name: 'flashback_persona',
          arguments: { persona: 'architect', request: 'test' }
        }
      };

      const proc = spawn(process.execPath, ['./flashbacker-mcp'], {
        cwd: tempDir,
        env: { ...process.env, CWD: tempDir }
      });

      return new Promise((resolve) => {
        let stdout = '';

        let requestSent = false;
        proc.stdout.on('data', (data) => {
          stdout += data.toString();
          // Wait for server to signal readiness before sending request
          if (!requestSent && stdout.includes('ready')) { // Replace 'ready' with actual readiness message if needed
            proc.stdin.write(JSON.stringify(request) + '\n');
            requestSent = true;
          }
        });

        // Fallback: if readiness message is not received in time, send anyway after timeout
        setTimeout(() => {
          if (!requestSent) {
            proc.stdin.write(JSON.stringify(request) + '\n');
            requestSent = true;
          }
        }, 1000);

        setTimeout(() => {
          proc.kill();
          fs.rmSync(tempDir, { recursive: true, force: true });

          const isBlocked = stdout.includes('🛑') && stdout.includes('not initialized');

          if (isBlocked) {
            console.log('✅ PASS: Command blocked when project not initialized');
            resolve(true);
          } else {
            console.log('❌ FAIL: Command should be blocked but was not');
            console.log(`   Output: ${stdout}`);
            resolve(false);
          }
        }, 2000);
      });
    }
  },
  {
    name: 'Manual save shows warning',
    test: async () => {
      const request = {
        jsonrpc: '2.0',
        id: 1,
        method: 'tools/call',
        params: {
          name: 'flashback_save_session',
          arguments: { context: true }
        }
      };

      const proc = spawn(process.execPath, ['./flashbacker-mcp']);

      return new Promise((resolve) => {
        let stdout = '';
        let ready = false;

        proc.stdout.on('data', (data) => {
          const str = data.toString();
          stdout += str;
          // Wait for a readiness signal before sending the request.
          // Adjust the readiness check as needed for your server.
          if (!ready && (str.includes('Node.js') || str.toLowerCase().includes('listening') || str.toLowerCase().includes('ready'))) {
            ready = true;
            proc.stdin.write(JSON.stringify(request) + '\n');
          }
        });

        // Fallback: If no readiness signal after 1s, send anyway.
        setTimeout(() => {
          if (!ready) {
            proc.stdin.write(JSON.stringify(request) + '\n');
            ready = true;
          }
        }, 1000);

        setTimeout(() => {
          proc.kill();

          const hasWarning = stdout.includes('⚠️') && stdout.includes('automatic');

          if (hasWarning) {
            console.log('✅ PASS: Warning shown for manual saves');
            resolve(true);
          } else {
            console.log('❌ FAIL: No warning for manual saves');
            console.log(`   Output: ${stdout}`);
            resolve(false);
          }
        }, 2000);
      });
    }
  }
];

async function runTests() {
  console.log('\n🧪 Testing Poly-Proxy Opinion Enforcement\n');
  console.log('='.repeat(60));

  let passed = 0;
  let failed = 0;

  for (const test of tests) {
    console.log(`\nTest: ${test.name}`);
    try {
      const result = await test.test();
      if (result) {
        passed++;
      } else {
        failed++;
      }
    } catch (error) {
      console.log(`❌ FAIL: ${error.message}`);
      failed++;
    }
  }

  console.log('\n' + '='.repeat(60));
  console.log(`\n📊 Results: ${passed} passed, ${failed} failed\n`);

  process.exit(failed > 0 ? 1 : 0);
}

if (require.main === module) {
  runTests().catch(console.error);
}
