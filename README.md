Grey Wolf Optimizer (GWO) in Rust
A high-performance Rust implementation of the Grey Wolf Optimizer (GWO), a swarm-intelligence metaheuristic inspired by the social hierarchy and hunting mechanisms of grey wolves.This implementation leverages the ndarray ecosystem for efficient vectorization and provides a robust framework for solving continuous optimization problems in high-dimensional spaces.

🐺 Background:
The GWO AlgorithmThe Grey Wolf Optimizer simulates the strict leadership hierarchy and hunting steps of wolf packs:
  Social Hierarchy: The population is divided into four levels: Alpha (\(\alpha \), best solution), Beta (\(\beta \), second-     best), Delta (\(\delta \), third-best), and Omega (\(\omega \), followers).
  Hunting Mechanism: The algorithm mathematically models three main phases:
    Encircling: Wolves update their positions around the estimated location of the prey.
    Hunting: Guided by the \(\alpha ,\beta \), and \(\delta \) wolves, the pack moves toward the optimal region.
    Attacking: Wolves converge on the prey (global optimum) by decreasing an inertia weight factor linearly over time.

🚀 Features
Vectorized Operations: Powered by the ndarray crate for fast, matrix-based updates.
Reproducible Results: Uses a fixed seed with StdRng for deterministic performance testing.
Modular Fitness: Easily swap between objective functions (e.g., Sphere, Rastrigin).
Self-Contained: Implements population initialization, ranking, and position averaging out of the box.

🛠 Prerequisites & InstallationTo use this project in 2026, ensure you have the Rust toolchain installed.
1. Add DependenciesAdd the following to your Cargo.toml:toml[dependencies]
ndarray = "0.16.1"
ndarray-rand = "0.15.0"
rand = "0.8.5"
Use code with caution.
2. Implementation OverviewThe core logic resides in main.rs, where parameters are defined as constants:
   N: Population size (Number of wolves).
   T: Maximum number of iterations.
   DIM: Dimensionality of the search space.
   LB/UB: Lower and Upper boundaries.

📖 UsageRun the optimizer with:bashcargo run --release
Use code with caution.Modifying the Objective FunctionTo optimize a different function, update the objective_function signature:rustfn objective_function(x: ArrayView1<f64>) -> f64 {
    // Implement your function here (e.g., Ackley, Rosenbrock)
    x.mapv(|xi| xi.powi(2)).sum() // Default: Sphere Function
}
Use code with caution.📊 ComplexityThe computational complexity of this GWO implementation is \(O(T\times N\times DIM)\):T: Number of iterations.N: Number of wolves.DIM: Number of dimensions.🛠 Roadmap (Future Enhancements)Support for Non-linear Inertia Weights (Cosine/Gaussian) to improve convergence.Parallel execution using the rayon crate for large populations.Integration with csv for exporting iteration-wise fitness logs.📜 LicenseDistributed under the MIT License. See LICENSE for more information.
