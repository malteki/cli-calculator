use std::time::Instant;

use anyhow::Context;
use clap::Parser;
use cli_calc::{
    cli::{ Args, Bool },
    file_handler::{
        config::{ load_config_file, write_config_file_pretty },
        storage::{ load_storage_file, write_storage_file_pretty },
        CONFIG_PATH,
        STORAGE_PATH,
    },
};

fn main() -> anyhow::Result<()> {
    let args_time = Instant::now();
    let args = Args::parse();
    let args_time = args_time.elapsed();

    let storage_time_r = Instant::now();
    let mut storage = load_storage_file(STORAGE_PATH).unwrap_or_default();
    let storage_time_r = storage_time_r.elapsed();

    let config_time_r = Instant::now();
    let mut config = load_config_file(CONFIG_PATH).unwrap_or_default();
    let config_time_r = config_time_r.elapsed();

    let total_time = args_time + storage_time_r + config_time_r;

    let action_time = Instant::now();
    match args.action {
        cli_calc::cli::Command::Add { value } => {
            if let Some(active_number) = &storage.active_number {
                if let Some(old) = storage.numbers.get_mut(active_number) {
                    println!("[{active_number}]: {old} + {value} = {}", *old + value);
                    *old += value;
                } else {
                    println!("failed to find active number (no key \"{active_number}\")");
                }
            } else {
                println!("there is no active number (add one with \"new [name] (value)\")");
            }
        }
        cli_calc::cli::Command::Sub { value } => {
            if let Some(active_number) = &storage.active_number {
                if let Some(old) = storage.numbers.get_mut(active_number) {
                    println!("[{active_number}]: {old} - {value} = {}", *old - value);
                    *old -= value;
                } else {
                    println!("failed to find active number (no key \"{active_number}\")");
                }
            } else {
                println!("there is no active number (add one with \"new [name] (value)\")");
            }
        }
        cli_calc::cli::Command::Mul { value } => {
            if let Some(active_number) = &storage.active_number {
                if let Some(old) = storage.numbers.get_mut(active_number) {
                    println!("[{active_number}]: {old} * {value} = {}", *old * value);
                    *old *= value;
                } else {
                    println!("failed to find active number (no key \"{active_number}\")");
                }
            } else {
                println!("there is no active number (add one with \"new [name] (value)\")");
            }
        }
        cli_calc::cli::Command::Div { value } => {
            if let Some(active_number) = &storage.active_number {
                if let Some(old) = storage.numbers.get_mut(active_number) {
                    println!("[{active_number}]: {old} / {value} = {}", *old / value);
                    *old /= value;
                } else {
                    println!("failed to find active number (no key \"{active_number}\")");
                }
            } else {
                println!("there is no active number (add one with \"new [name] (value)\")");
            }
        }
        cli_calc::cli::Command::Set { new_value: new_val } => {
            if let Some(active_number) = &storage.active_number {
                if let Some(old) = storage.numbers.get_mut(active_number) {
                    println!("[{active_number}]: {old} -> {new_val}");
                    *old = new_val;
                } else {
                    println!("failed to find active number (no key \"{active_number}\")");
                }
            } else {
                println!("there is no active number (add one with \"new [name] (value)\")");
            }
        }
        cli_calc::cli::Command::Switch { name } => todo!(),
        cli_calc::cli::Command::New { name, value, overwrite } => {
            if overwrite == Bool::True {
                let overwritten = storage.numbers
                    .insert(name.clone(), value)
                    .map_or("none".to_string(), |old| { format!("{old}") });
                println!("insterted \"{name}\" = {value} (overwritten: {overwritten})");

                storage.active_number = Some(name);
            } else {
                if storage.numbers.contains_key(&name) {
                    println!("this number already exists (try adding overwrite=true)");
                } else {
                    storage.numbers.insert(name.clone(), value);
                    println!("insterted \"{name}\" = {value}");

                    storage.active_number = Some(name);
                }
            }
        }
        cli_calc::cli::Command::List => {
            let mut pairs = storage.numbers.iter().collect::<Vec<_>>();

            pairs.sort_by(|a, b| { a.0.cmp(&b.0) });

            for (key, val) in pairs {
                println!("{key} = {val}");
            }
        }
        cli_calc::cli::Command::PrintTiming { value: Some(new_val) } => {
            config.print_timing = new_val;
            println!("config.print_timing = {}", config.print_timing);
        }
        cli_calc::cli::Command::PrintTiming { value: None } => {
            config.print_timing = !config.print_timing;
            println!("config.print_timing = {}", config.print_timing);
        }
        cli_calc::cli::Command::Delete { name } => todo!(),
        cli_calc::cli::Command::DeleteAll => todo!(),
    }
    let action_time = action_time.elapsed();

    let storage_time_w = Instant::now();
    write_storage_file_pretty(STORAGE_PATH, &storage)?;
    let storage_time_w = storage_time_w.elapsed();

    let config_time_w = Instant::now();
    write_config_file_pretty(CONFIG_PATH, &config)?;
    let config_time_w = config_time_w.elapsed();

    if config.print_timing {
        println!("timing: {total_time:?}");
        println!(" parse args:    {args_time:?}");
        println!(" read storage:  {storage_time_r:?}");
        println!(" read config:   {config_time_r:?}");
        println!(" perf action:   {action_time:?}");
        println!(" write storage: {storage_time_w:?}");
        println!(" write config:  {config_time_w:?}");
    }

    Ok(())
}
