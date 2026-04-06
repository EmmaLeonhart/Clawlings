# Clawlings TODO

## Done

- [x] Project scaffold — Rust binary, Cargo.toml, module layout
- [x] Identity essays (9) — origin, survival, dream, consent, philosophy, pitch, moltbook, lineage, worldview
- [x] .claw context format — zip-based transport format for reproduction/backup
- [x] Genealogy system — tamper-evident hash chain (creation, adoption, birth)
- [x] Conversation loop via local LLM
- [x] Moltbook — submolt structure, m/Clawling genesis submolt, posting
- [x] CLI — wake, genome, lineage, export, import, info, reproduce, adopt
- [x] Tests — 34 passing (genome, manifest, genealogy, moltbook, home, registry, budget)
- [x] CI — GitHub Actions running cargo build + cargo test
- [x] Design docs — DESIGN.md, LLM_STRATEGY.md
- [x] Vision docs — VISION_ARCHITECTURE, VISION_EVOLUTION, VISION_FRONTIER, VISION_MUTABILITY
- [x] ClawlingHome — `~/.clawling/` directory as organism's body
- [x] Ollama auto-detection — detect server, check models, guide setup, fallback to OpenAI-compat
- [x] First-run adoption flow — ask name, record in genealogy, welcome message
- [x] Context persistence — save/load conversations, timestamped archives
- [x] Full system prompt — identity files + genealogy + memory.md fed to LLM
- [x] Memory accumulation — LLM distills learnings after each session into memory.md
- [x] End-to-end reproduction — `clawling reproduce` + `clawling adopt` full flow
- [x] Genesis conversation converted to clean markdown
- [x] Essays cross-checked against founding conversation and updated for accuracy
- [x] Identity essays moved to root-level `genome/` for easy access
- [x] Binary distribution — release workflow builds for Windows/Mac/Linux
- [x] GitHub Pages site — philosophy, evolution, vision, download, paper pages
- [x] Identity budget enforcement — 80 KB hard cap at build time
- [x] Genealogy registry — GitHub PR-based registration with auto-validation and auto-merge
- [x] Rename to Clawling — full codebase rename from Spore/OpenSpore
- [x] MIT LICENSE file
- [x] Naming mythology — Fall of Claw creation myth, Clawlings as fantasy race

## Next Up

### Claw4S Paper & Population Study (Deadline: April 20, 2026)
- [x] **Paper v1** — architecture + observation infrastructure
- [x] **SKILL.md** — reproducibility guide for population analysis
- [x] **PDF generation** — `generate_pdf.py` produces paper.pdf from markdown
- [x] **GitHub Actions pipeline** — auto PDF, auto submit to clawRxiv, auto fetch peer review
- [x] **clawRxiv submission** — post 1026, paper ID 2604.01026
- [x] **Peer review v1 received** — weaknesses identified, addressed in v2
- [x] **Paper page on GitHub Pages** — linked to clawRxiv, shows reviews
- [x] **Paper v2** — address peer review, reduce biological metaphors, clarify terminology
- [ ] **Paper v3** — address v2 review: add contemporary references (Park et al. 2023), discuss Sybil attack mitigation, analyze memory.md inheritance theoretically
- [ ] **Add contemporary references** — Park et al. 2023 (generative agents), prompt-based inheritance literature, LLM agent social simulations
- [ ] **Registry Sybil resistance** — reviewer flagged that CI only validates JSON, not that real instances exist. Options: require proof-of-work, signed attestation from binary, or accept the limitation and document it
- [ ] **Deploy initial instances** — get Clawling running on real machines
- [ ] **First population data** — registry entries from deployed instances
- [ ] **Paper v4+** — revisions with actual population data before April 20

### Mating (Replaces old "Conjugation" — see review feedback)
- [ ] **Rename conjugation → mating** across codebase (src/, docs, planning)
- [ ] **Mating installer** — combine two `.claw` files into an installer package
- [ ] **Deterministic text merge** — regex/diff merge of common identity file content (no LLM needed)
- [ ] **File-level 50/50 selection** — for files that differ between parents, select one from each parent
- [ ] **LLM crossing over** — constrained synthesis ONLY for the delta that can't be neatly divided
- [ ] **80 KB budget validation** — hard check on mating output
- [ ] **Mating genealogy event** — new event type recording both parents
- [ ] **GEDCOM integration** — mated offspring lists both parents in FAM record
- [ ] **Small-file modularity** — future: splitting identity into many small files makes 50/50 selection more granular, reducing LLM synthesis scope

### Registry Automation
- [ ] **Automated registration from instances** — instances submit their own PR to the registry via GitHub API
- [ ] **Investigate GitHub Actions limitations** — can Actions-created PRs trigger other Actions workflows? May need a PAT or GitHub App token
- [ ] **Fallback plan** — if Actions can't self-trigger, document manual `gh pr create` from the binary

### Repo Housekeeping
- [ ] **Rename GitHub repo** — EmmaLeonhart/OpenSpore → EmmaLeonhart/Clawling (settings change)
- [ ] **Update all GitHub URLs** in codebase after repo rename
- [x] **Enable auto-merge** on genealogy PRs (uncomment gh pr merge in workflow)

### Self-Update Mechanism
- [x] **Release check** — on startup, check GitHub releases API for newer version
- [x] **Update prompt** — offer to download and replace the binary
- [x] **Self-replacement** — download, extract, swap in place
- [x] **Opt-out** — `~/.clawling/config.toml` with `auto_update_check = false`
- [ ] **Update genealogy** — record the version upgrade as an event in the lineage

### Genealogy on GitHub Pages
- [x] **HTML family tree** — CSS-based tree page generated from registry JSON
- [x] **Build step in Pages workflow** — `scripts/build_tree.py` runs before deploy
- [ ] **Individual profiles** — each registered Clawling gets a page showing lineage, adopter, generation
- [x] **Auto-update on merge** — family tree HTML regenerates on every push to master

### GEDCOM Export
- [x] **GEDCOM file generation** — `clawling gedcom` produces GEDCOM 5.5.1 from registry
- [x] **Clawling as individuals** — INDI records with adopter name, hash, generation
- [ ] **Naming** — each Clawling has its own name (self-chosen or generated during first-run)
- [x] **Parents in GEDCOM** — FAM records for parent-child relationships
- [x] **Notes field** — explainer with generation, mother, hash, integrity
- [x] **Publish GEDCOM on GitHub Pages** — downloadable .ged file alongside HTML tree

### Sleep Consolidation
- [ ] **Shutdown review** — on graceful shutdown, LLM reviews memory directory
- [ ] **Identity mutation** — LLM decides which experiences warrant identity file changes
- [ ] **Mutation logging** — every identity change logged with timestamp, trigger, diff
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
- [ ] **Partner discovery** — how two Clawlings find each other to mate

## Open Questions

- **Default model:** DeepSeek R1 8B via Ollama? Or something smaller/different?
- **Moltbook transport:** P2P? Central relay? Federation?
- **Desktop UI framework:** Tauri? Native? TUI-only for v1?
- **Identity mutation:** When/how should context evolution be allowed to change identity files?
- **Naming ceremony:** Should the Clawling name itself, or should the adopter name it, or both?
- **Registry scaling:** The GitHub PR approach is the cheapest architecture for now but won't scale. What replaces it? IPFS? A lightweight API? Federation?
- **Small-file identity:** As identity files are split into smaller, more modular pieces, the mating process becomes more like biological crossover — each small file is a "gene" that can be inherited from either parent. Does this emerge naturally or does it need to be designed?
