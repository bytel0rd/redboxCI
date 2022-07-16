use std::error::Error;

use crate::errors;

use super::workspace_manager::WorkspaceManager;

use super::project_manager::ProjectManager;
use super::project::Project;
use super::workspace::Workspace;

pub struct BuildOrchestrator<'pm, 'ws> {

    project_manager: &'pm ProjectManager<'pm>,

    workspace_manager: &'ws WorkspaceManager<'ws>

}


impl <'pm, 'ws> BuildOrchestrator<'pm, 'ws> {

    fn clone_project(&self, project: &Project, workspace: &Workspace ) -> Result<(), Box<dyn errors::EngineError>> {

        match  project.get_source_control() {
            
            Some(config) => {

                config.clone_source()?;

                Ok(())
            },

            None => Err(Box::new(errors::BaseEngineError::new("source control not found", "RBX_CBO_001")))

        }

    }

    
    fn execute_pipeline(&self) {
        
    }
    
}


// impl AppEngine {

//     // pub fn new() -> Self {

//     //     AppEngine { }

//     // }

//     pub fn save_configuration(&self) {

//     }

//     pub fn load_configuration(&self) {

//     }

// }

