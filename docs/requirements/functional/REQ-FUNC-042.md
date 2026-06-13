---
status: deferred
date: 2026-06-13
---

# REQ-FUNC-042 — Automatic import folder

## Statement
On the desktop client, the user shall be able to specify a local directory that will then be used to auto-import books.

## Rationale
Adding a book to the library should be as frictionless as possible. Automatically importing from a designated directory helps.

## Acceptance Criteria
- The user specifies (at least) a directory
- The system watches that directory and imports files with matching file types
- The user may change the directory at a later point again

## Verification Method
Demonstration

## More Information
Inspired by calibre, which has the same functionality.
