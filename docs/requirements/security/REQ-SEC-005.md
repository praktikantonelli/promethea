---
status: proposed
date: 2026-05-26
---
# REQ-SEC-005 — Support secure deployment over HTTPS

## Statement 
The system shall support deployment behind HTTPS for all remote browser and API traffic.

## Rationale 
Remote access over the internet should protect credentials and personal reading data in transit.

## Acceptance Criteria
- Deployment documentation includes an HTTPS-capable reverse-proxy or equivalent configuration pattern.
- The application does not require mixed-content HTTP calls from an HTTPS frontend.
- Cookie or token settings are compatible with HTTPS deployment.

## Verification Method 
Inspection

## More Information 
TLS termination may be handled by a reverse proxy rather than the application process. This requirement may be too strict because the project is supposed to be self-hosted and only exposed locally; whether the user exposes it to the web is their choice. 
