---
status: proposed
date: 2026-05-26
---
# REQ-SEC-004 — Protect secrets and provider credentials

## Statement 
The system shall store API keys, session secrets, and provider credentials outside source-controlled code and outside client-delivered frontend bundles.

## Rationale 
External metadata providers and authentication may require secrets that must not be exposed to clients or repositories.

## Acceptance Criteria
- No provider API key is embedded in the compiled frontend bundle.
- Runtime configuration supports providing secrets through environment variables, config files, or secret stores.
- Configuration examples use placeholders rather than real secrets.

## Verification Method 
Inspection

## More Information 
Exact secret-storage mechanism depends on deployment mode.
