---
status: planned
date: 2026-05-26
---
# REQ-FUNC-006 — Edit EPUB content

## Statement
The system shall support authenticated editing of EPUB textual content for selected books.

## Rationale
Even revised e-books sometimes contain typos, bad formatting or unwanted adverts. Owning an e-book shall enable a user to modify its content as they please.

## Acceptance Criteria
- A user can open an EPUB content-edit workflow for an imported EPUB.
- The system stores the edited version and overrides the previous one only when saving was successful.
- The user receives validation feedback if the edited EPUB cannot be saved in a usable package structure.

## Verification Method
Demonstration

## More Information
Current assumption is that the web interface should use an existing JS/TS editor, whereas a future desktop application would leverage a locally installed text editor such as VS Code or NeoVim.
