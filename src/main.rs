pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;
pub mod bruteforce;
pub mod utils;
pub mod round;
pub mod roundmanager;

use std::collections::HashMap;
use plotters::prelude::*;
use log::{debug, trace, info};
use log::LevelFilter;
use crate::utils::pretty_string_poss;
use crate::contestant::ContestantPairs;
use crate::contestant::Players;
use crate::gamemaster::GameMaster;
use crate::gamestrategy::GameStrategy;
use crate::bruteforce::BruteForce;

const OUTPUT_FOLDER: &'static str = "trials/";

fn game(num_pairs: i32, timeout_rounds: usize) -> u32 {

    let mut game = GameMaster::initialize_game(num_pairs, timeout_rounds as i32);

    let mut strategy = BruteForce::initialize(game.contestants());
    info!("-------------------------Correct pairing-------------------------\n{}", ContestantPairs(&game.matches));
    while game.in_progress() {
        
        info!("\\/\\/\\/\\/\\/\\/\\/\\/ROUND {}\\/\\/\\/\\/\\/\\/\\/\\/", game.get_iterations());
        let guess = strategy.ceremony_pairs();
        info!("Ceremony guess --- rounds used: {} -----------------------------------------------", &strategy.round_manager.times_round_used);

        let num_correct = game.ceremony(&guess);
        if !game.in_progress() {
            break;
        }

        strategy.ceremony_feedback(num_correct, guess);
        trace!("{}", pretty_string_poss(&strategy.possibilities));
        info!("Poss remaining: {}, Num correct: {}", strategy.poss_left(), game.num_matched);

        let booth_result = game.truth_booth(strategy.send_to_booth());
        strategy.booth_feedback(booth_result);
    }
    game.output_stats();
    strategy.output_stats();
    return game.get_iterations() as u32;
}

fn generate_normal_game_distribution(num_trials: usize, file_name: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut trial = 0;
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();
    while trial < num_trials {
        trial += 1;
        let num_rounds = game(8, 500);

        if frequency_map.contains_key(&num_rounds) {
            let count = frequency_map.get_mut(&num_rounds).unwrap();
            *count += 1;
        } else {
            frequency_map.insert(num_rounds, 1);
        }
    }
    let output_location = OUTPUT_FOLDER.to_owned() + file_name;
    let root = BitMapBackend::new(&output_location, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35 as u32)
        .y_label_area_size(40 as u32)
        .margin(5 as u32)
        .caption("Are You The One Distribution of Rounds To Win", ("sans-serif", 25 as f32))
        .build_cartesian_2d((0u32..25u32).into_segmented(), 0u32..1000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Rounds to win")
        .axis_desc_style(("sans-serif", 15 as i32))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(frequency_map.iter().map(|(rounds, count)| (*rounds, *count))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", output_location);

    Ok(())
}

fn generate_running_time_plot(max_pairs: i32, step_size: usize, iterations: usize, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut results: Vec<(u32, u32)> = vec![]; // instead is (contestant size, 
    
    for _i in 0..iterations {
        for num_pairs in (4..max_pairs).step_by(step_size) {
            let num_rounds = game(num_pairs, 500);
            results.push((2 * num_pairs as u32, num_rounds));
        }
    }
        

    let output_location = OUTPUT_FOLDER.to_owned() + file_name;
    let root = BitMapBackend::new(&output_location, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40i32)
        .y_label_area_size(40i32)
        .build_cartesian_2d(0f64..200f64, 0f64..300f64)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Rounds to win")
        .x_desc("Number of Contestants")
        .axis_desc_style(("sans-serif", 15 as i32))
        .draw()?;
    
    scatter_ctx.draw_series(results.iter().map(|(x, y)| Circle::new((*x as f64, *y as f64), 2 as f32, RED.filled())))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", output_location);

    Ok(())
}

fn main() {
    // simple_logging::log_to_file("test.log", LevelFilter::Trace);

    generate_normal_game_distribution(2000, "normal_game_bruteforce_round_optimization.png");
    generate_running_time_plot(100, 2, 4, "running_time_bruteforce_round_optimization.png");
}
