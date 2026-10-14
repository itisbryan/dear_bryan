---
name: browser-lifecycle
description: Use when an agent opens or controls a browser for web automation, QA, screenshots, or tests, especially with OMP browser tabs, Playwright, Puppeteer, Selenium, or Chrome. Covers reusing tabs, closing owned browser resources, and safely handling stale processes.
---

# Browser lifecycle

Browser automation owns resources. Release what this task created, even if navigation, a test, or an assertion fails. A skill reduces normal-case leaks; it cannot clean up after a crashed or forcibly terminated agent.

## Before opening

1. Prefer the harness's managed browser over launching Chrome from a shell. Reuse an existing task-owned tab/context when possible instead of creating a browser per page or check.
2. Decide ownership before acting: **owned** (launched/opened for this task), **shared** (another task or agent uses it), or **attached** (a user's browser, CDP connection, relay, or existing browser). Never close or terminate a shared or attached browser. A task may release only its own automation tab if the backend supports that without closing the user's tab.
3. Record the handles needed for cleanup (tab, context, browser, driver). Do not rely on a PID or a profile directory name as proof of ownership.

## During and after the task

- Put cleanup in `finally` around the work that acquired the resource. Close owned tabs/contexts before closing an owned browser or driver. When an API's close semantics are unclear, check whether it closes the *connection*, the *page*, or the *entire browser* before calling it on an attached target.
- On cancellation or a failed check, still attempt the same owner-scoped cleanup. Report a cleanup failure rather than claiming the browser was closed.
- Keep a browser open only when the user explicitly needs a persistent session; identify the retained resource and why it remains. Do not use persistence merely to avoid reopening a page.
- At handoff or completion, account for each resource opened by the task. If the agent cannot run cleanup (crash, force kill), note that manual/runtime recovery may be necessary.

## OMP browser facade

- `browser.open({ name: "task-name" })` opens or reuses a named tab. Choose a task-unique name and reuse it for that task; `browser.tab(name)` is only a proxy and does not open/check a tab.
- Keep the returned `tab` handle and call `await tab.close()` when finished. If `browser.open` fails before returning a handle (for example, navigation times out), check whether **that task-owned name** remains in `browser.tabs()` and release it. For multiple owned tabs, close those names individually. Do **not** call `browser.close({ all: true })` in a shared agent environment: it can release tabs belonging to other work.
- Leave `persist` at its default `false` unless continued use is intentional. OMP may freeze owned tabs at turn settle and idle-close eligible tabs later; that timeout is a fallback, not a substitute for explicit close.
- For a spawned `app.path` browser, ordinary tab close does not terminate the application. Only use `kill: true` when terminating a browser spawned and owned by this task is intended and no other managed tab depends on it. Never use `kill` to try to terminate a relay or CDP-attached user browser.

```js
await (async () => {
  const name = `qa-check-${crypto.randomUUID()}`;
  let tab;
  try {
    tab = await browser.open({ name, url: "https://example.com" });
    // Inspect or interact with the page.
  } finally {
    if (tab) await tab.close();
    else if ((await browser.tabs()).some(t => t.name === name)) {
      await browser.tab(name).close();
    }
  }
})();
```

## Other automation libraries

- For Playwright/Puppeteer, reuse a browser and create short-lived contexts/pages as needed. Close owned pages/contexts, then the browser **only if this task launched it**; keep launch and cleanup in `try/finally` or the framework's teardown. For Selenium, call `driver.quit()` on a driver this task created.
- If attaching to an existing browser, use the library's disconnect/release mechanism where available; do not assume `browser.close()` is safe for that backend. Verify the library's semantics first.
- If a shell command launches a test runner, configure the test runner's normal teardown and wait for it to exit. The OMP tab API cannot manage unrelated Chrome processes launched by shell scripts.

## Suspected stale Chrome processes

- Distinguish a running CPU-consuming process from a zombie PID or an unused profile directory. Inspect parent process, command line, profile path, and whether a task still owns it before taking action.
- Never run a blanket `pkill Chrome`, delete all browser profiles, or kill a PID solely because it is old. A PID may have been reused; another agent or the user may be using the browser. Do not delete a profile while its browser is running.
- Prefer the original automation API's graceful close. If no owner is live and an independently launched process is confirmed to belong exclusively to the task, ask before terminating it or deleting its profile; otherwise leave it and report the evidence.
