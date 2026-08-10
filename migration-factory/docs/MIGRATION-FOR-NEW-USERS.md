# Migration for new users

A plain guide to running the application modernisation factory if you are new to AI coding tools and new to this process.

You do not need to understand how models work. You need to know what each stage is for, what you attach, what you approve, and what “done” means.

If you want the short routing table (stuck at X, run skill Y), use [OPERATOR-RUNBOOK.md](./OPERATOR-RUNBOOK.md).  
If you want the full operating detail, use [FIELD-GUIDE.md](./FIELD-GUIDE.md).  
This doc is the “explain it to me like I just joined” version.

---

## What this factory is (and is not)

**It is** a pipeline of specialised Cursor agents and skills. Each one has one job. You move a small piece of a legacy application (a **slice**) through the line, with human checkpoints between stages.

**It is not** “ask one chatbot to rewrite the whole old system.” That freestyles architecture, invents behaviour, and cannot prove the new system matches the old one.

Think assembly line, not magic wand:

1. Find what the old system actually does  
2. Write tests that pin that behaviour  
3. Record the old answers (or explicitly waive if you cannot)  
4. Lock the target architecture rules  
5. Build the new code under those rules  
6. Prove the new code matches the frozen old answers  

The AI does the heavy reading and drafting. **You** own the judgments that change scope, truth, or risk.

---

## Words you will see

| Term | Plain meaning |
| --- | --- |
| **Slice** | A small, named chunk of the old app you migrate together (example: `account-read`) |
| **Agent** | A Cursor chat that can read many files and propose or edit code |
| **Skill** | A recipe you attach so the agent follows the factory process instead of freestyling |
| **Prompt** | The job description for that stage (under `prompts/`) |
| **Behaviour card** | A plain-English write-up of what one feature does today, with code citations |
| **Characterization** | Tests that pin *current* behaviour, not “how we wish it worked” |
| **Golden** | A frozen recorded answer from the legacy system. Do not rewrite it to make modern code pass |
| **PACK** | The target architecture contract (`PACK.yaml`). After BIND it is law for Conversion |
| **BIND** | A human lock-in (candidates, pack, etc.). Agents do not self-bind |
| **PARITY** | Only Verification may claim the new system matches the goldens (`GREEN` / `FAIL`) |
| **Waiver** | An explicit “we could not run legacy RECORD” record. Conversion may continue. You still must not claim parity |
| **APP_MANIFEST / COVERAGE.md** | App-wide “what we know / what is left.” Read `COVERAGE.md`. Do not hand-edit it |

---

## What you need open

1. The **legacy repo** (the application you are migrating)  
2. The **factory folder** (`migration-factory/`: Field Guide, prompts, schemas, skills)  
3. **Cursor**, where you open a **new agent chat per stage** (keeps jobs clean)

Before any agent run: attach `docs/FIELD-GUIDE.md` plus that stage’s prompt and schemas. Agents are told to refuse without them. That is intentional.

---

## The big picture (one slice)

```text
Pick slice
  → Discovery (find + document as-is behaviour)
  → Test generation (specs + stubs)
  → Test execution (RECORD goldens on legacy)  OR  explicit waiver
  → Architecture (DRAFT pack → human BIND)
  → Conversion (build modern, one behaviour / PR)
  → Verification (COMPARE modern vs same goldens → PARITY)
```

You usually finish one slice before starting another. The whole legacy estate is tracked separately in the portfolio inventory (see the end of this guide).

---

## Stage 0: Pick a slice

**Your job:** Choose a small, honest chunk. Not “migrate everything.”

**Good slice:** one API area, a few related endpoints, clear entry files.  
**Bad slice:** “the whole legacy estate” or “all of order and customer and shared libs.”

**How:** Use the operator skill `slice-scoping` (or the runbook row for it). Write down:

