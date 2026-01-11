import os from "os";
import net from "net";
import path from "path";
import { spawn, spawnSync } from "child_process";
import { fileURLToPath } from "url";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const appName = process.platform === "win32" ? "promethea.exe" : "promethea";

async function getFreePort(): Promise<number> {
  return await new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const addr = server.address();
      if (!addr || typeof addr === "string") return reject(new Error("No address"));
      const port = addr.port;
      server.close(() => resolve(port));
    });
  });
}

function waitForPort(host: string, port: number, timeoutMs = 20000) {
  const start = Date.now();
  return new Promise<void>((resolve, reject) => {
    const tryOnce = () => {
      const socket = new net.Socket();
      socket
        .once("error", () => {
          socket.destroy();
          if (Date.now() - start > timeoutMs) {
            reject(new Error(`Timed out waiting for ${host}:${port}`));
          } else {
            setTimeout(tryOnce, 250);
          }
        })
        .connect(port, host, () => {
          socket.end();
          resolve();
        });
    };
    tryOnce();
  });
}

// Track the tauri-driver child process so we can stop it
let tauriDriver: ReturnType<typeof spawn> | undefined;

function closeTauriDriver() {
  if (!tauriDriver) return;
  if (tauriDriver.killed) return;

  try {
    tauriDriver.kill();
  } catch {

  } finally {
    tauriDriver = undefined;
  }
}


export const config: WebdriverIO.Config = {
  hostname: "127.0.0.1",
  port: 0 as any,

  specs: ["./tests/specs/**/*.e2e.ts"],
  maxInstances: 1,

  // This is the key part: tell tauri-driver which binary to launch
  capabilities: [
    {
      "tauri:options": {
        application: path.resolve(
          __dirname,
          `../backend/target/debug/${appName}`
        ),
      },
    } as unknown as WebdriverIO.Capabilities,
  ],

  reporters: ["spec"],
  framework: "mocha",
  mochaOpts: {
    ui: "bdd",
    timeout: 60_000,
  },

  //
  // Build the app before starting the webdriver session.
  // Tauri docs do this in onPrepare to guarantee the binary exists. :contentReference[oaicite:6]{index=6}
  //
  onPrepare: () => {
    // Run from repo root (one level up)
    const repoRoot = path.resolve(__dirname, "..");

    // This builds a debug binary without bundling installers.
    // Equivalent idea to the docs' example build step, but for pnpm. :contentReference[oaicite:7]{index=7}
    const result = spawnSync(
      "pnpm",
      ["tauri", "build", "--debug", "--no-bundle"],
      {
        cwd: repoRoot,
        stdio: "inherit",
        shell: true,
      }
    );

    if (result.status !== 0) {
      process.exit(result.status ?? 1);
    }
  },

  //
  // Start tauri-driver before WebdriverIO tries to create a session. :contentReference[oaicite:8]{index=8}
  //
  beforeSession: async (config) => {
    const port = await getFreePort();
    (config as any).port = port;
    const tauriDriverPath = path.resolve(os.homedir(), ".cargo", "bin", process.platform === "win32" ? "tauri-driver.exe" : "tauri-driver");

    tauriDriver = spawn(tauriDriverPath, ["--port", String(port)], {
      stdio: "inherit",
      windowsHide: true,
      shell: false,
    });

    tauriDriver.on("error", (error) => {
      console.error("tauri-driver error:", error);
      process.exit(1);
    });

    await waitForPort("127.0.0.1", port);

    tauriDriver.on("exit", (code, signal) => {
      console.error(`tauri-driver exited (code=${code}, signal=${signal})`);
      process.exit(code ?? 1);
    });
  },

  onComplete: () => {
    closeTauriDriver();
  },

};
