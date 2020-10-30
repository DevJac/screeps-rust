use stdweb::js;

struct JsLog;
struct JsNotify;

impl log::Log for JsLog {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        let message = format!("{}", record.args());
        js! {
            console.log(@{message});
        }
    }
    fn flush(&self) {}
}

impl log::Log for JsNotify {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        let message = format!("{}", record.args());
        js! {
            Game.notify(@{message}, 30);
        }
    }
    fn flush(&self) {}
}

pub fn setup_logging(verbosity: log::LevelFilter) {
    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| {
            out.finish(format_args!("{:>5}: {}", record.level(), message))
        })
        .chain(Box::new(JsLog) as Box<dyn log::Log>)
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Warn)
                .format(|out, message, record| {
                    let time = screeps::game::time();
                    out.finish(format_args!("[{}]{:>5}: {}", time, record.level(), message))
                })
                .chain(Box::new(JsNotify) as Box<dyn log::Log>),
        )
        .apply()
        .expect("expect logging::init() to be called only once")
}
