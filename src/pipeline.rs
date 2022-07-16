use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPipeLine {

    job: String,

    version: usize,

    global: Option<Global>,

    pipelines: Vec<PipeLine>

}

pub type EnvironmentVariables = std::collections::HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Global {

    environment_variables: Option<EnvironmentVariables>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipeLine {

    name: String,

    status: String,

    environment_variables: Option<EnvironmentVariables>,

    steps: Vec<PipeLineStep>


}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipeLineStep {

    name: String,

    executor: String,

    description: Option<String>,

    environment_variables: Option<EnvironmentVariables>,

    commands: Vec<String>

}