- `SLICE_ID` (example: `account-read`)  
- A short seed in plain language  
- Entry points you already know (RAML, listener XML, main classes)  
- Out of scope (other apps, queues, admin consoles)

**Done when:** You can say the slice id and seeds in one sentence without hand-waving.

---

## Stage 1: Discovery

**What it is for:** Inventory what the old code **actually does** in this slice. No redesign. No “should.”

**Phase A (candidates)**  
Open a new agent. Attach Field Guide + Discovery prompt + schema.  
It produces a candidate list, a stub `MANIFEST.yaml`, and an SME brief.

**Your gate (bind):** For each candidate, say accept, reject, or defer. Record that in the MANIFEST. Do not let the agent invent accepts.

**Phase B (behaviour cards)**  
After bind, deepen only the accepted items.  
Each card explains as-is behaviour with evidence (file paths, line ranges). Weird quirks stay weird. Do not “clean them up” here.

**Done when:** Accepted features are `documented` with cards you have read and are happy with.

**Junior tip:** If something is unclear, mark `needs-SME`. Do not guess.

---

## Stage 2: Test generation

**What it is for:** Turn behaviour cards into a **matrix of characterization cases** and harness stubs. Asserts are often `TO_BE_RECORDED` because the next stage owns real expected values.

**Phase A:** Agent proposes cases (happy path, empty, backend down, backend 5xx, and so on).  
**Your gate:** Approve, drop, or defer rows on the matrix. This stops invented “ISTQB completeness” and speculative tests.

**Phase B:** Agent writes scenario specs + stubs and updates `TRACEABILITY.yaml`.

**Done when:** Approved cases are specified, stubs exist, and you have not claimed any legacy green yet.

**Junior tip:** These are not “tests that prove the rewrite is good.” They are pins for what legacy does today.

---

## Stage 3: Test execution (or waiver)

**What it is for:** Run the harness against **legacy**, freeze **goldens**, then REPLAY until legacy stays green.

**RECORD:** exercise legacy → scrub noise → write golden files.  
**REPLAY:** run again and compare to those goldens.

**Your gate:** Approve goldens before they become the source of truth. You are encoding “bugs as truth” on purpose when that is what production does today.

**If you cannot run legacy** (no licence, no environment): do **not** fake goldens. Use the `waive-characterization` path (`WAIVED_PATHFINDER` or similar). That unlocks Architecture/Conversion for a pathfinder, and it **forbids** claiming `PARITY=GREEN` until real RECORD/COMPARE exists later.

**Done when:** Either `REPLAY_GREEN` on legacy, or an explicit waiver + ADR on disk.

**Junior tip:** Provisional tests written later during Conversion are **not** goldens. They must stay labelled provisional.

---

## Stage 4: Architecture (skill, not a standing agent)

**What it is for:** Produce a machine-readable **PACK** that says how Conversion must build (stack, mapping rules, forbidden patterns, edit surface, quality gates).

**DRAFT:** Architecture skill authors `PACK.yaml` + ADR.  
**Your gate (BIND):** A human sets `status: BOUND` with `bound_by` / `bound_at` in git. Chat “looks good” is not a bind.

After BIND, the pack is law. Conversion must not freestyle a nicer design.

**Done when:** Effective pack is BOUND (and any overlay is BOUND too if you use overlays).

**Junior tip:** If characterization was waived, the pack must say so honestly. Empty fake `replay_green_run_ids` are not allowed without the waiver shape the schema expects.

---

## Stage 5: Conversion

**What it is for:** Implement **documented** behaviours on the target stack, following the BOUND pack. One behaviour per PR by default.

**Your job:** Launch Conversion with the Field Guide + Conversion prompt + pack + cards + batch limit (example: first batch = feature `001` only). Review the PR. Check:

- Edits only inside the allowed edit surface (often `modern/**`)  
- Pack id and version cited  
- Goldens untouched  
- No parity claim  
- Waiver / residual risks called out if relevant  

