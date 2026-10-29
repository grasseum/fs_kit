const assert = require("node:assert/strict");
const { mkdtempSync, mkdirSync, rmSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { join } = require("node:path");
const { test } = require("node:test");

const { glob } = require("..");

test("glob returns an empty array for empty input", () => {
  assert.deepEqual(glob([]), []);
});

test("glob returns matching subfolders", () => {
  const root = mkdtempSync(join(tmpdir(), "fs-kit-node-test-"));
  const originalDirectory = process.cwd();
  mkdirSync(join(root, "alpha"));
  mkdirSync(join(root, "beta"));

  try {
    process.chdir(root);
    const result = glob(["./**"]);

    assert.deepEqual(
      result.map((entry) => entry.name).sort(),
      ["alpha", "beta"],
    );
    assert.ok(result.every((entry) => entry.info === "folder"));
  } finally {
    process.chdir(originalDirectory);
    rmSync(root, { recursive: true, force: true });
  }
});
