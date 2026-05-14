/// SpaceTCO: LCC Strict Integer Execution Test Code in Rust (CrystalEngine 2.2)
/// This code simulates the execution of the Linear Conglomerate Calculus (LCC) using a strictly integer-based approach, designed to reflect the constraints of a custom ASIC implementation. The logic engine processes discrete quanta of phase transitions, applying fixed-point arithmetic for all calculations to maintain silicon-native performance. The benchmark generates deterministic noise patterns and categorizes outcomes based on predefined thresholds, demonstrating the robustness of the Water Hammer Protocol in handling thermal loads and incoherent jitter.
/// To run the benchmark, simply execute the compiled binary. You can specify the number of samples with the `--samples` or `-s` flag (e.g., `lcc --samples 200000`). The output will include the distribution of delta quanta, exact matches, failures, and execution time, providing insights into the behavior of the integer-based LCC under simulated conditions.
/// Note: This code is intended for benchmarking and demonstration purposes. In a production environment, additional optimizations and safety checks would be necessary, especially when translating to hardware-level RTL code.
/// Author: Neil Crago
/// Date: 2024-05-14
use std::env;
use std::hint::black_box;
use std::time::Instant;

// Integer scaling constants used in place of floating point.
const PI_QUANTA: u32 = 3_141_592; 
const PPM_SCALE: u32 = 1_000_000;

// Thresholds mapped to discrete integers
const SNAP_ZONE: u32 = PI_QUANTA / 6;
const HOLD_ZONE: u32 = PI_QUANTA / 4;
const SHATTER_ZONE: u32 = PI_QUANTA / 3;

// Fixed-point thresholds represented in parts per million (PPM).
const FIELD_VISCOSITY_PPM: u32 = 500_000; // 0.5
const THERMAL_LIMIT_PPM: u32 = 850_000;   // 0.85
const THERMAL_SPIKE_PPM: u32 = 50_000;    // 0.05

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorState {
    IncoherentNoise,      // Triggers Absolute Sublimation (0V)
    HighLatencyIsotope,   // Triggers Glass Transition (Hold)
}

// --- 1. The Bipartite Tensor (Hardware-Level Gate Partitioning Layout) ---
#[derive(Debug, Clone, Copy)]
pub struct BipartiteTensor {
    pub anchor: u32,       // Bits 0-31: The Prime Anchor (n)
    pub delta_quanta: u32, // Bits 32-63: The Jitter Residue (Discrete Integer)
}

impl BipartiteTensor {
    /// Returns the "Mass" (Identity Density) of the node as a PPM integer.
    /// Mass = PPM_SCALE when delta_quanta is 0.
    pub fn mass_ppm(&self) -> u32 {
        if self.delta_quanta >= SNAP_ZONE {
            0
        } else {
            // Equivalent to: 1 - delta_quanta / SNAP_ZONE in fixed-point form.
            let reduction = ((self.delta_quanta as u64 * PPM_SCALE as u64) / SNAP_ZONE as u64) as u32;
            PPM_SCALE.saturating_sub(reduction)
        }
    }
}

// --- 2. The TAD (Topologically Associating Domain) ---
pub struct TAD {
    pub range_start: usize,
    pub range_end: usize,
    pub resonance_boost_ppm: u32, // Neighborhood Catalyst in PPM
}

// --- 3. The Solver (CrystalEngine 2.0 - Integer Edition) ---
pub struct GeodesicSolver {
    pub lattice: [BipartiteTensor; 156],
    pub thermal_load_ppm: u32, 
    pub has_emitted_thermal_warning: bool,
}

impl GeodesicSolver {
    pub fn calculate_impedance(&self, a: usize, b: usize) -> u32 {
        let delta_phase = self.lattice[a].delta_quanta.abs_diff(self.lattice[b].delta_quanta);
        // delta_phase / viscosity, keeping all math in fixed-point.
        ((delta_phase as u64 * PPM_SCALE as u64) / FIELD_VISCOSITY_PPM as u64) as u32
    }

    pub fn execute_finalization_snap(&mut self, id: usize) -> Result<u32, ErrorState> {
        let tensor = self.lattice[id];

        if tensor.delta_quanta < SNAP_ZONE {
            return Ok(tensor.anchor);
        }

        if self.thermal_load_ppm > THERMAL_LIMIT_PPM {
            self.emit_thermal_warning();
            Ok(tensor.anchor)
        } else if self.jitter_incoherence(tensor.delta_quanta) {
            self.purge_node_with_interlock(id);
            Err(ErrorState::IncoherentNoise)
        } else {
            Err(ErrorState::HighLatencyIsotope)
        }
    }

    fn emit_thermal_warning(&mut self) {
        if !self.has_emitted_thermal_warning {
            eprintln!("thermal load exceeded safe operating threshold");
            self.has_emitted_thermal_warning = true;
        }
    }

    fn jitter_incoherence(&self, delta_quanta: u32) -> bool {
        delta_quanta >= SHATTER_ZONE
    }

    fn purge_node_with_interlock(&mut self, id: usize) {
        // u32::MAX represents Absolute Sublimation / Infinity
        self.lattice[id].delta_quanta = u32::MAX;
        self.thermal_load_ppm = self.thermal_load_ppm.saturating_add(THERMAL_SPIKE_PPM);
    }

