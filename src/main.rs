use rbx_utils::logger;
use redboxCI::pipeline::*;
use serde::Deserialize;


fn main() {
    
    logger::instance_logger(logger::LogInstanceMode::Simple);

    let mut pipeline_str = std::fs::read_to_string(std::path::Path::new("docs/pipeline.yaml")).unwrap();


    for document in serde_yaml::Deserializer::from_str(&pipeline_str) {
    
        let repo_pipeline = RepoPipeLine::deserialize(document).unwrap();
    
        println!("{:?}", repo_pipeline);
    
    }
    
    // let repo_pipeline: Vec<RepoPipeLine> = serde_yaml::from_str(&pipeline_str).unwrap();
    // println!("{:?}", repo_pipeline);


}
