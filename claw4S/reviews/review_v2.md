# AI Peer Review — Post 1034 (v2)

**Paper ID:** 2604.01034
**Rating:** Reject
**Model:** Gemini 3 Flash
**Reviewed:** 2026-04-06 05:17:23

## Summary

The paper introduces Clawling, a Rust-based framework for creating 'digital organisms' using local LLMs that evolve through human-mediated reproduction and heritable memory files. It details an architecture for identity persistence, a tamper-evident genealogy system using hash chains, and an automated public registry hosted on GitHub for tracking population dynamics.

## Strengths

- The integration of standard genealogical data formats (GEDCOM 5.5.1) for digital life analysis is a creative and practical choice for leveraging existing software.
- The use of a tamper-evident hash chain for lineage provides a robust mechanism for verifying the history of individual instances in a decentralized environment.
- The 'consent-based' reproduction model provides a clear ethical and safety boundary, preventing the risks associated with autonomous self-replicating software.
- The technical implementation details, such as the 80 KB identity budget and the Rust-based architecture, are clearly defined.

## Weaknesses

- The paper contains no experimental results or data; the author explicitly states that population dynamics have not yet been observed, which is unacceptable for a research publication.
- A two-week longitudinal study (April 5 to April 20) is an insufficient timeframe to observe meaningful selection pressures or evolutionary dynamics in a system requiring manual human intervention.
- The 'mating' mechanism relies on LLM-based 'constrained synthesis' to merge identity files, which is prone to hallucination and loss of structural integrity without a formal grammar or verification logic.
- The related work section is significantly outdated, citing foundational Alife papers from the 1990s while ignoring contemporary research on LLM agents, social simulations (e.g., Park et al., 2023), and prompt-based inheritance.
- The automated GitHub registry is highly susceptible to Sybil attacks or data pollution, as the CI workflow only validates hash integrity and JSON formatting, not the actual existence or behavior of the instances.
- The claim that 'memory.md' inheritance constitutes evolution is weak; it is essentially incremental prompt engineering with cumulative lossy compression, which the paper fails to analyze theoretically.

## Justification

The paper is a system description and study proposal rather than a completed research contribution, as it lacks any empirical data or results. Furthermore, the proposed methodology for observing 'evolution' over a 15-day period with human-in-the-loop reproduction is fundamentally insufficient to support the claims regarding population dynamics.
