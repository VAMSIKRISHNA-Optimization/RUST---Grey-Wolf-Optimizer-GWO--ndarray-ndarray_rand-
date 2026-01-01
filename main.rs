#![allow(non_snake_case)]

/// 1. LIBRARIES and DEPENDENCIES
//1.1 Libraries for Matrix Operations
use ndarray::{Array2, ArrayView1};
use ndarray_rand::RandomExt;
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::rand::{SeedableRng, rngs::StdRng};

// // Initialize with a fixed seed
const SEED: u64 = 6789;

// // Basic Intialization parameters
const N: usize = 25; // Np: Population-Size
const T: u32   = 100; // T : Number of iterations

// // Problem Specific paramters
const DIM: usize = 100;   // Number of dimensions
const LB: f64    = -5.12;  // Lower Bound of the Search Space
const UB: f64    = 5.12;   // Upper Bound of the Search Space


fn main() 
{
    // All Testing here
    
    // // Seeding
    let mut rng = StdRng::seed_from_u64(SEED);

    // // Population Initialization of Grey Wolves
    let mut population = population_initialization(N, DIM, LB, UB, &mut rng);

    // // Fitness Evaluation of the Initial Population
    let mut fitness_values = fitness_evaluation_of_initial_population(N, &population, objective_function);

    // // Ranking as per Social Hierarchy
    let mut fitness_values_entires: Vec<_> = fitness_values.indexed_iter()
                                            .map(|(idx, &val)| (idx, val))
                                            .collect();

    fitness_values_entires.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Greater));
    
    // Ranking into Alpha, Beta, Delta
    let mut alpha_fitness = fitness_values_entires[0].1;
    let mut beta_fitness  = fitness_values_entires[1].1;
    let mut delta_fitness = fitness_values_entires[2].1;

    // Assigning Positions to Alpha, Beta, Delta
    let mut alpha_position = population.row(fitness_values_entires[0].0 .0).to_owned();
    let mut beta_position  = population.row(fitness_values_entires[1].0 .0).to_owned();
    let mut delta_position = population.row(fitness_values_entires[2].0 .0).to_owned();

    ////  GWO Loop ////
    let mut Iteration: u32 = 0;

    'GWO_LOOP: loop
    {
        // Linearly Decreasing Inertia Weight
        let a = 2.0 - 2.0 * (Iteration as f64) / (T as f64);

        // Iterate through all the grey wolves of the population
        for pop_mem_ind in 0..population.nrows()
        //for pop_mem_ind in 0..2
        {
            // a. Update parameters: A and C
            let r1: f64 = rng.r#gen();
            let r2: f64 = rng.r#gen();

            let  A1 = 2.0 * a * r1 - a;
            let  C1 = 2.0 * r2;

            let r1: f64 = rng.r#gen();
            let r2: f64 = rng.r#gen();

            let  A2 = 2.0 * a * r1 - a;
            let  C2 = 2.0 * r2;

            let r1: f64 = rng.r#gen();
            let r2: f64 = rng.r#gen();

            let  A3 = 2.0 * a * r1 - a;
            let  C3 = 2.0 * r2;

            // b. Distance Calculations
            let D_alpha = (C1 * &alpha_position - &population.row(pop_mem_ind)).mapv(|x| x.abs());
            let D_beta  = (C2 * &beta_position  - &population.row(pop_mem_ind)).mapv(|x| x.abs());
            let D_delta = (C3 * &delta_position - &population.row(pop_mem_ind)).mapv(|x| x.abs());

            // c. Three-Point Estimation
            let X1 = &alpha_position - A1 * D_alpha;
            let X2 = &beta_position  - A2 * D_beta;
            let X3 = &delta_position - A3 * D_delta;

            // d. Position Averaging
            let new_position = (X1 + X2 + X3) / 3.;
            
            // e. Boundary Check
            let bounded_new_position = new_position.mapv(|x| x.clamp(LB, UB));

            // f. Update Position
            population.row_mut(pop_mem_ind).assign(&bounded_new_position);  

            // g. Evaluate new fitness
            fitness_values[[pop_mem_ind,0]] = fitness_evaluation_of_individual_member(population.row(pop_mem_ind), objective_function);

            // h. Update Alpha, Beta, Delta if needed
            if fitness_values[[pop_mem_ind,0]] < alpha_fitness
            {
                alpha_fitness = fitness_values[[pop_mem_ind,0]];
                alpha_position = population.row(pop_mem_ind).to_owned();            
            }
            else if fitness_values[[pop_mem_ind,0]] < beta_fitness && fitness_values[[pop_mem_ind,0]] >= alpha_fitness
            {
                beta_fitness = fitness_values[[pop_mem_ind,0]];                         
                beta_position = population.row(pop_mem_ind).to_owned();
            }
            else if fitness_values[[pop_mem_ind,0]] < delta_fitness && fitness_values[[pop_mem_ind,0]] >= beta_fitness
            {
                delta_fitness = fitness_values[[pop_mem_ind,0]];                         
                delta_position = population.row(pop_mem_ind).to_owned();
            }


        }


        println!("Iter#:{:?} Alpha: {:?}", Iteration+1, alpha_fitness);
        Iteration += 1;
        if Iteration >= T
        {
            break 'GWO_LOOP;
        }
        
    }

    println!("Final Alpha Fitness: {:?}", alpha_fitness);
    println!("Final Alpha Position: {:?}", alpha_position);

}


fn population_initialization(n: usize, d: usize, x_min: f64, x_max: f64, rng: &mut StdRng) -> Array2<f64>
{
    // Generate an N x D array with values in [x_min, x_max)
    let population: Array2<f64> = Array2::random_using((n , d), Uniform::new(x_min, x_max), rng);
    return population;
}


fn fitness_evaluation_of_initial_population(n: usize, pop: &Array2<f64>, f: fn(ndarray::ArrayView1<f64>)->f64) -> Array2<f64>
{
    let mut vec_evaluated_fitness: Array2<f64> = Array2::zeros((n, 1));
    
    for (ind, row) in pop.rows().into_iter().enumerate() 
    {
        vec_evaluated_fitness[[ind, 0]] = f(row);
    }

    return vec_evaluated_fitness;

}

fn fitness_evaluation_of_individual_member(pop: ndarray::ArrayView1<f64>, f: fn(ndarray::ArrayView1<f64>)->f64) -> f64
{
   return f(pop);
}

fn objective_function(x: ArrayView1<f64>) -> f64
{
    // Rastrigin Function Implementation
    // let a: f64 = 10.0;
    // let d: usize = x.shape()[1];

    // let sum_of_squares = x.mapv(|xi| xi.powi(2) - a * (2.0 * std::f64::consts::PI * xi).cos());
    // let result = a * (d as f64) + sum_of_squares;

    // Sphere Function Implementation
    let result = x.mapv(|xi| xi.powi(2)).sum();

    return result;
}