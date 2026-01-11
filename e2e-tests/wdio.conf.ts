import os from "os";
import path from "path";
import { spawn, spawnSync } from "child_process";
import { fileURLToPath } from "url";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

// Track the tauri-driver child process so we can stop it
let tauriDriver: ReturnType<typeof spawn> | undefined;

function closeTauriDriver() {
  if (tauriDriver && !tauriDriver.killed) {
    tauriDriver.kill();
  }
}

// Ensure cleanup even if tests crash
process.on("exit", closeTauriDriver);
process.on("SIGINT", () => {
  closeTauriDriver();
  process.exit(130);
});
process.on("SIGTERM", () => {
  closeTauriDriver();
  process.exit(143);
});

export const config: WebdriverIO.Config = {
  host: "127.0.0.1",
  port: 4445,

  specs: ["./tests/specs/**/*.e2e.ts"],
  maxInstances: 1,

  // This is the key part: tell tauri-driver which binary to launch
  capabilities: [
    {
      maxInstances: 1,
      "tauri:options": {
        application: path.resolve(
          __dirname,
          "../backend/desktop/target/debug/promethea.exe"
        ),
      },
    },
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
  beforeSession: () => {
    const tauriDriverPath = path.resolve(os.homedir(), ".cargo", "bin", "tauri-driver");

    tauriDriver = spawn(tauriDriverPath, [], {
      stdio: [null, process.stdout, process.stderr],
      shell: false,
    });

    tauriDriver.on("error", (error) => {
      console.error("tauri-driver error:", error);
      process.exit(1);
    });
  },

  afterSession: () => {
    closeTauriDriver();
  },

  //
  // TypeScript support for WDIO runner
  //
  autoCompileOpts: {
    autoCompile: true,
    tsNodeOpts: {
      transpileOnly: true,
      project: path.resolve(__dirname, "tsconfig.json"),
    },
  },
};
