---
date: 2026-06-08
id: VIEW-005
---

# Logical Domain Model View

## Viewpoint

Logical, Information.

## Representation

The initial domain model should preserve room for future work/edition/file and medium support while still allowing a practical v0.1 implementation. The following conceptual model is proposed:

```text
Work
├── id
├── title / canonical title
└── has many Editions

Edition
├── id
├── work_id
├── title
├── publication metadata
├── medium type, initially ebook
└── has many BookFiles

BookFile
├── id
├── edition_id
├── file_format, initially EPUB
├── checksum
├── asset_id
└── has many FileVersions

Asset
├── id
├── asset_type: epub, cover, author_image
├── storage_key/path
├── checksum
├── size_bytes
├── media_type
└── source metadata

Author
├── id
├── name
├── image_asset_id optional
└── linked to works/editions through roles

Series
├── id
├── title
└── related to editions and other series

SeriesMembership
├── series_id
├── edition_id or work_id
├── position / label
└── relationship metadata

ReadingState
├── user_id
├── work_id or edition_id
├── status: unread, currently_reading, read, abandoned, TBD
├── owned flag
└── current progress fields

ReadingEvent
├── id
├── user_id
├── work_id or edition_id
├── event_type
├── timestamp
├── progress/page data optional
└── notes/source optional

MetadataCandidate
├── id
├── provider_id
├── candidate payload
├── confidence/source metadata
└── review/apply state

Job
├── id
├── job_type
├── status
├── input summary
├── result/error
├── timestamps
└── operation history entries
```

The model separates catalog identity from file assets. This supports future audiobook and multi-medium workflows, but the initial implementation may simplify the model if an ADR explicitly documents the trade-off.

#### More Information
| Relation | Requirement / Decision |
| -------------- | --------------- |
| Implements | [REQ-FUNC-002](./../requirements/functional/REQ-FUNC-002.md), [REQ-FUNC-003](./../requirements/functional/REQ-FUNC-003.md), [REQ-FUNC-004](./../requirements/functional/REQ-FUNC-004.md), [REQ-FUNC-010](./../requirements/functional/REQ-FUNC-010.md), [REQ-FUNC-014](./../requirements/functional/REQ-FUNC-014.md), [REQ-FUNC-015](./../requirements/functional/REQ-FUNC-015.md), [REQ-FUNC-019](./../requirements/functional/REQ-FUNC-019.md), [REQ-FUNC-020](./../requirements/functional/REQ-FUNC-020.md), [REQ-FUNC-021](./../requirements/functional/REQ-FUNC-021.md), [REQ-FUNC-022](./../requirements/functional/REQ-FUNC-022.md), [REQ-FUNC-023](./../requirements/functional/REQ-FUNC-023.md), [REQ-FUNC-024](./../requirements/functional/REQ-FUNC-024.md), [REQ-FUNC-025](./../requirements/functional/REQ-FUNC-025.md),  [REQ-FUNC-026](./../requirements/functional/REQ-FUNC-026.md), [REQ-FUNC-034](./../requirements/functional/REQ-FUNC-034.md), [REQ-FUNC-035](./../requirements/functional/REQ-FUNC-035.md),  [REQ-FUNC-036](./../requirements/functional/REQ-FUNC-036.md), [REQ-COMP-003](./../requirements/compliance/REQ-COMP-003.md)|
| Related decisions | [DEC-005](./../decisions/DEC-005.md), [DEC-009](./../decisions/DEC-009.md), [DEC-010](./../decisions/DEC-010.md)|
| Open issues | final work/edition/file granularity, multi-author attribution rules, page-count rules, reading-speed formula, exact reading statuses, and future medium model.|

