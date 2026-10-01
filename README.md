# KOS-safety

Repositories: [kos-exec](https://github.com/KATECH-JYHAN/kos-exec) · [kos-comm](https://github.com/KATECH-JYHAN/kos-comm) · [kos-safety](https://github.com/KATECH-JYHAN/kos-safety)

Safety types and policies shared across the KOS platform.

- `AsilLevel` — ISO 26262 ASIL (QM, A–D) and its read/write rules
- `Criticality` — zone criticality classification
- `Priority` — task and message priorities
- `SafetyConfig`, `can_access`, and related checks — safety policy enforcement

Used by [KOS-exec](https://github.com/KATECH-JYHAN/kos-exec).

```bash
cargo test
```

## Author

Jun-young Han, Senior Researcher — KATECH SDV Platform Research Center · jyhan@katech.re.kr

## License

Copyright (c) 2026 Jun-young Han. Licensed under the [Apache License 2.0](LICENSE).
