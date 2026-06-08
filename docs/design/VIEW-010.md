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
├── owned: boolean
├── status: unread | currently_reading | read | abandoned | TBD
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
Unread -> CurrentlyReading -> Read
Unread -> Abandoned
CurrentlyReading -> Abandoned
Abandoned -> CurrentlyReading
Read -> CurrentlyReading, for reread if supported
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

Implements: [REQ-FUNC-019](./../requirements/functional/REQ-FUNC-019.md) through [REQ-FUNC-027](./../requirements/functional/REQ-FUNC-027.md), [REQ-COMP-003](./../requirements/compliance/REQ-COMP-003.md), [REQ-SEC-002](./../requirements/security/REQ-SEC-002.md), [REQ-REL-001](./../requirements/reliability/REQ-REL-001.md).  
Related decisions: 
Open issues: exact status vocabulary, reread handling, page-count rules, reading-speed formula, work-vs-edition granularity, and multi-author attribution policy.
