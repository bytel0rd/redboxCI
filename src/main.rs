use rbx_utils::logger;

fn main() {
    
    logger::instance_logger(logger::LogInstanceMode::Simple);

    println!("Hello, world!");

}
