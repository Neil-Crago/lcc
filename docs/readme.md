# CrystalEngine 2.0: SpaceTCO LCC Solver

This repository contains the reference implementation of the **Linear Conglomerate Calculus (LCC)**, a bipartite tensor instruction set architecture (ISA) designed for deterministic consensus and thermal efficiency.

## Prerequisites

* **Rust Toolchain:** Install `rustc` and `cargo` via [rustup.rs](https://rustup.rs/).
* **Dependencies:** The engine utilizes `num-integer` for GCD calculations and `uuid` for node identification.

## Build Instructions

To compile the engine with maximum optimizations (recommended for benchmarking):

```bash
cargo build --release
```

## Running Benchmarks

The engine includes three primary validation tests: `ComparativeTest1` (Standard LinAlg vs LCC), `ComparativeTest2` (TAD-Bridge Performance), and the `MaximumRecoveryTest`.

```bash
cargo run --release
```

## Project Structure

* `BipartiteTensor`: Implements hardware-level gate partitioning[cite: 188].
* `GeodesicSolver`: Executes the Axiomatic Truth Transform via the Path of Least Action.
* `TAD`: Implements Topologically Associating Domains for resonance recovery.

## Rosetta Stone

| Rust Variable / Struct | LCC Academic Mapping | Hardware/Logical Function |
|---|---|---|
| anchor: u32 | Prime Anchor (n) | Settled integer identity stored in the vaulted B0-31 segment. |
| delta: f32 | Residual Phase Variance | Fractional jitter processed in the voltage-gated B32-63 segment. |
| mass() | Identity Density | Metric calculating the stability of a node; 1.0 represents absolute truth. |
| action: f32 | dS (Accumulated Energy) | The computational expenditure required to traverse the field. |
| suction_break() | Quantized State Finalization | The deterministic trigger for snapping jitter into a crystalline anchor. |
| GeodesicSolver | Axiomatic Truth Transform | The engine that calculates the path of least action through the 156-Metric. |
| TAD | Topologically Associating Domain | A localized resonance scaffold used to recover high-latency data. |

## Citation

If using this engine for research, please cite the associated PeerJ manuscript:
*Crago, N. (2026). A Bipartite Tensor Instruction Set Architecture (ISA) for Deterministic Consensus and Thermal Efficiency in High-Performance Computing.*

## License

MIT

## DOI

DOI: [10.5281/zenodo.19731832](https://doi.org/10.5281/zenodo.19731832)
  
You can however cite all versions by using the DOI: [10.5281/zenodo.19728655](https://doi.org/10.5281/zenodo.19728655) as this will always resolve to the latest one.

---
  