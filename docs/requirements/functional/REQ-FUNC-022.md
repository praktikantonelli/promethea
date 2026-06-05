---
status: proposed
date: 2026-05-26
---
# REQ-FUNC-022 — Track reading progress events

## Statement 
The system shall allow recording reading progress events containing at least date/time and one progress measure such as page, percentage, or location.

## Rationale 
Reading progress supports reading-speed analytics and history beyond a single status field.

## Acceptance Criteria
- A user can create a progress event for a book.
- Progress events are associated with the correct book or edition.
- Invalid progress values are rejected or reported to the user.

## Verification Method 
Test

## More Information 
Which progress measures are mandatory depends on available page/location data and is TBD.
