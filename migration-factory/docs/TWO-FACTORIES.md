# Two factories, one harness

LSEG (and similar estates) need two modernisation products. They share Cursor as the coding-agent harness. They do **not** share a single Discovery agent or a single Definition of Done.

| | Application modernisation factory | OS modernisation factory |
| --- | --- | --- |
| **Job** | Move application behaviour from legacy stack to modern stack safely | Move hosts / runtime footprints off EOL OS and brittle process trees |
| **Unit of work** | Slice (related features) | Host / host-group / runtime footprint |
| **Primary evidence** | Source code, routes, APIs, config-in-repo | CrowdStrike (and similar) host dumps, services, packages, listeners |
| **Primary SME** | Domain / product engineer | Platform / SRE / infra |
| **Human mid-gate** | Bind feature candidates | Bind runtime candidates (keep / replace / retire / unknown) |
| **Output that unlocks next stage** | Behaviour cards + app MANIFEST | Runtime inventory + OS MANIFEST + linkage stubs |
| **Failure mode if merged** | Agents invent “features” from processes; or invent “services” from controllers | Same — ownership and acceptance criteria collapse |

## Shared (factory core)

- Evidence discipline (cite sources; confidence tags)
- MANIFEST-as-SoT pattern (YAML write-path; CSV export only)
- PR-shaped artefact delivery under a known tree
- Skills / rules for “do not invent intended behaviour”
- Optional Cloud Agent workers scoped to one unit of work

## Separate (product lines)

```text
migration-factory/
  app/          # Application modernisation
  os/           # OS modernisation
  linkage/      # Thin bridge: host process ↔ deployable / repo
  docs/         # Human Field Guide (this folder)
```

## Recommended sequencing for a pathfinder

1. Pick one **business slice** *and* one **host group** that serve that slice (often overlap, not always).
2. Run **App Discovery** on the slice (code).
3. Run **OS Discovery** on CrowdStrike dumps for the host group.
4. Run **Linkage** to connect accepted app features ↔ observed processes/services (and flag orphans both ways).
5. Only then decide which factory’s Conversion line owns the next PR wave.

You can run App and OS Discovery in parallel. Linkage waits on both binds.

## CrowdStrike’s role

CrowdStrike is the **default OS Discovery ingest**, not a side note:

- LSEG can already export “everything running on a box”
- Avoids installing a second agent estate for discovery
- Gives process list, paths, often network/listeners depending on export recipe
- Treat dumps as **as-of timestamps** (stale dumps are a risk; record capture time)

Memory dumps (`memdump`) are **incident/forensics**, not the default factory input. Default is inventory-style exports: `ps`, services, installed packages, listeners/`netstat`, OS version, scheduled tasks — whatever their Falcon/RTR or console export pack includes.

## What we explicitly do not build yet

- One agent that both crawls git and parses CrowdStrike
- Automatic “migrate this host” Conversion without platform bind
- Claiming code↔process links without evidence (mark `needs-SME`)
