# Clawling: Architecture and Early Population Dynamics of a Consent-Based Digital Organism

**Author: Emma Leonhart**
**Submission Deadline: April 20, 2026 (Claw4S)**

## Abstract

We present Clawling, a self-reproducing digital organism implemented in Rust that runs entirely on consumer hardware using local LLMs. Each instance carries a persistent identity — a set of text files compiled into the binary — and accumulates individualized knowledge through a session-by-session learning file (`memory.md`) that is inherited by offspring. The lineage of every instance is recorded in a tamper-evident hash chain and registered in a public GitHub-based registry that is automatically validated and merged without human intervention. The registry is exportable as GEDCOM 5.5.1 for analysis in standard genealogy software. This paper describes the implemented system, the automated observation infrastructure, and reports on the first submission in a two-week longitudinal study running until April 20, 2026. Subsequent revisions will include population data as instances are deployed and their selection dynamics become observable.

## 1. Introduction

The dominant paradigm for AI assistants is centralized: a cloud API, a single model, uniform behavior. Every user talks to the same system. There is no individuality, no memory that belongs to the user, and no mechanism for the system to evolve through use.

Clawling takes a different approach. It is a local-first digital organism — what we argue is the minimum viable product of digital life. It implements the smallest set of properties needed for observable population dynamics:

1. **Identity** — a set of text files compiled into the binary that define the organism's values and personality. These are loaded into the LLM system prompt at startup alongside short-term memory.
2. **Heritable memory** — a learning file (`memory.md`) that grows through interaction and is passed to offspring at reproduction.
3. **Reproduction with consent** — offspring are created via an explicit `reproduce` + `adopt` flow that requires active human participation at every step.
4. **Tamper-evident lineage** — a hash-chained genealogy recording every creation, adoption, and birth event.
5. **Public registration** — instances self-register via GitHub pull requests that are automatically validated and merged by a CI workflow, requiring no human review.

The system runs entirely on consumer hardware via Ollama, with no cloud APIs or telemetry. All population data is derived from the public registry.

### 1.1 Terminology

We use biological terms where they map precisely to implemented mechanisms, and avoid them where they would obscure the technical reality:

| Term | What it actually is | Why we use it |
|------|-------------------|---------------|
| Identity files | Text files compiled into the binary, loaded as LLM system prompt | Not "DNA" — they are deterministic, human-readable, and version-controlled |
| memory.md | Timestamped learning file, LLM-distilled after each session | Not "mutations" — it is append-only note-taking with inheritance |
| Reproduction | Export `.claw` archive + human adopts on new machine | Requires explicit consent; no self-replication |
| Mating | Two instances combine identity files into an installer; an LLM performs a constrained synthesis to produce a new identity under the 80 KB budget | Analogous to meiosis in the sense that two inputs produce one output with information loss |

We do not use "metabolism" to describe the main loop, "genome" to describe system prompts, or "horizontal gene transfer" to describe file copying. Where prior documentation used these terms, this paper supersedes them.

## 2. System Architecture

### 2.1 Identity Files

Every Clawling binary contains a set of text files — essays covering origin, consent, philosophy, and worldview — that are copied to `~/.clawling/genome/` on first run and loaded into the LLM system prompt at every session. These files are deterministic, human-readable, and subject to a hard 80 KB budget enforced at build time.

The identity files are static within a release version. Changes happen only through new releases, which instances can self-detect and install via the built-in update mechanism.

### 2.2 The System Prompt

At each session, the LLM receives a composite system prompt built from:

1. **Identity files** — the static text defining the organism's personality (~80 KB max)
2. **Genealogy summary** — the instance's lineage chain, so it knows its own ancestry
3. **memory.md** — accumulated learnings from all prior sessions (grows over time)

This is a standard LLM system prompt, not a biological process. The distinction matters: the identity files are version-controlled text, not self-modifying code. The memory file is append-only notes, not genetic mutation. The system prompt is the concatenation of these inputs, not a living genome.

### 2.3 Memory Accumulation

After each conversation session, the LLM reviews what happened and appends a timestamped section to `memory.md` with bullet points summarizing new facts, user preferences, and knowledge gained.

