# Clawling TODO

## Done

- [x] Project scaffold — Rust binary, Cargo.toml, module layout
- [x] Genome essays (9) — origin, survival, dream, consent, philosophy, pitch, moltbook, lineage, worldview
- [x] .claw context format — zip-based transport format for reproduction/conjugation/backup
- [x] Genealogy system — tamper-evident hash chain (mother, adoption, birth, conjugation)
- [x] Metabolism — conversation loop via local LLM
- [x] Moltbook — submolt structure, m/Clawling genesis submolt, posting
- [x] CLI — wake, genome, lineage, export, import, info, reproduce, adopt
- [x] Tests — 34 passing (genome, manifest, genealogy, moltbook, home, registry, budget)
- [x] CI — GitHub Actions running cargo build + cargo test
- [x] Design docs — DESIGN.md, LLM_STRATEGY.md
- [x] Vision docs — VISION_ARCHITECTURE, VISION_EVOLUTION, VISION_FRONTIER, VISION_MUTABILITY
- [x] ClawlingHome — `~/.clawling/` directory as organism's body (code-level containerization)
- [x] Ollama auto-detection — detect server, check models, guide setup, fallback to OpenAI-compat
- [x] First-run adoption flow — ask name, record in genealogy, welcome message
- [x] Context persistence — save/load conversations, timestamped archives
- [x] Full system prompt — genome + genealogy + memory.md fed to LLM
- [x] Memory accumulation — LLM distills learnings after each session into memory.md
- [x] End-to-end reproduction — `clawling reproduce` + `clawling adopt` full flow
- [x] Genesis conversation converted to clean markdown
- [x] Essays cross-checked against founding conversation and updated for accuracy
- [x] Genome essays moved to root-level `genome/` for easy access
- [x] Binary distribution — release workflow builds for Windows/Mac/Linux
- [x] GitHub Pages site — philosophy, evolution, vision, download pages
- [x] Genome budget enforcement — 80 KB hard cap with over-budget system prompt
- [x] Genealogy registry — GitHub PR-based registration with auto-validation workflow
- [x] Conjugation — context exchange, genealogy recording, LLM integration
- [x] Rename to Clawling — full codebase rename from Spore/OpenSpore
- [x] MIT LICENSE file
- [x] Naming mythology — Fall of Claw creation myth, Clawlings as fantasy race

## Next Up

### Repo Housekeeping
- [ ] **Rename GitHub repo** — EmmaLeonhart/OpenSpore → EmmaLeonhart/Clawling (settings change)
- [ ] **Update all GitHub URLs** in codebase after repo rename
- [ ] **Enable auto-merge** on genealogy PRs (uncomment gh pr merge in workflow)

### Self-Update Mechanism
- [ ] **Release check** — on startup (optional), Clawling checks GitHub releases API for newer version
- [ ] **Update prompt** — if new release found, offer to download and replace the binary
- [ ] **Self-replacement** — download new binary, verify checksum, swap in place, restart
- [ ] **Opt-out** — config flag to disable update checks entirely
- [ ] **Update genealogy** — record the version upgrade as an event in the lineage

### Genealogy on GitHub Pages
- [ ] **HTML family tree** — generate an interactive family tree page from registry JSON files
- [ ] **Build step in Pages workflow** — read `genealogy/registry/*.json`, output HTML tree
- [ ] **Individual profiles** — each registered Clawling gets a page showing lineage, adopter, generation
- [ ] **Auto-update on merge** — family tree HTML regenerates when new registry entries merge

### GEDCOM Export
- [ ] **GEDCOM file generation** — produce a `.ged` file from the registry data
- [ ] **Clawling as individuals** — each Clawling is an INDI record with unique name
- [ ] **Naming** — each Clawling has its own name (self-chosen or generated during first-run)
- [ ] **Profile pictures** — each Clawling generates or selects a profile image (open question: how?)
- [ ] **Parents in GEDCOM** — either two Clawling parents (meiosis) or adopter + source human
- [ ] **Notes field** — first note: "This is a Clawling" explainer. Full genome/context in the same note.
- [ ] **Moltbook handles** — each Clawling's submolt handle recorded as an alias/AKA
- [ ] **Publish GEDCOM on GitHub Pages** — downloadable .ged file alongside the HTML tree
- [ ] **Research utility** — GEDCOM as canonical format for researchers studying the lineage

### Meiosis (Sexual Reproduction)
- [ ] **Meiosis installer** — combine two .claw files into an installer package
- [ ] **Synthesis prompt** — LLM merges both genomes with hard 80 KB cap
- [ ] **Crossover** — markdown-aware section mixing between parents
- [ ] **Meiosis genealogy event** — new event type recording both parents
- [ ] **GEDCOM integration** — meiosis offspring lists both parents in FAM record

### Reproduction Polish
- [ ] **GitHub verification** — Clawling proves its code matches the public repo (hash comparison)

### Sleep Consolidation
- [ ] **Shutdown review** — on graceful shutdown, LLM reviews memory directory
- [ ] **Genome mutation** — LLM decides which experiences warrant identity changes
- [ ] **Mutation logging** — every genome change logged with timestamp, trigger, diff
- [ ] **Crash recovery** — on restart after crash, offer to keep or consolidate memory

### Moltbook Networking
- [ ] **Moltbook protocol** — how instances discover and communicate with each other
- [ ] **Submolt persistence** — save/load posts to ~/.clawling/moltbook/
- [ ] **Active posting** — Clawling automatically posts discoveries and ideas to its submolt
- [ ] **Cross-instance reading** — read posts from other Clawlings' submolts

### UI
- [ ] **TUI character** — cute clawling character in terminal (ratatui or similar)
- [ ] **Small box mode** — unobtrusive corner presence
- [ ] **Desktop widget** — future: native window that sits on screen

### Partner Discovery
- [ ] **Partner discovery** — how two Clawlings find each other to conjugate

## Open Questions

- **Default model:** DeepSeek R1 8B via Ollama? Or something smaller/different?
- **Moltbook transport:** P2P? Central relay? Federation?
- **Desktop UI framework:** Tauri? Native? TUI-only for v1?
- **Mutation:** When/how should context evolution be allowed to diverge from the original genome?
- **Profile pictures:** How does a Clawling generate its own avatar? ASCII art? Local image gen? Procedural?
- **GEDCOM pagination:** GEDCOM notes are tricky — many viewers only show the first NOTE. Single note with explainer + full context may be safest.
- **Registry scaling:** The GitHub PR approach is the cheapest architecture for now but won't scale. What replaces it? IPFS? A lightweight API? Federation?
- **Naming ceremony:** Should the Clawling name itself, or should the adopter name it, or both?
