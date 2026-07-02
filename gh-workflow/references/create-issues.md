# create-issues (subskill of gh-workflow)

Load this when the user wants to create/open an issue, ticket, bug report, decision, or spike. Creates structured, actionable issues for the project tracker — each self-contained enough that anyone can act on it without digging through chat history.

## Template

Every issue must follow this exact structure:

```markdown
## Context

Background information: what system, feature, or workflow is involved. Describe the current state — what exists today, what the relevant architecture or constraints are, and any recent changes that led here. A reader unfamiliar with this area should understand the landscape after reading this section.

## Problem

What is wrong, missing, or not working. Be specific:
- Current behavior vs. expected behavior.
- Why it matters (user impact, performance, correctness, maintainability).
- Any constraints or edge cases that make this non-trivial.
- If a bug: steps to reproduce, observed output, expected output.

## What to decide/do

A clear, scoped list of what needs to happen:
- **Decision items** (if unresolved): what options are on the table, their trade-offs, and who needs to weigh in.
- **Action items** (if decided): concrete steps to implement, fix, or investigate.
- Acceptance criteria — how we know this is done.
- If a spike/research ticket: what questions must be answered and what constitutes a sufficient answer.

## References

- Links to relevant code, files, or lines.
- Links to related issues, PRs, or design documents.
- Links to external docs, specs, or prior art.
- Any Slack threads, screenshots, or logs that provide additional context.
```

## Formatting rules

- Use proper markdown headings (`##`) — never raw `\n` escape sequences.
- Each section is a `##`-level heading with a descriptive name.
- Use bullet lists for clarity, not dense paragraphs.
- Links must be concrete (file paths, line numbers, issue URLs), not vague "see the code."
- Acceptance criteria use `- [ ]` checklist items under "What to decide/do."
- Keep the issue self-contained — a new team member should understand it without reading Slack history.

## Example: Bug report

```markdown
## Context

The order_deliveries list page uses a `TableCardComponent` for bulk actions. The component has a built-in delete button that always renders when any row is selected, even if the consumer doesn't support bulk delete.

## Problem

When the picking batches list page adopted `TableCardComponent` for bulk print-only workflows, the delete button still appears. Clicking it triggers a 404 because no `bulk_delete_url` is configured. Users see a broken button, and the UI implies functionality that doesn't exist.

Steps to reproduce:
1. Go to picking batches list.
2. Select one or more rows via checkbox.
3. Observe the "Delete selected" button in the bulk action bar.
4. Click it — 404.

## What to decide/do

- [ ] Add a `bulk_delete_url` prop to `TableCardComponent`.
- [ ] Only render the delete button when `bulk_delete_url` is present.
- [ ] Verify existing delete consumers (order_deliveries, shipments) still show the delete button.
- [ ] Add a component test asserting the button is absent when the prop is missing.

## References

- `app/components/table_card_component.rb` — the bulk action bar rendering
- `app/components/table_card_component.html.erb` — template where delete button is unconditionally rendered
- `app/views/picking_batches/index.html.erb` — consumer that shouldn't show delete
- Related PR: https://github.com/org/repo/pull/1234
```

## Example: Decision / spike ticket

```markdown
## Context

We need to pick a caching strategy for the catalog API. Traffic patterns show 80% of requests hit the same 200 SKUs during business hours. Current DB query time per request is ~40ms under load.

## Problem

Response times degrade during peak hours (200ms+ p99). We need to decide between Redis caching, CDN edge caching, or in-memory application cache before implementation begins. Each option has different trade-offs for invalidation, consistency, and operational complexity.

## What to decide/do

Options:
1. **Redis** — centralized, easy invalidation, adds network hop, extra infra.
2. **CDN edge cache** — fastest reads, complex invalidation, good for public data.
3. **In-memory** — zero infra, lost on restart, inconsistent across instances.

Decision needed by: Friday EOD.
Stakeholders: backend lead, infra team.

Outcome: pick one strategy and write a one-page ADR documenting the rationale.

## References

- Current catalog endpoint: `internal/api/handler/catalog.go`
- Traffic analysis: https://datadog.link/dashboard/catalog-p99
- Redis proposal doc: https://docs.internal/caching-proposal
```

## Workflow

### 1. Gather context

Work from the current conversation. If the user passed an issue reference, fetch its full body and comments (see the `fetch-issue` subskill). If the user described a bug or task, use that as the source material.

### 2. Explore the codebase (if needed)

If file paths, line numbers, or specific code references would strengthen the issue, locate them. Concrete references make issues self-contained.

### 3. Draft the issue

Fill in each section of the template. Keep the language precise and neutral. Avoid editorializing — state facts, not opinions.

### 4. Review with the user

Present the drafted issue in the chat. Ask:

- Does this capture the full picture?
- Are the acceptance criteria concrete enough?
- Are references complete?

Iterate until the user approves.

### 5. Publish

Create the issue on the project tracker. Report the issue URL to the user.

```bash
gh issue create --title "<title>" --body "<body>"
```

For GitHub projects, add labels and assignees as needed:

```bash
gh issue create --title "<title>" --body "<body>" --label "<label>" --assignee "<username>"
```
