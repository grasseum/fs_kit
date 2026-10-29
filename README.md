# fs-kit

Rust filesystem path matching library with Python and Node.js bindings.

## Python

Install the Python package from this directory:

```sh
python -m pip install .
```

Run its tests with:

```sh
python -m pip install -e '.[test]'
python -m pytest
```

## Node.js

Build and test the native addon from `node/`:

```sh
npm install
npm test
npm run build
```

`npm publish` builds the native addon for the publishing platform before packaging.
