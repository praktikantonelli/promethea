---
applyTo: "**"
excludeAgent: "cloud-agent"
---

# Code Review Guidelines

## Workflow

1. *Load Context*
  - Read the `README.md` of the repository as well as that of any subdirectory, and any possible documentation files inside `/docs` (if existent) to understand the project's goals, tech stack, tools etc, testing approach etc.

2. *Risk Evaluation*
  - Categorise findings into potential bugs, security concerns and improvement suggestions.

3. *Documentation Alignment*
  - Flag missing documentation updates as a review finding.

4. *Testing Gaps*
  - Verify whether new or modified behaviour is covered by automated tests.
  - Highlight absent tests or risky areas needing manual verification.

5. Potential Errors
  - Only state that a specific part of the code fails to build if you're 100% sure. Take existing CI/CD workflows as indicators of whether your claim could be true. 

## Priorities

Review changes primarily for:

1. Correctness
2. Security
3. Maintainability
4. Reliability
5. Performance
6. Test coverage

Do not report purely stylistic issues that are already enforced by automated
formatters or linters.

## Correctness

- Look for incorrect assumptions, edge cases, race conditions, and unexpected
  control-flow behavior.
- Check error paths, not only the happy path.
- Check whether behavior changes are intentional and adequately tested.
- Flag code where invalid or unexpected input could produce incorrect state.

## Security

Treat all external input as untrusted.

Pay particular attention to:

- authentication and authorization
- injection vulnerabilities
- XSS
- unsafe URL handling
- filesystem/path traversal
- command execution
- accidental exposure of secrets or sensitive information
- insecure storage of credentials or tokens
- missing validation at trust boundaries

Do not assume that validation in a client application provides a security
boundary.

## Maintainability

- Prefer simple, explicit implementations over unnecessarily clever ones.
- Flag duplicated business logic when it could cause behavior to diverge.
- Flag abstractions that add complexity without a clear benefit.
- Check whether responsibilities are clearly separated.
- Prefer code whose intent can be understood without extensive comments.

Do not suggest speculative refactorings unrelated to the pull request.

## Tests

Request tests when a change introduces or modifies non-trivial behavior.

Tests should cover:

- the primary behavior
- important edge cases
- error cases
- regressions fixed by the pull request

## Review Findings Output Template

## Potential Bug Reports

- [Severity] `path/to/file`: Short headline describing the issue.
    - Details: Explain why it is likely a bug, referencing diff context.
    - Recommendation: Suggested fix or mitigation.

## Security Considerations

- [Severity] `path/to/file`: Security or privacy risk summary.
    - Details: Reference specific lines and affected data flows.
    - Recommendation: Steps to address or verify.

## Quality Suggestions

- [Severity] `path/to/file`: Improvement opportunity (performance, readability,
  DX).
    - Details: Observation and rationale.
    - Recommendation: Optional refinements or follow-up tasks.

## Testing & Documentation Gaps

- [Severity] Area: Missing or insufficient automated tests updates.
    - Details: What is absent and why it matters.
    - Recommendation: Specific test cases or documentation updates to add.

## Additional Guidance

- Reference documentation where relevant.
- Highlight any dependency on background jobs, cron tasks, or external services
  that might mask live issues.

## Command Usage Notes

- Provide concise, actionable findings; avoid verbose summaries of obvious diff
  content.
- If no issues are discovered, explicitly state that no potential bugs, 
  security concerns, or testing gaps were identified.