**Done when:** Features in the batch are converted or explicitly blocked, with `PARITY=UNVERIFIED` handed to Verification.

**Junior tip:** Do not “fix” asymmetric error bodies or rename fields because modern style prefers it. Preserve-wire means preserve the wire.

---

## Stage 6: Verification

**What it is for:** Prove modern matches the **same** legacy goldens. Produce a plain-English narrative and an evidence pack. Set `PARITY=GREEN` or `FAIL`.

**Rules that matter:**

- Mode is COMPARE against read-only goldens  
- Never RECORD from modern (that would rewrite the past)  
- Under a pathfinder waiver with no goldens, Verification cannot honestly mint GREEN. Stay unverified / waived until RECORD exists  

**Done when:** Evidence is on disk and PARITY is an honest status, not a vibe from provisional unit tests.

---

## How you actually click through a day

1. Open the [Operator runbook](./OPERATOR-RUNBOOK.md).  
2. Find the row for where you are stuck.  
3. Run that operator skill (or paste the stage prompt it points at).  
4. Attach Field Guide + schemas every time.  
5. Read the artefacts. Approve or send back with a clear gate reply.  
6. Commit artefacts to git so the next stage (and the next human) can see them.  
7. Move one stage forward. Do not skip binds.

You are not failing if the AI drafts 90% and you change 10%. You are failing if you rubber-stamp scope, goldens, or parity.

---

## How you know what is left in the whole old application

Discovery is slice-scoped on purpose. It never pretends to know the whole estate up front.

App-level tracking lives in:

- `inventory/<APP_ID>/APP_MANIFEST.yaml` (source of truth agents update)  
- `inventory/<APP_ID>/COVERAGE.md` (generated human glance: counts, unscanned hints, status)

Read COVERAGE. **Never hand-edit it.** Regenerate from the manifest.

There is no honest single “87% migrated” number when part of the app is still unscanned. You track known behaviours and unscanned surfaces separately. Only a human residual gate can declare the app complete enough.

---

## Waiver honesty (read this once)

When you cannot run legacy RECORD (no runtime, license, harness, or environment):

- Waive Test execution explicitly (`WAIVED_PATHFINDER` or similar) with artefacts on disk  
- Convert under a BOUND pack that cites that waiver  
- Keep provisional modern tests clearly labelled  
- Refuse to call that “verified” or “parity green”  

A waiver unlocks pathfinder progress on the line. It is not production parity. Verification still owns any future `PARITY=GREEN` claim after real RECORD/COMPARE exists.

---

## What good looks like for a junior

- You can explain each stage in one sentence  
- You never skip a human bind / matrix / BIND / golden / waiver gate  
- You open a new agent per stage instead of one endless chat  
- You treat quirks as data until someone explicitly changes the contract in a new pack version  
- You use COVERAGE.md to pick the next slice, not gut feel alone  

Welcome to the line. Keep the stamps human. Let the agents carry the clipboard.

---


## Experimental: estate discovery overnight

There is an **experimental** overnight prompt (`prompts/estate-discovery-loop-v0.1.md`) that hunts for candidate slices across a larger app. It only writes Phase A candidates into the portfolio inventory. It does **not** replace Discovery bind, and it must never be told it "found everything."

Until it graduates from experimental, treat the normal slice-by-slice path in this guide as the default.

## Where to go next

| Need | Open |
| --- | --- |
| Gate → skill routing | [OPERATOR-RUNBOOK.md](./OPERATOR-RUNBOOK.md) |
| Full stage how-to | [FIELD-GUIDE.md](./FIELD-GUIDE.md) |
| Why two factories (app vs OS) | [TWO-FACTORIES.md](./TWO-FACTORIES.md) |
| Operator skills | `../skills/operator/README.md` |
| Portfolio schema | `../schemas/app-manifest.schema.md` |
| Experimental estate discovery | `../prompts/estate-discovery-loop-v0.1.md` |
