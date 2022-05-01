pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;
pub mod bruteforce;
pub mod utils;
pub mod round;
pub mod roundmanager;

use std::collections::HashMap;
use std::time;
use env_logger;
use plotters::prelude::*;
use log::{debug, error, trace, info};
use log::LevelFilter;
use crate::utils::pretty_string_poss;
use crate::contestant::ContestantPairs;
use crate::contestant::Players;
use crate::gamemaster::GameMaster;
use crate::gamestrategy::GameStrategy;
use crate::bruteforce::BruteForce;

const OUTPUT_FOLDER: &'static str = "trials/";

fn game(num_players: usize, timeout_rounds: usize) -> u32 {

    let mut game = GameMaster::initialize_game(12, 500);
    

    let mut strategy = BruteForce::initialize(game.contestants());
    info!("-------------------------Correct pairing-------------------------\n{}", ContestantPairs(&game.matches));

    // let time_sleep = time::Duration::from_secs(3);
    // std::thread::sleep(time_sleep);

    while game.in_progress() {
        info!("\\/\\/\\/\\/\\/\\/\\/\\/ROUND {}\\/\\/\\/\\/\\/\\/\\/\\/", game.get_iterations());
        let guess = strategy.ceremony_pairs();
        info!("Ceremony guess --- rounds used: {} -----------------------------------------------", &strategy.round_manager.times_round_used);

        let num_correct = game.ceremony(&guess);
        if num_correct == game.contestants().len() /2 {
            break;
        }

        strategy.ceremony_feedback(num_correct, guess);
        trace!("{}", pretty_string_poss(&strategy.possibilities));
        debug!("Poss remaining: {}, Num correct: {}", strategy.poss_left(), game.num_matched);
        
        let booth_result = game.truth_booth(strategy.send_to_booth());
        strategy.booth_feedback(booth_result);
    }
    game.output_stats();
    strategy.output_stats();
    return game.get_iterations() as u32;
}

fn generate_normal_game_distribution(num_trials: usize) -> Result<(), Box<dyn std::error::Error>>{
    let mut trial = 0;
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();
    while trial < num_trials {
        trial += 1;
        let num_rounds = game(12, 500);

        if frequency_map.contains_key(&num_rounds) {
            let count = frequency_map.get_mut(&num_rounds).unwrap();
            *count += 1;
        } else {
            frequency_map.insert(num_rounds, 1);
        }
    }
    let output_location = OUTPUT_FOLDER.to_owned() + "normal_game.png";
    let root = BitMapBackend::new(&output_location, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35 as u32)
        .y_label_area_size(40 as u32)
        .margin(5 as u32)
        .caption("Histogram Test", ("sans-serif", 50.0 as f32))
        .build_cartesian_2d((0u32..25u32).into_segmented(), 0u32..250u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15 as i32))
        .draw()?;

    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
    ];

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

fn main() {
    // env_logger::init();
    simple_logging::log_to_file("test.log", LevelFilter::Debug);
    generate_normal_game_distribution(1000);
}
