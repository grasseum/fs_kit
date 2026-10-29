# fs-kit Node.js addon

This package exposes the shared Rust path matching implementation to Node.js.

## Build

From this directory, install the N-API CLI and build the native addon:

```sh
npm install
npm run build
```

## Usage

```js
const { getValidateDir } = require("./node");

const matches = getValidateDir(["./src"]);
console.log(matches);
```

`getValidateDir` accepts an array of path patterns and returns an array of
objects with `name`, `fullname`, `info`, and `symlink` string fields.
