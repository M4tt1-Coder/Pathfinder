# 003 — Document Test Structure Guidance

**Date**: 2026-10-10
**Tool**: GitHub Copilot
**Model**: AI assistant using Copilot SDK in VS Code
**Iterations**: 1

## Prompt

**2026-10-10 23:08**

Include these test structure guide lines into the ./tests/README.md file: If I
were starting a Rust project today, I'd use this as the default:

- `src/**` — unit tests beside their implementation, with separate child test
  modules for large files.
- `tests/*.rs` — integration tests organized by feature, API area, or
  user-visible behavior.
- `tests/common/` — shared setup and test helpers.
- `tests/fixtures/` — reusable test data when needed.
- Documentation comments — examples that double as executable documentation.
- Dedicated harnesses or directories — only for specialized test types that
  warrant them.

The guiding principle is that unit versus integration describes the testing
boundary, while feature versus regression versus workflow describes the purpose
of the test. Those are different dimensions, and keeping them separate makes a
Rust test suite easier to grow without overengineering it. I will follow these
architecture patterns in the future.
