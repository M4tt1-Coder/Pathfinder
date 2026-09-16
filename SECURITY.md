# Security Policy

## About This Policy

PathFinder is a Rust library and command-line application for shortest-path
algorithms and graph processing.

This policy explains how to privately report security vulnerabilities in
PathFinder and how security reports are handled.

## Supported Versions

PathFinder currently maintains a single active development line.

| Version / Branch | Security Support           |
| ---------------- | -------------------------- |
| `main`           | ✅ Supported               |
| Latest release   | ✅ Supported               |
| Older releases   | ❌ Not actively maintained |

Security fixes are developed against `main` and, where applicable, included in
the next release. Older releases are not guaranteed to receive backported
security fixes.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues,
pull requests, or discussions.**

Security vulnerabilities should be reported privately through the repository's
**GitHub Security Advisory / private vulnerability reporting** feature:

https://github.com/M4tt1-Coder/Pathfinder/security/advisories/new

When submitting a report, please provide as much relevant information as
possible, including:

- A clear description of the vulnerability
- The affected component or functionality
- The affected version, commit, or branch
- Steps to reproduce the issue
- A minimal proof of concept or malicious input, if available
- The expected and actual behavior
- The potential security impact
- Relevant operating-system, Rust, or runtime information
- Any suggested mitigation or fix, if known

For PathFinder in particular, security reports involving the following areas
are relevant:

- Malicious or malformed graph-file input
- Graph parsing and input validation
- CLI argument parsing and validation
- Unexpected crashes, panics, or resource exhaustion caused by untrusted input
- Incorrect filesystem access caused by untrusted input
- Memory-safety or undefined-behavior issues
- Dependency vulnerabilities that affect PathFinder
- Vulnerabilities that could allow unintended code execution or privilege
  escalation

Please do not include passwords, API keys, private data, or other sensitive
information in a report unless it is necessary to demonstrate the issue.

If private vulnerability reporting is temporarily unavailable, open a public
issue containing **no vulnerability details** and ask for a private reporting
channel instead.

## Response and Disclosure

Security reports will be reviewed privately.

The maintainers will acknowledge and investigate reports when reasonably able.
If a vulnerability is confirmed, the maintainers will work on an appropriate
fix or mitigation and determine the affected versions.

Public disclosure should be coordinated with the maintainers and, where
possible, should take place only after a fix or effective mitigation is
available.

When appropriate, a confirmed vulnerability may be documented through a GitHub
Security Advisory so that affected users can understand the impact and update
to a fixed version.

## Scope

This policy covers the PathFinder source code, its CLI, its graph-input
processing, its library APIs, and dependencies used by the project.

PathFinder is currently a local library and command-line application and does
not operate a network service. Vulnerabilities in downstream applications that
embed PathFinder are outside the project's direct scope unless the underlying
issue originates in PathFinder itself.

For non-security bugs, incorrect algorithm results without a security impact,
feature requests, and general development issues, please use the project's
normal GitHub issue tracker instead.
