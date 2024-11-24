use super::runner;
use crate::{internal_types::Args, PlayerInfo};

pub fn send_scores_and_end_game(scores: Vec<(&PlayerInfo, i32)>) -> ! {
    let scores_str = scores
        .iter()
        .map(|(player, score)| format!("{} {}\n", player.name(), score))
        .collect::<Vec<String>>()
        .join("");

    let response = runner::send_command("SCORES", Args::new(), &scores_str);

    assert!(
        response.0 == crate::types::Status::Ok,
        "Failed to send scores"
    );

    runner::send_command("END", Args::new(), "");

    // TODO log
    loop {}
}
