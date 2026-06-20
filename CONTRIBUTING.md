# Contributing to GatKeeper

Thank you for your interest in contributing to GatKeeper! This document provides guidelines and instructions for contributing.

## 🎯 First Time Contributors

Look for issues labeled **`good first issue`** — they are specifically scoped for newcomers.

## 🚀 Development Setup

### Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs/))
- **Git**
- **Node.js** 22+ (for PDF generation scripts)
- **Ollama** (optional, for local LLM testing)

### Clone & Build

```bash
# Clone the repository
git clone https://github.com/karlalbert152-sys/gatkeeper.git
cd gatkeeper

# Build the project
cargo build

# Run tests
cargo test

# Check for lint issues
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Project Structure

```
gatkeeper/
├── crates/
│   ├── gatkeeper-cli/           # CLI binary (entry point)
│   ├── gatkeeper-core/          # Core types, config, report generation
│   ├── gatkeeper-parser/        # Tree-sitter multi-language parser
│   ├── gatkeeper-agents/        # 6 specialized AI agents
│   ├── gatkeeper-subconscious/  # Subconscious Engine (24/7 simulation)
│   └── gatkeeper-dna/           # DNA Engine (codebase memory)
├── gatkeeper-lenses/            # Intent comprehension
├── migrations/                  # SQLite schema
├── templates/                   # Config templates
└── tests/                       # Integration tests
```

## 📝 Making Changes

### Branch Naming

Use descriptive branch names:
- `feat/add-new-agent` — New feature
- `fix/resolve-sql-injection-false-positive` — Bug fix
- `docs/update-readme` — Documentation
- `refactor/agent-orchestrator` — Refactoring
- `test/add-security-agent-tests` — Tests

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(agents): add XSS detection patterns to SecurityAgent
fix(parser): handle empty source files gracefully
docs: update CONTRIBUTING.md with setup instructions
test(core): add risk score calculation tests
refactor(subconscious): extract scheduler into separate module
```

### Code Style

- **Rust**: Follow `rustfmt` defaults (run `cargo fmt`)
- **No comments** unless explicitly requested
- **Use `thiserror`** for error types
- **Use `tracing`** for logging (not `println!`)
- **Use `async`** with Tokio for I/O-bound operations
- **Keep functions focused** — single responsibility

### Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p gatkeeper-core
cargo test -p gatkeeper-agents

# Run with output
cargo test -- --nocapture
```

When adding new functionality:
1. Add unit tests in the same file (if small) or `tests/` module
2. Add integration tests in `tests/` directory
3. Ensure `cargo clippy` passes

## 🔀 Pull Request Process

1. **Fork** the repository
2. **Create** your branch from `main`
3. **Make** your changes
4. **Run** `cargo fmt` and `cargo clippy`
5. **Run** `cargo test` and ensure all pass
6. **Write** a clear PR description explaining:
   - What the change does
   - Why it's needed
   - How to test it
7. **Request** a review from a maintainer

### PR Title Format

Same as commit messages: `feat(agents): add new detection pattern`

### PR Description Template

```markdown
## What does this PR do?
Brief description of the change.

## Why is this change needed?
Context and motivation.

## How was this tested?
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` is clean
- [ ] Manual testing (if applicable)

## Related Issues
Closes #123
```

## 🐛 Reporting Bugs

Use the **Bug Report** issue template. Include:
- GatKeeper version
- OS and Rust version
- Steps to reproduce
- Expected vs actual behavior
- Logs/output if applicable

## 💡 Suggesting Features

Use the **Feature Request** issue template. Include:
- Clear description of the feature
- Use case / motivation
- Proposed implementation (if you have ideas)

## 🔒 Security Vulnerabilities

**Do NOT open a public issue for security vulnerabilities.**

Instead, email security concerns to: **karlalbert152@gmail.com**

We will respond within 48 hours and work with you to understand and address the issue.

## 📚 Documentation

Documentation improvements are always welcome:
- Fix typos or unclear explanations
- Add examples
- Improve API documentation
- Translate docs (see `README_FR.md` for French)

## 🏷️ Labels

| Label | Description |
|-------|-------------|
| `good first issue` | Perfect for newcomers |
| `help wanted` | Community contributions welcome |
| `bug` | Something isn't working |
| `enhancement` | New feature or improvement |
| `documentation` | Docs improvement |
| `security` | Security-related issue |
| `agent:*` | Specific agent (security, logic, etc.) |
| `engine:*` | Specific engine (subconscious, dna) |

## 📜 License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE) or [Apache License 2.0](LICENSE).

## 🙏 Thank You!

Every contribution matters. Whether it's:
- 🐛 Reporting a bug
- 📝 Improving docs
- 💻 Writing code
- 🧪 Adding tests
- 💡 Suggesting features
- ⭐ Starring the repo

You're helping make software security better for everyone.

---

*"While you slept, I found 3 things you need to know." — GatKeeper*
