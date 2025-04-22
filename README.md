# Tantivy Node.js binding

uses napi-rs

## build

```bash
yarn build
```

## Change of API

this implementation tries to wrap tantivy without API changes.
But since JS is not Rust, some minor adaptations have been made.

| JS signature                                  | Rust signature |
| TopDocs::search() -> Result<Vec<TopDocFruit>> | s
