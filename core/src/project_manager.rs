use crate::{project::Project, persistence::PermanentStore, errors::BaseEngineError};
use log::{debug, error, info, trace, warn};


pub enum ProjectError {
    
    PersistenceError(BaseEngineError),

    DuplicateError(BaseEngineError),

    NotFoundError(BaseEngineError),

}


pub struct ProjectManager {

    store: Box<dyn PermanentStore<Project, String>>

}


impl ProjectManager {

    pub fn create_project <SourceConfigType> (&self, project: Project) -> Result<Project, ProjectError> {

        let savedProject =  self.store.retrieve(project.get_id());

        if (savedProject.is_err()) {

            let error = BaseEngineError::new(savedProject.unwrap_err().get_message().to_string(), "RBX_CPE_0001".to_string());

            return Err(ProjectError::PersistenceError(error));

        }

        if (savedProject.unwrap().is_some()) {

            let error = BaseEngineError::new(format!("project with Id: {} already exist", project.get_id()), "RBX_CPE_0002".to_string());

            return Err(ProjectError::DuplicateError(error));

        }

        match self.store.create(project) {

            Ok(project) => {

               return Ok(project)

            },

            Err(error) => {

                let error = BaseEngineError::new(format!("Error occurred while creating project"), "RBX_CPE_0003_F".to_string());

                return Err(ProjectError::PersistenceError(error));

            }

        };


    }


    pub fn retrieve_project (&self, project_id: String) -> Result<Project, ProjectError> {

        let saved_project =  self.store.retrieve(project_id.clone());

        if (saved_project.is_err()) {

            let error = BaseEngineError::new(saved_project.unwrap_err().get_message().to_string(), "RBX_CPE_0001".to_string());

            return Err(ProjectError::PersistenceError(error));

        }

        let possible_project = saved_project.unwrap();

        if (possible_project.is_none()) {

            let error = BaseEngineError::new(format!("project with Id: {} not found", project_id), "RBX_CPE_0002".to_string());

            return Err(ProjectError::NotFoundError(error));

        }

        Ok(possible_project.unwrap())
        


    }

    pub fn update_project (&self, project: Project) -> Result<Project, ProjectError> {

        self.retrieve_project(project.get_id())?;

        match self.store.update(project) {

            Ok(project) => {

               return Ok(project)

            },

            Err(error) => {

                // error!

                let error = BaseEngineError::new(format!("Error occurred while updating project information"), "RBX_CPE_0003_F".to_string());

                return Err(ProjectError::PersistenceError(error));

            }

        };


    }

}