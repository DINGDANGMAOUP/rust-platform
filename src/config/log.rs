use crate::config::config::SystemConfig;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{DateType, KeepType, Packer, Rolling, RollingType};
use rbatis::rbdc::DateTime;
use std::time::Duration;

pub fn init_log(config: &SystemConfig) {
    //init fast log
    let mut cfg = Config::new().level(parse_log_level(&config.logging.level));
    cfg = cfg.file_split(
        &config.logging.dir,
        Rolling::new(parse_rolling_type(
            &config.logging.rolling.as_str(),
            &config,
        )),
        parse_keep_type(&config.logging.keep_type),
        parse_packer(&config.logging.pack_compress),
    );
    if config.app.debug {
        cfg = cfg.console();
    }
    cfg = cfg.chan_len(Some(config.logging.chan_len));
    let _ = fast_log::init(cfg);
    if config.app.debug == false {
        println!("[app] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn parse_rolling_type(log_rolling: &str, config: &SystemConfig) -> RollingType {
    let lower = log_rolling.to_lowercase();
    let rolling_type;
    if log_rolling.ends_with("B") {
        rolling_type = RollingType::BySize(parse_log_size(&config.logging.rolling));
    } else if lower.as_str().ends_with("minute")
        || lower.as_str().ends_with("hour")
        || lower.as_str().ends_with("day")
    {
        match lower.as_str() {
            "minute" => {
                rolling_type = RollingType::ByDate(DateType::Minute);
            }
            "hour" => {
                rolling_type = RollingType::ByDate(DateType::Hour);
            }
            "day" => {
                rolling_type = RollingType::ByDate(DateType::Day);
            }
            _ => {
                if lower.ends_with("minute") {
                    let value: u64 = lower
                        .trim_end_matches("minute")
                        .parse()
                        .expect("parse number fail");
                    rolling_type = RollingType::ByDuration((
                        DateTime::now().0,
                        Duration::from_secs(value * 60),
                    ));
                } else if lower.ends_with("hour") {
                    let value: u64 = lower
                        .trim_end_matches("hour")
                        .parse()
                        .expect("parse number fail");
                    rolling_type = RollingType::ByDuration((
                        DateTime::now().0,
                        Duration::from_secs(value * 60 * 60),
                    ));
                } else if lower.ends_with("day") {
                    let value: u64 = lower
                        .trim_end_matches("day")
                        .parse()
                        .expect("parse number fail");
                    rolling_type = RollingType::ByDuration((
                        DateTime::now().0,
                        Duration::from_secs(value * 24 * 60 * 60),
                    ));
                } else {
                    panic!("unknown log_rolling '{}'", log_rolling);
                }
            }
        }
    } else {
        panic!("unknown log_rolling '{}'", log_rolling);
    }
    return rolling_type;
}

fn parse_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
        _ => Box::new(fast_log::plugin::packer::LogPacker {}),
    }
}

fn parse_log_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn parse_keep_type(arg: &str) -> KeepType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            return KeepType::KeepNum(num.parse::<i64>().unwrap());
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            return KeepType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()));
        }
        arg if arg.to_uppercase().as_str() == "ALL" => {
            return KeepType::All;
        }
        _ => {
            panic!("unknown keep_type '{}'", arg)
        }
    }
}

fn parse_log_level(arg: &str) -> log::LevelFilter {
    return match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };
}

mod test {

    #[test]
    fn test_config() {
        use fast_log::Config;
        use log::Log;

        fast_log::init(Config::new().file("logs/test.log").chan_len(Some(100000))).unwrap();
        log::info!("Commencing yak shaving{}", 0);
        fast_log::logger().flush();
    }
}
