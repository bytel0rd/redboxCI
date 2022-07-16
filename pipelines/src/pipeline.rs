use std::{collections::HashMap, rc::Rc};

use crate::errors;

pub struct Project {

    pub id: String,

    pub name: String,

    pub workspace_dir: Option<String>,

    pub configuration: HashMap<String, String>,

    pub environment_variables: HashMap<String, String>,

    pub commands: Vec<String>,
}


pub trait RBXPlugin {

    fn run(&self, project: Project);

}


pub struct PipeExecutorManager {

    executors: HashMap<String, Rc<Box<dyn RBXPlugin>>>

}

impl PipeExecutorManager {
    
    fn add_executor<T: Into<String>>(&mut self, name: T, executor: Box<dyn RBXPlugin>) -> Result<(), Box<dyn errors::EngineError>> {
        
        self.executors.insert(name.into(), Rc::new(executor));

        Ok(())

    }

    fn get_executor<T: Into<String>>(&self, name: T) -> Result<Rc<Box<dyn RBXPlugin>>, Box<dyn errors::EngineError>> {

        match self.executors.get(&name.into()) {

            Some(executor) => Ok(executor.clone()),

            None => Err(Box::new(errors::BaseEngineError::new("Executor not found", "RBX_CPL_001")))

        }

    }

}