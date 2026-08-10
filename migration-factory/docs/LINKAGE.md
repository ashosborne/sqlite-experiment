# Linkage bridge (thin)

Connects **Application Discovery** artefacts to **OS Discovery** artefacts without merging the factories.

## Why it exists

App-mod knows features and repos. OS-mod knows processes and hosts. Migration planning needs both:

- Which hosts must move before / with which app slices?
- Which processes are orphans (running, no known repo)?
- Which documented features have no runtime footprint in the dump (wrong host group, batch job, or missing dump)?

## Inputs

- App: `discovery/<SLICE_ID>/MANIFEST.yaml` (accepted / documented features + entrypoints)
- OS: `os-discovery/<HOST_GROUP_ID>/MANIFEST.yaml` (bound runtime items)
- Optional: CMDB / deploy manifests / service catalogues if LSEG provides them

## Outputs

```text
linkage/<PAIR_ID>/
  LINKS.yaml          # confirmed + candidate links
  ORPHANS.md          # app-without-runtime / runtime-without-app
  SME_BRIEF.md
```

## Rules

- Default confidence is `inferred` until a human confirms
- Matching heuristics (path tokens, port, service name, package) are hints, not truth
- Linkage agent does not re-run Discovery
- Confirmed links are the only ones Conversion may treat as dependencies

## Human gate

Platform + domain jointly confirm links that affect sequencing (e.g. “NCD API slice runs on these three Win2012 hosts”).
