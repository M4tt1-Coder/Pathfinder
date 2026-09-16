# Changes Log

This directory contains brief overviews of changes made by AI-assisted work.
Each entry is paired with an AI diary entry and helps readers quickly understand
what changed without reading the full prompt history.

## Structure

Entries are organized by feature branch and use the same numeric prefix as the
matching entry under `diary/`:

```
changes-log/
  development/
    005-create-changes-log-infrastructure.md
  feature/add-retry-logic/
    001-add-retry-logic.md
```

Create one entry for every AI-assisted prompt handled in the repository,
whether or not a commit is created. Use the next three-digit number in the
matching `diary/<branch_name>/` directory.

## Entry Template

```markdown
# NNN — Short Title

**Date**: YYYY-MM-DD
**Tool**: [tool name]

## Overview

[One or two sentences describing what changed.]

## Affected Areas

- [Files, modules, workflows, or documentation changed.]

## Validation

- [Checks or tests run, or `Not run` with a brief reason.]
```

Keep entries concise. Record outcomes rather than copying the full prompt;
the paired diary entry is the source for prompt details.
