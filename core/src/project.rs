use std::rc::Rc;

use uuid::Uuid;

use crate::source_control::SourceControlCFG;

#[derive(Debug)]
pub struct Project {

    name: String,

    source_control: Option<Box<dyn SourceControlCFG>>,

    pipeline: Vec<String>,

    id: String

}


impl Project {
    
    pub fn new(name: String ) -> Self {

        Project { 
            name, 
            source_control: None, 
            pipeline: Vec::new(), 
            id: Uuid::new_v4().to_string()
        }

    }

    pub fn get_name(&self) -> &String {

       &self.name

    }

    pub fn set_name(mut self, name: String) -> Self {

        self.name = name;

        self

    }

    pub fn set_source_control(mut self, source_control_cfg: Option<Box<dyn SourceControlCFG>>) -> Self {

        self.source_control = source_control_cfg;

        return self;

    }

    pub fn get_source_control(&self) -> Option<&Box<dyn SourceControlCFG>> {

        return self.source_control.as_ref();

    }

    pub fn set_id(mut self, uuid: String) -> Self {

        self.id = uuid;

        return self;

    }

    pub fn get_id(&self) -> &String {

        return &self.id;

    }

    
    pub fn set_pipelines(mut self, pipelines: Vec<String>) -> Self {

        self.pipeline = pipelines;

        return self;
        
    }

    pub fn get_pipelines(&self) -> &Vec<String> {

        return &self.pipeline;

    }

}

