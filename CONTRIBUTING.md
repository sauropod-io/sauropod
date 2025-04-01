# Contributing

## Dependencies for development

- All the [dependencies listed in README.md](README.md#dependencies)
- [pre-commit](https://pre-commit.com/)

## Set up

```bash
# Install the pre-commit hooks
pre-commit install --install-hooks
```

## Running linters

```bash
make lint
# Alternatively run `pre-commit run` or `pre-commit run --all-files`
```

## Running the server for development

```bash
RUST_LOG=debug cargo run -p sauropod-server
```

## Developing the UI

If you're making changes to the UI, run:

```bash
npm run dev
```
