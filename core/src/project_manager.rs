use super::store_manager::StoreManger;
use super::errors::BaseEngineError;
use super::project::Project;
use super::persistence::{AddToStore, RetrieveFromStore, UpdateInStore, RemoveFromStore};


use log::{debug, error, info, trace, warn};
pub enum ProjectError {
    
    PersistenceError(BaseEngineError),

    DuplicateError(BaseEngineError),

    NotFoundError(BaseEngineError),

}


pub struct ProjectManager {

    store: StoreManger<Project, String>

}


impl ProjectManager {

    pub fn create_project <SourceConfigType> (&self, project: Project) -> Result<Project, ProjectError> {

        let savedProject =  self.store.retrieve(project.get_id().to_string());

        if let Err(store_error) = savedProject {

            error!("retrieve project store error {:?}", &store_error);
        
            let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
            return Err(ProjectError::PersistenceError(error));
        
        }

        if (savedProject.unwrap().is_some()) {

            let error = BaseEngineError::new(format!("project with Id: {} already exist", project.get_id()), "RBX_CPE_0002".to_string());

            return Err(ProjectError::DuplicateError(error));

        }

        match self.store.add(project) {

            Ok(project) => {

               return Ok(project)

            },

            Err(error) => {

                error!("app project store error {:?}", error);

                let error = BaseEngineError::new(format!("Error occurred while creating project"), "RBX_CPE_0003_F".to_string());

                return Err(ProjectError::PersistenceError(error));

            }

        };


    }


    pub fn retrieve_project (&self, project_id: String) -> Result<Project, ProjectError> {

        let saved_project =  self.store.retrieve(project_id.clone());

        if let Err(store_error) = saved_project {

            let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
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

        self.retrieve_project(project.get_id().into())?;

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

