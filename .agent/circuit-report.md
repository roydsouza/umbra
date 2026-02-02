# Circuit Mapping Report (v2)

**Contextual Grounding:** Follow the architecture defined in `arti-expert.md` and respect the limits in `physical-constraints.md`.

**Steps:**
1. **Probe:** Execute `map_arti_internals.py` to locate Prop 340 implementation.
2. **Filter:** Discard any results that don't include `tracing` hooks, as per our focus in `physical-constraints.md`.
3. **Draft:** Create a implementation plan for a new crate in `modules/`. 
   * *Self-Correction:* If the plan involves modifying `arti-upstream`, restart the draft. (Rule: Modular First).
4. **Final Review:** Verify the proposed Rust code uses `aarch64` specific optimizations for the M5.
5. **Output:** Write the finalized report to `artifacts/prop340-mapping.md`.

