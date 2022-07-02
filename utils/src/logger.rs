use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;


#[derive(Debug)]
pub enum LogInstanceMode {
    Simple,
    Complex,
}

pub fn instance_logger(mode: LogInstanceMode) {


    match mode {

        LogInstanceMode::Simple => {

            init_simple_logger();

        }

        LogInstanceMode::Complex => {

            init_log4rs();

        },


    }

    log::info!("Logger init with {:?}", mode);

}

fn init_simple_logger() {
    
    pretty_env_logger::init();

}


fn init_log4rs() {

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    let console_appender = Appender::builder().build("stdout", Box::new(stdout));

    let root = Root::builder().appender("stdout").build(LevelFilter::Info);

    let log_config = log4rs::config::Config::builder()
        .appender(console_appender)
        .build(root)
        .unwrap();

    log4rs::init_config(log_config).unwrap();
    
}
