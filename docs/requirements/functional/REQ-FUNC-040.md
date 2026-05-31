---
status: draft
date: 2025-05-31
---
# REQ-FUNC-040 — Display book table view

## Statement
The system shall display a table-based overview of books. 

## Rationale
This is a Calibre feature that is helpful when trying to sort or filter books by metadata. 

## Acceptance Criteria
- The system renders a table overview of all books in a scrollable table. 
- Each row in the table displays default metadata columns, including title, author(s), series information, publication date, addition date and number of pages.
- Selecting a row opens a detailed view including the book's cover image and more metadata information.
- The user can toggle different metadata columns from being visible.
- The user can sort the table based on each individual column in ascending or descending order by clicking on the column's header.

## Verification Method
Demonstration

## More Information
Alternative view to [REQ-FUNC-011](./REQ-FUNC-011.md). The two views should complement each other. 
