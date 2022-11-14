use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, FileSpec, Level, Logger, Naming, Record, WriteMode};


pub fn init_logger() -> anyhow::Result<()> {
    Logger::try_with_env_or_str("info")?      // Write all error, warn, and info messages
        .format(log_json_format)
        .log_to_file(
            FileSpec::default().directory("/var/tmp/log/")          // create files in folder ./log_files
                .basename("msvc_rs")
        )
        .write_mode(WriteMode::BufferAndFlush)
        .rotate(                      // If the program runs long enough,
                                      Criterion::Age(Age::Day), // - create a new file every day
                                      Naming::Timestamps,       // - let the rotated files have a timestamp in their name
                                      Cleanup::KeepLogFiles(7), // - keep at most 7 log files
        )
        .append()
        .start()?;
    Ok(())
}

fn log_json_format(
    writer: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = match record.level() {
        Level::Error => "error",
        Level::Warn => "warn",
        Level::Info => "info",
        Level::Debug => "debug",
        Level::Trace => "trace",
    };
    write!(
        writer,
        r#"{{"level":"{}","ts":"{}","logger":"{}","caller":"{}:{}","msg":{:?}}}"#,
        level,
        now.format("%Y-%m-%d %H:%M:%S"),
        record.module_path().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args().to_string()
    )
}

#[allow(dead_code)]
const TIME_FORMAT: &[time::format_description::FormatItem<'static>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory][offset_minute]"
);
// const DEFAULT_TIME_FORMAT: &[time::format_description::FormatItem<'static>] = time::macros::format_description!(
//     "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]"
// );
#[allow(dead_code)]
const DEFAULT_TIME_FORMAT: &[time::format_description::FormatItem<'static>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second]"
);