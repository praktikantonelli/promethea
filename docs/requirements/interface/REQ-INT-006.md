---
status: planned
date: 2026-05-26
---
# REQ-INT-006 — External image URL download interface

## Statement 
The system shall allow an authenticated user to paste an image URL for an author image and have the system download and store the image as an author asset.

## Rationale 
The draft explicitly asks for author images that can be fetched automatically or by pasted URL.

## Acceptance Criteria
- The UI accepts an image URL on the author edit/detail view.
- The backend downloads only supported image media types and rejects unsupported responses.
- The resulting image is displayed on the author detail page after successful download.
- The original URL is recorded as asset source metadata when available.

## Verification Method 
Test

## More Information 
Allowed image types and maximum download size are TBD and tracked as security/performance details.