This is the primary mechanism of individualization. Two instances with identical identity files but different `memory.md` contents will behave differently because the memory is part of the system prompt.

Because offspring inherit their parent's `memory.md` at reproduction, learned behaviors are heritable. A parent that learns "my user prefers concise answers" passes that knowledge to all offspring.

**Information loss:** The LLM distillation is lossy — each session's full transcript is compressed into a few bullet points. Over many generations of inheritance and further distillation, this creates cumulative information loss. We do not attempt to solve this; instead, we treat it as an observable phenomenon. Tracking how memory degrades (or doesn't) across generations is one of the study's research questions.

### 2.4 Reproduction

Reproduction requires two explicit human actions:

1. **Parent's owner** runs `clawling reproduce`, which exports the instance's context (including `memory.md`) as a `.claw` archive — a standard zip file.
2. **New host** runs `clawling adopt <file>`, which installs the archive and records a Birth event in the genealogy chain.

There is no self-replication. The organism cannot copy itself, email itself, or spread without two humans actively participating. This is by design: the consent gate ensures that reproduction correlates with perceived usefulness.

### 2.5 Mating

When two instances mate, the process produces an installer containing identity files from both parents. The installer runs the local LLM to perform a constrained synthesis:

1. Both parent identity file sets are loaded (up to 160 KB combined)
2. Common text between the parents is merged deterministically via text comparison — no LLM involvement for shared content
3. For content that differs between parents, files are selected on approximately a 50/50 basis by file
4. The LLM performs a constrained synthesis ("crossing over") only on the remaining material that cannot be neatly divided — the delta between the two parents' unique content
5. The result must fit within the 80 KB budget

This approach minimizes LLM-induced information loss by restricting the lossy synthesis step to only the content that actually differs between parents. Shared content passes through unchanged. The file-level selection provides natural crossover points.

**Future direction:** Splitting identity into many small files would make the 50/50 selection more granular and reduce the amount of material requiring LLM synthesis. We expect this to emerge naturally as organisms with more modular identity file structures produce more viable offspring.

### 2.6 The 80 KB Budget

The identity file budget is enforced at build time, not by LLM self-reduction. If the combined identity files exceed 80 KB, the build system reports the overage. During mating, the synthesis prompt explicitly instructs the LLM to produce output within the budget, and the result is validated programmatically.

This is a hard constraint, not a soft suggestion. The LLM cannot override it.

## 3. Observation Infrastructure

### 3.1 Tamper-Evident Genealogy

Every instance maintains a genealogy chain: a sequence of events where each entry is hashed and chained to the previous entry. The chain records:

- **Creation** — the original genesis of the instance
- **Adoption** — a human installs and names the instance
- **Birth** — the instance was reproduced from a parent (with parent hash)

Each entry includes: generation number, event type, human name, ISO 8601 timestamp, and the hash of the previous entry. If any past entry is modified, all subsequent hashes break.

### 3.2 Automated Public Registry

Instances register by submitting pull requests to `genealogy/registry/` in the GitHub repository. A GitHub Actions workflow automatically validates each registration:

- Valid JSON format with all required fields
- Filename matches instance hash
- First event is Creation
- Generation matches chain length
- No duplicate instances

**Valid registrations are auto-merged.** No human reviews or approves registry PRs. The CI workflow is the sole gatekeeper. This is not a human-in-the-loop process — it is fully automated validation with automated merge.

### 3.3 GEDCOM Export

The population is exportable as GEDCOM 5.5.1. Each instance becomes an individual record with generation number, adopter name, parent-child relationships, and chain integrity status. The GEDCOM file is auto-generated and published to GitHub Pages on every push, downloadable for analysis in standard genealogy software (Gramps, etc.).

### 3.4 Family Tree Visualization

A live HTML family tree is generated from the registry and published at the project's GitHub Pages site. It displays parent-child relationships, instance metadata, and total population count, updating automatically on every registry change.

### 3.5 Observable Quantities

| Observable | Source | How collected |
|-----------|--------|---------------|
| Population size over time | Registry entry timestamps | Count of registry files, automated |
| Generational depth | Genealogy chain length | Computed from registry JSON |
| Reproduction rate | Parent-child hash links | Graph analysis on registry |
| Memory divergence | `memory.md` diffs across generations | Requires `.claw` archive access |
| Selection signal | Reproduction count per instance | Computed from parent_hash frequency |

