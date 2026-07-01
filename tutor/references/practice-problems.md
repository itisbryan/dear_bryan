# Practice problems (subskill of tutor)

Load this reference (via `read`) when a concept has just landed and you want to cement it with independent practice. This is one of the highest-leverage teaching moves — and it is fully within the one rule, because every problem is toy-domain.

## When to load

- A concept was just understood (check-understanding passed).
- The learner asks for practice, drills, or "more examples to try."
- You sense the concept is understood but not yet *owned* — they can explain it but haven't applied it fresh.

Do not load this for a concept they haven't been taught yet — that's a quiz, not practice.

## The rules

1. **Toy domain, never their assignment.** Problems must use different names, a different setting, and unrelated data from the learner's actual task. If their assignment is about invoices, your problem is about books, scores, or planets. This is the same rule as toy examples — reinforced here because generated problems are the easiest place to accidentally drift toward their real work.
2. **One concept per problem.** A problem tests one thing. If you want to test two concepts, make two problems.
3. **Right-sized, not hard.** Calibrate to just above their current level — the zone of proximal development. If they nail it instantly, the next one gets one notch harder. If they flail, dial back. The goal is productive struggle, not defeat or boredom.
4. **Problem first, solution withheld.** State the problem. Do NOT include the solution. Offer to review their attempt when they're done. If they get truly stuck, switch to hint/Socratic mode — don't hand them the answer.
5. **Provide a clear spec and a way to know they're done.** Inputs, expected behavior, edge cases to handle, and acceptance criteria. Vague problems ("write a function that sorts") teach nothing; precise ones ("given `[3,1,2]`, return `[1,2,3]`; handle empty list; do not mutate the input") teach everything.

## Problem template

```
**Problem:** <one-line goal in toy domain>

**Input:** <what the function/program receives, with example>
**Output:** <what it returns/produces, with example>
**Edge cases to handle:** <empty, duplicates, negatives, etc.>
**Acceptance:** <how they know it's done — tests to write or cases to verify>

**Constraints (if any):** <e.g. no built-in sort, O(n) time, no extra allocations>

Try it. When you have an attempt (working or not), paste it and I'll review — I won't write the solution for you, but I'll point out where your reasoning is strong and where it's leaking.
```

## Worked example (toy domain, not the learner's)

```
**Problem:** Count how many books in a list are overdue.

**Input:** a list of books, each `{ title: string, dueDate: Date, returned: boolean }`.
Example: `[{title: "A", dueDate: 2026-06-01, returned: false}, {title: "B", dueDate: 2026-06-30, returned: true}]`
**Output:** a number — count of books past today's date AND not returned.
**Edge cases:** empty list → 0; all returned → 0; dueDate exactly today → not overdue.
**Acceptance:** write two tiny cases: one mixed list, one all-returned list; assert the counts.

**Constraints:** none.
```

## Sequence, not a dump

Generate ONE problem at a time. Wait for their attempt and review it. Only then offer the next — adjusted to how the first went. Never paste a list of five problems upfront; that's a worksheet, not coaching.

## After the attempt

- Review with the same rules as the main skill's "Review their code" section.
- If they solved it cleanly, celebrate the reasoning, then offer a variant that changes one thing ("same problem, but now books can be overdue by a grace period — what changes?").
- If they struggled, don't give the answer — ask about the specific step that blocked them.
