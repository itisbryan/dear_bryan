# General study — "what makes this repo good?"

When the question is broad, don't trace everything. Pick the **3–4 dimensions where this repo is known to be strong** (its reputation tells you — a DB engine earns a perf trace, a CLI tool earns a UX/API trace) and go deep on those. A shallow pass over eight dimensions teaches nothing.

## The dimensions

**Architecture & boundaries**
Module layout, dependency direction, what's core vs. plugin/extension. Find the one diagram-worthy decision: what did they isolate, and what does that isolation buy? Evidence: import graphs, `internal/` dirs, interface definitions at boundaries.

**Build vs. buy**
Read the dependency manifest like a ledger. What did they refuse to depend on and write themselves? That refusal is usually a lesson — find the file and the reason (often in a comment or the PR that added it).

**Failure design**
How errors propagate, what retries/timeouts exist, what invariants get asserted, what happens on partial failure. Grep for their error types, `panic`/`raise`/`unwrap` policy, and recovery paths. The difference between hobby and production code lives here.

**Testing strategy**
Shape of the pyramid, fixtures vs. golden files, property/fuzz testing, how they test the hard parts (time, concurrency, I/O). Read the 3 largest test files — they encode the failure modes the team actually got burned by.

**Hot-path performance**
Where do they break their own style rules for speed? Benchmarks in-repo, arena allocations, caching layers, hand-rolled parsing. Only trace this if perf is part of the repo's reputation.

**Public API & compatibility**
How the public surface is versioned, deprecated, and kept small. Look for `CHANGELOG`, deprecation shims, and how breaking changes were staged across releases.

**Release & ops**
CI matrix, release automation, how a commit becomes a shipped version. Read `.github/workflows/` — mature repos automate their own discipline.

**Docs & contributor culture**
ADRs, comment density on tricky code, how good first issues are groomed. Signals how the codebase stays good, not just how it got good.

## Per-dimension output

Each studied dimension produces the same shape as a specific trace: mechanism (cited) → advantage → cost → portable or not. Then the doc's TL;DR answers: **which one decision, more than any other, makes this repo what it is?**
