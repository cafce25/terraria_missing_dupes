use std::{error::Error, path::PathBuf};

use clap::Parser;
use terra_plr::{Player, inventory::Item};

use args::Args;
use research_item::ResearchItem;

mod args;
mod research_item;

const VERSION: i32 = *terra_plr::SUPPORTED_VERSIONS.end(); // assume the latest version
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    let name_len = args.player_name.trim_end_matches(".plr").len();
    args.player_name.truncate(name_len);

    let player_dir = get_player_dir();
    let mut player_file = [
        player_dir.join(format!("{}.plr", args.player_name)), // within default directory
        PathBuf::from(format!("{}.plr", args.player_name)),   // try as full path as well
    ]
    .into_iter()
    .map(std::fs::File::open)
    .find_map(Result::ok)
    .ok_or("could not find player")?;
    let player = Player::read_player(&mut player_file)?;

    let mut research_items =
        csv::Reader::from_reader(include_bytes!("../terraria_dupe_items.csv").as_slice());

    let mut total_research_items = 0;
    let mut unresearched_items = 0;

    for item in research_items.deserialize() {
        let Ok(research_item): Result<ResearchItem, _> = item else {
            continue;
        };
        total_research_items += 1;

        let item = Item::from_id(research_item.id, VERSION).unwrap();
        let item_name = format!("{item:?}");

        let count = player
            .item_researched_count
            .get(&item)
            .copied()
            .unwrap_or(0);
        let fully_researched = count >= research_item.amount.as_i32();
        unresearched_items += usize::from(!fully_researched);
        if (fully_researched && args.complete) || (!fully_researched && args.missing) {
            println!(
                "{:47} ({}, {:47}) {:3}/{:3} {:80}",
                research_item.name,
                research_item.id,
                item_name,
                count,
                research_item.amount,
                research_item.url.trim()
            );
        }
    }
    println!("Total missing: {unresearched_items}/{total_research_items}");

    Ok(())
}

#[cfg(target_os = "linux")]
fn get_player_dir() -> PathBuf {
    let data_dir = if let Ok(xdg_data_dir) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg_data_dir)
    } else {
        std::env::home_dir()
            .unwrap_or_default()
            .join(".local")
            .join("share")
    };
    data_dir.join("Terraria").join("Players")
}

#[cfg(target_os = "windows")]
fn get_player_dir() -> PathBuf {
    let data_dir = PathBuf::from(std::env::vars("USERPROFILE").unwrap_or_default())
        .join("Documents")
        .join("My Games");
    data_dir.join("Terraria").join("Players")
}