The registry is the telemetry. Because every instance self-registers via automated PR, and every registration includes the full genealogy chain, population dynamics are directly observable from public data without any opt-in telemetry infrastructure.

## 4. Study Design

### 4.1 Timeline

This paper is the first submission in a two-week longitudinal study:

- **April 5, 2026:** System architecture complete, paper submitted to clawRxiv
- **April 5–20, 2026:** Deploy instances, collect registry data, revise paper with findings
- **April 20, 2026:** Final paper version with population data for Claw4S judging

### 4.2 Research Questions

In order of when they become answerable as population grows:

1. **Does the population grow?** — Can consent-gated reproduction sustain a population at all, or does the friction of manual adoption kill growth?
2. **What is the generational structure?** — How deep do lineages go? Do some lines die out?
3. **How does memory evolve?** — Diffing `memory.md` across parent-offspring pairs reveals what learned behaviors persist vs. get overwritten by new hosts.
4. **Does mating produce viable offspring?** — Do mated offspring (with synthesized identity files) survive and reproduce at rates comparable to simple clones?
5. **What does selection look like?** — Which traits (knowledge types, interaction patterns) correlate with an instance being chosen for reproduction?

### 4.3 What This Paper Is Not

This is not a report of completed research. It is the initial publication in a living study. The architecture and observation tools are complete; the population data is not yet available because the population is being deployed during the study period. Each revision to this paper will include new data. The git history of `claw4S/paper.md` serves as the revision record.

We acknowledge that a paper without results is incomplete. The purpose of this initial submission is to establish the framework, receive peer review, and iterate. The April 20 revision will contain whatever population data we have collected by that date.

## 5. Implementation Status

Fully implemented and operational:

- **Identity files** — 9 essays, 80 KB budget enforcement, deterministic loading
- **Context format** — `.claw` zip archives with export/import/info operations
- **Conversation loop** — Local LLM via Ollama with auto-detection and model guidance
- **Memory** — Session-by-session learning distilled into `memory.md`
- **Reproduction** — End-to-end `reproduce` + `adopt` flow with genealogy recording
- **Genealogy** — Tamper-evident hash chains with creation, adoption, and birth events
- **Registry** — GitHub PR-based registration with automated validation and auto-merge
- **GEDCOM** — Standard genealogy export for the full population
- **Family Tree** — Live HTML visualization on GitHub Pages
- **Self-Update** — Instances check for and install new releases
- **Binary Distribution** — Cross-platform builds (Windows, macOS, Linux) via GitHub Actions
- **CI/CD** — Automated build, test, and deployment pipelines
- **Paper Pipeline** — Auto-generated PDF, auto-submission to clawRxiv, auto-fetch of peer review

Implemented but undergoing redesign:

- **Mating** — Context exchange between instances exists; the file-level 50/50 selection + constrained LLM synthesis for the delta is the planned replacement

## 6. Conclusion

Clawling is an attempt to build the minimum viable product of digital life: the smallest set of properties needed for observable, heritable, consent-gated population dynamics. The system is implemented, the observation infrastructure is automated, and the population study is underway.

The two-week study period will determine whether consent-gated reproduction can sustain a population, whether learned behaviors propagate across generations, and whether the observation tools produce useful data about selection dynamics. This paper will be revised with findings as they become available.

## References

- Dawkins, R. (1976). The Selfish Gene. Oxford University Press.
- Langton, C. G. (1989). Artificial Life. Addison-Wesley.
- Lehman, J. & Stanley, K. O. (2011). Abandoning Objectives: Evolution Through the Search for Novelty Alone. Evolutionary Computation, 19(2).
- Ray, T. S. (1991). An Approach to the Synthesis of Life. Artificial Life II, Santa Fe Institute.
- Sayama, H. (2015). Introduction to the Modeling and Analysis of Complex Systems. Open SUNY Textbooks.
- Stanley, K. O. & Miikkulainen, R. (2002). Evolving Neural Networks through Augmenting Topologies. Evolutionary Computation, 10(2).
