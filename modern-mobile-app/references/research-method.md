# Design from Shipped References (reference)

Source: Mobbin, "I Gave Claude 600,000 UI Screens… Then This Happened" (demo of Claude connected to Mobbin's MCP server over 600,000+ real, shipped app screens and flows).
Core thesis: **AI design quality jumps when the model researches real shipped UI instead of relying on its own generic patterns.** Asked to draft a fintech onboarding flow twice, Claude produced something generic without the library and a far more "considered" one with it — grounded in real finance apps and accounting for identity checks and compliance.

## The research-then-design workflow

1. **Research the category first.** Before drafting a flow, research how the top apps in the category handle it: find the most relevant screens and end-to-end flows, compare patterns across products, identify what the best apps do differently.
2. **Output a traceable report.** Common patterns, trade-offs, and blind spots — every decision cites a shipped example the team can click through. References are evidence, not decoration. (In the demo: a Curve onboarding proposal placed the setup bonus on the homepage right after onboarding, justified by the app One doing the same.)
3. **Compare competitors at screen level.** Head-to-head teardowns of specific flows (e.g., DoorDash vs. Uber Eats checkout tipping) surface concrete differentiators, not vague "best practices."
4. **Treat copy as a first-class variable.** Distill what top apps do differently in their wording, not just layout — then tailor copy to your audience with cited inspirations.
5. **Build hard constraints into the flow architecture early.** Fintech onboarding must incorporate identity checks and compliance — these reshape onboarding, they don't just add steps. Industry context must shape the flow (banking onboarding is not generic onboarding).

## Rules of thumb

- The best apps in a category converge on common patterns; designing with those patterns is a strength, not copying.
- Copy quality differentiates top apps as much as layout does.
- Every design decision should be traceable to a shipped example.

## Warnings

- Don't accept AI-generated UI uncritically: in the demo, some generated iOS screens came out visibly broken.
- Don't rely on the model's unaided taste: without real references, output looks fine but generic.
- Convergence on a pattern doesn't mean it's right for your product — mind the blind spots and trade-offs.
- Grounding data is only as good as the library's coverage.
