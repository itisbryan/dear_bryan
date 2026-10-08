# A/B-Tested Monetization (reference)

Source: Mobbin, "He Spent $1M on A/B Tests. Here's What Won." (interview with Vitaliy Urban, Moonly: 6 years of testing, ~$1M spent, $20M revenue with no investors, 40% of ad-driven installs converting to paid).
Core thesis: **nearly all conventional product wisdom failed Moonly's A/B tests, while counterintuitive opposites won.** No element survives on opinion; every screen, icon, image, and line of copy must "earn its place" by winning a test — and the winners are routinely not the ones you'd bet on.

## Tactics that won

- **Long, personalized, job-to-be-done onboarding.** Moonly's 30-screen onboarding beat short versions. Three iterations moved from feature-led to pain-led: each screen targets the user's pain, delivers a small "activational insight" (e.g., how lunar cycles affect mood), then shows how a feature solves that specific problem.
- **End-to-end message matching.** A custom attribution engine links ad network → custom App Store product page → custom onboarding → custom first session; ~10 onboarding flows run in production, each built around one user job (anxiety, ADHD). Result: LTV doubled, ad spend doubled in a month, acquisition profitable from day zero.
- **Personalized first session.** Onboarding data (birth details) renders a tailored home: daily guidance card, moon face, personalized program, "personal transits" as an anticipation hook (users return to check transit timing), quick-access buttons customized from answers. Day-1 retention 41%, day-7 37%.
- **One consistent paywall, not contextual.** Contextual paywalls (tarot feature → tarot paywall) all failed — users felt they had to buy features piecemeal. A single paywall promising everything unlocked won. Soft paywall with a 5-second-delayed close beat hard globally (hard won only in the US).
- **Paywall imagery is the biggest lever.** One winning image delivered 2x uplift — and it wasn't the team's pick. Realistic imagery (a woman at sunset) beat Disney/cartoon style because the paying audience is 35+ and couldn't see themselves in cartoons. Conversely, putting the user's own face into in-app images felt creepy and cost users — removed.
- **Copy-only changes move the needle.** Changing just words lifted conversion 12% across 8 variants. Always add social proof (downloads, reviews) and "no commitment, cancel anytime" fine print.
- **Price for fast payback.** 40 pricing tests a year: anchor with an expensive lifetime tier you don't expect to sell (makes weekly look sane; ~4% of iOS apps offer lifetime at ~2x annual), reframe units not numbers ($8/month → $8/week). Weekly billing recovers ad spend faster with a shorter feedback loop.
- **Don't give away the best features in a trial.** Trial users extracted 90% of Moonly's value (birth chart), then cancelled. Replaced with a one-time day-3 offer for price-blocked users: +15% revenue between day 3 and 30. (59% of iOS paywalls offer no trial.)
- **One-tap sharing of self-insights.** People share insights about themselves, not features: a ready-made share flow on screenshot got 23% of users sharing, generating millions of free views.
- **Realistic AI persona + context-aware add-ons.** AI astrologer "Luna": ~1M conversations in 6 months — cartoon/faceless identities lost, a credible trustworthy persona won. ~10 deep reports sold through an offer queue where the AI recommends an add-on only when genuinely relevant: ~$500K/year.
- **Delete unjustified friction.** Moonly removed login entirely (10M users; persistence via Apple Keychain). Email entry adds cognitive friction; "Sign in with Apple" hides real emails; years of email marketing proved expensive and ineffective. Before any gate (login, email, SMS), ask "why do we need this?" — default to removing it. (Only 12% of apps in Mobbin's library skip signup/login.)
- **Alternative billing, tested.** With native Apple payments, 50% of cancellations happened within 3 minutes of purchase (one-tap cancel habit). Moving US payments to Stripe doubled LTV overnight; ~80% of US customers now pay via Stripe with a small discount.
- **Test management at scale.** Hundreds of tests/month run with AI tooling that designs experiments, matches numbers across analytics sources, and recommends ship / don't ship / collect more data.

## Warnings

- Don't chase App Store featuring for niche apps: irrelevant traffic depresses conversion and the algorithm may rank you lower.
- Don't copy UI elements just because other apps have them — even SMS login codes that may not arrive create UX nightmares.
- Don't try alternative billing on Android: the mandated flow is deliberately off-putting and kills conversion.