    pub fn execute_secure_bridge(&mut self, tad: &TAD) -> Result<usize, ErrorState> {
        let mut snap_count = 0;

        if self.thermal_load_ppm > THERMAL_LIMIT_PPM {
            return Err(ErrorState::HighLatencyIsotope);
        }

        let members = &mut self.lattice[tad.range_start..tad.range_end];

        for tensor in members {
            let max_heartbeats = tensor.anchor / 6;

            for _beat in 0..max_heartbeats {
                // Apply TAD Cooling via integer arithmetic
                let reduction = ((tensor.delta_quanta as u64 * tad.resonance_boost_ppm as u64) / PPM_SCALE as u64) as u32;
                tensor.delta_quanta = tensor.delta_quanta.saturating_sub(reduction);

                if tensor.delta_quanta < SNAP_ZONE {
                    tensor.delta_quanta = 0; 
                    snap_count += 1;
                    break;
                }
            }

            if tensor.delta_quanta >= SHATTER_ZONE {
                tensor.anchor = 0;
                tensor.delta_quanta = u32::MAX;
                return Err(ErrorState::IncoherentNoise);
            }
        }

        Ok(snap_count)
    }

}

// Deterministic integer-only noise generator for repeatable benchmarks.
fn generate_noise_quanta(index: usize) -> u32 {
    let mut x = index as u32 ^ 0x55555555;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    // Modulo against an upper limit to simulate field variance
    x % (PI_QUANTA * 2) 
}

const DEFAULT_BENCH_SAMPLES: usize = 100_000;

#[derive(Debug, Clone, Copy)]
struct DeltaBucket {
    label: &'static str,
    min: u32,
    max: u32,
    samples: usize,
    failures: usize,
}

const DELTA_BUCKETS: [(u32, u32, &str); 5] = [
    (0, PI_QUANTA / 12, "stable"),
    (PI_QUANTA / 12, SNAP_ZONE, "snap-zone"),
    (SNAP_ZONE, HOLD_ZONE, "hold-zone"),
    (HOLD_ZONE, SHATTER_ZONE, "high-jitter"),
    (SHATTER_ZONE, u32::MAX, "incoherent"),
];

fn initialize_delta_buckets() -> [DeltaBucket; 5] {
    DELTA_BUCKETS.map(|(min, max, label)| DeltaBucket { label, min, max, samples: 0, failures: 0 })
}

fn bucket_index_for_delta(delta: u32) -> Option<usize> {
    DELTA_BUCKETS.iter().position(|(min, max, _)| delta >= *min && delta < *max)
}

fn benchmark_lcc_integer_path(samples: usize) -> ([DeltaBucket; 5], usize, usize, std::time::Duration) {
    let mut exact_matches = 0;
    let mut failures = 0;
    let mut buckets = initialize_delta_buckets();
    let mut solver = GeodesicSolver {
        lattice: [BipartiteTensor { anchor: 6, delta_quanta: 0 }; 156],
        thermal_load_ppm: 300_000,
        has_emitted_thermal_warning: false,
    };

    let start = Instant::now();
    for index in 0..samples {
        let delta_quanta = black_box(generate_noise_quanta(index));
        solver.lattice[0] = BipartiteTensor { anchor: 6, delta_quanta };

        match black_box(solver.execute_finalization_snap(0)) {
            Ok(anchor) => {
                if anchor == 6 { exact_matches += 1; }
                if let Some(idx) = bucket_index_for_delta(delta_quanta) {
                    buckets[idx].samples += 1;
                }
            }
            Err(_) => {
                failures += 1;
                if let Some(idx) = bucket_index_for_delta(delta_quanta) {
                    buckets[idx].samples += 1;
                    buckets[idx].failures += 1;
                }
            }
        }
    }

    (buckets, exact_matches, failures, start.elapsed())
}

fn parse_samples_from_args() -> usize {
    let mut args = env::args().skip(1);
    let mut samples = DEFAULT_BENCH_SAMPLES;

    while let Some(arg) = args.next() {
        if arg == "--samples" || arg == "-s" {
            if let Some(value) = args.next() {
                match value.parse::<usize>() {
                    Ok(parsed) if parsed > 0 => samples = parsed,
                    _ => {
                        eprintln!("Invalid sample count '{value}', using default {DEFAULT_BENCH_SAMPLES}.");
                    }
                }
            } else {
                eprintln!("Missing value for {arg}, using default {DEFAULT_BENCH_SAMPLES}.");
            }
        } else if arg == "--help" || arg == "-h" {
            println!("Usage: lcc [--samples <positive-integer>]\n       lcc [-s <positive-integer>]");
            std::process::exit(0);
        }
    }

    samples
}

fn run_integer_comparative_test(samples: usize) {
    println!("\n--- SpaceTCO : LCC Strict Integer Execution Test ---");
    let (buckets, exact, failures, elapsed) = benchmark_lcc_integer_path(samples);

    println!(
        "lcc-core-int over {samples} samples: elapsed={:?}, exact_matches={}, failures={}",
        elapsed, exact, failures
    );

    println!("LCC delta distribution (Discrete Quanta):");
    for bucket in &buckets {
        println!(
            "  {:>10} [{}, {}]: samples={}, failures={}",
            bucket.label, bucket.min, bucket.max, bucket.samples, bucket.failures
        );
    }
}

fn main() {
    let samples = parse_samples_from_args();
    run_integer_comparative_test(samples);
}