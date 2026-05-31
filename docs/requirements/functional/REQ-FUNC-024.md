---
status: planned
date: 2026-05-26
---
# REQ-FUNC-024 — Analyze pages read per period

## Statement 
The system shall provide analytics for number of pages read per selected period when page-count or progress data is available.

## Rationale 
The draft explicitly lists pages read per period as an analytics example.

## Acceptance Criteria
- The system computes page totals from page counts and/or reading progress events according to documented rules.
- The analytics view indicates when page totals are unavailable due to missing page data.
- The user can view at least monthly and yearly page totals.

## Verification Method 
Analysis

## More Information 
Page calculation rules are TBD and should account for books without reliable page counts.
