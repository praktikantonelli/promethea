---
date: 2026-06-08
id: VIEW-010
---

# Reading Tracking and Analytics View

## Viewpoint

Logical, Information, Interaction, State Dynamics.

## Representation

Reading tracking is modeled through current reading state plus append-only reading events. Analytics should derive from stored state/events rather than from ad hoc UI-only calculations.

```text
ReadingState
├── user_id
├── work_id or edition_id
├── status: unread | currently_reading | read | abandoned 
├── current_progress_value optional
├── current_progress_unit optional
├── started_at optional
├── finished_at optional
└── updated_at

ReadingEvent
├── id
├── user_id
├── work_id or edition_id
├── event_type: status_change | progress_update | completed | abandoned | TBD
├── old_status optional
├── new_status optional
├── progress_value optional
├── progress_unit optional
├── event_time
└── source: manual | import | future_sync | TBD
```

Status transition baseline:

```text
unread -> currently_reading -> read
unread -> abandoned
currently_reading -> abandoned
abandoned -> currently_reading
read -> currently_reading, for reread
```

Analytics use cases:

```text
Books read per period
  -> count completion events or read-state transitions according to reread policy

Pages read per period
  -> aggregate page/progress events using documented page-count rules

Books by author
  -> group completed/read books by author using documented multi-author attribution policy

Reading speed by author
  -> compute only when sufficient started, finished/progress, page-count, and attribution data exists
```

The UI must clearly indicate when analytics cannot be computed because required page counts, timestamps, or attribution rules are missing.

#### More Information

| Relation | Requirement / Decision|
| -------------- | --------------- |
| Implements | [REQ-FUNC-019](./../requirements/functional/REQ-FUNC-019.md), [REQ-FUNC-020](./../requirements/functional/REQ-FUNC-020.md), [REQ-FUNC-021](./../requirements/functional/REQ-FUNC-021.md), [REQ-FUNC-022](./../requirements/functional/REQ-FUNC-022.md), [REQ-FUNC-023](./../requirements/functional/REQ-FUNC-023.md), [REQ-FUNC-024](./../requirements/functional/REQ-FUNC-024.md), [REQ-FUNC-025](./../requirements/functional/REQ-FUNC-025.md), [REQ-FUNC-026](./../requirements/functional/REQ-FUNC-026.md), [REQ-FUNC-027](./../requirements/functional/REQ-FUNC-027.md), [REQ-COMP-003](./../requirements/compliance/REQ-COMP-003.md), [REQ-SEC-002](./../requirements/security/REQ-SEC-002.md), [REQ-REL-001](./../requirements/reliability/REQ-REL-001.md)|
| Related decisions |[DEC-005](./../decisions/DEC-005.md), [DEC-010](./../decisions/DEC-010.md) |
| Open issues | exact status vocabulary, reread handling, page-count rules, reading-speed formula, work-vs-edition granularity, and multi-author attribution policy.|

