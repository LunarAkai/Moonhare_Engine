/// Wrapper around `log` and `fern` crates
/// 
use std::{fmt::Display, io, time::SystemTime};

/// Configures the Log Output Settings
pub fn configere_logger() -> Result<(), fern::InitError>{
    let base_config = fern::Dispatch::new();

    // configure colors for the whole line
    let colors_line = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(fern::colors::Color::TrueColor { r: 85, g: 85, b: 85 })
        .debug(fern::colors::Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(fern::colors::Color::Black);

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("moonhare_engine.log")?);


    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            // special format for debug messages coming from our own crate.
            if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
                out.finish(format_args!(
                    "DEBUG @ {}: {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "{color_line}[{date} {level} {target}] {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = humantime::format_rfc3339_seconds(SystemTime::now()),
                    level = record.level(),
                    target = record.target(),
                    message = message
                ))
            }
        })
        .chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}

pub fn mh_info<T: Display>(arg: T) {
    log::info!("{}", arg);
}

pub fn mh_warn<T: Display>(arg: T) {
    log::warn!("{}", arg);
}

pub fn mh_debug<T: Display>(arg: T) {
    log::debug!("{}", arg);
}

pub fn mh_trace<T: Display>(arg: T) {
    log::trace!("{}", arg);
}

pub fn mh_error<T: Display>(arg: T) {
    log::error!("{}", arg);
}


