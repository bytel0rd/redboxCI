use super::store_manager::StoreManger;
use super::errors::BaseEngineError;
use super::project::Project;
use super::persistence::{AddToStore, RetrieveFromStore, UpdateInStore, RemoveFromStore};


use log::{debug, error, trace};

#[derive(Debug)]
pub enum ProjectError {
    
    PersistenceError(BaseEngineError),

    DuplicateError(BaseEngineError),

    NotFoundError(BaseEngineError),

}


pub struct ProjectManager<'store> {

    store: &'store StoreManger<Project, String>

}


impl <'store> ProjectManager<'store> {

    pub fn new(store: &'store StoreManger<Project, String>) -> Self {

        ProjectManager { store }
        
    }

    pub fn create_project (&self, project: Project) -> Result<Project, ProjectError> {

        trace!("creating project {}", project.get_id());

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

               debug!("Successfully created project {:?}", project);

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

        trace!("retrieving project {}", project_id);

        let saved_project =  self.store.retrieve(project_id.clone());

        if let Err(store_error) = saved_project {

            error!("store error: {:?}", store_error);

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

        trace!("updating project {}", project.get_id());

        self.retrieve_project(project.get_id().into())?;

        match self.store.update(project) {

            Ok(project) => {

                debug!("successfully created project {:?}", project);

               return Ok(project)

            },

            Err(error) => {

                error!("update project error {:?}", error);

                let error = BaseEngineError::new(format!("Error occurred while updating project information"), "RBX_CPE_0003_F".to_string());

                return Err(ProjectError::PersistenceError(error));

            }

        };


    }


    pub fn delete_project (&self, project_id: String) -> Result<(), ProjectError> {

        trace!("deleting project {}", project_id);

        match self.store.delete(&project_id) {
            
            Ok(isDeleted) => {

                if (!isDeleted) {

                    let message = format!("unable to delete project: {:?}", project_id);

                    let error = BaseEngineError::new(message.to_string(), "RBX_CPE_0001".to_string());
    
                    return Err(ProjectError::PersistenceError(error));

                }

                Ok(())

            },

            Err(store_error) => {

                let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
                return Err(ProjectError::PersistenceError(error));

            }

        }

      
    }

}


#[cfg(test)]
mod test {

    use crate::project_manager::ProjectError;

    use super::super::persistence::*;
    use super::super::store_manager::*;
    use super::super::project::*;
    use super::ProjectManager;
    
    use log::{info, error};

    #[test]
    fn should_throw_duplicate_project_error() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Project, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Project::new("a".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let project_manager = ProjectManager::new(&store);

        match project_manager.create_project(Project::new("test".to_string())) {
            
            Ok(v) => panic!("{:?}", v),

            Err(err) => {

                if let ProjectError::DuplicateError(error_value) = &err {
                    
                    info!("expected error {:?} ", error_value);

                    return ;
                }

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

    #[test]
    fn should_create_project() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Project, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut create_operator: MockAddToStore<Project, String> = MockAddToStore::new();

        create_operator.expect_add().return_once(|project| Ok(project));

        store.set_add_operator(Box::new(create_operator));

        let project_manager = ProjectManager::new(&store);

        match project_manager.create_project(Project::new("test".to_string())) {
            
            Ok(project) => {

                info!("created project {:?} ", project);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }


    #[test]
    fn should_throw_project_not_found() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Project, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let project_manager = ProjectManager::new(&store);

        match project_manager.retrieve_project("random-project-id".to_string()) {
            
            Ok(v) => panic!("{:?}", v),

            Err(err) => {

                if let ProjectError::NotFoundError(error_value) = &err {
                    
                    info!("expected error {:?} ", error_value);

                    return ;
                }

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

    #[test]
    fn should_retrieve_project() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Project, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Project::new("a".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let project_manager = ProjectManager::new(&store);

        match project_manager.retrieve_project("random-project-id".to_string()) {
            
            Ok(project) => {

                info!("retreved project {:?} ", project);

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }


    #[test]
    fn should_update_project() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Project, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Project::new("name".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut update_operator: MockUpdateInStore<Project, String> = MockUpdateInStore::new();

        update_operator.expect_update().return_once(|project| Ok(project));

        store.set_update_operator(Box::new(update_operator));

        let project_manager = ProjectManager::new(&store);

        match project_manager.update_project(Project::new("test".to_string())) {
            
            Ok(project) => {

                info!("updated project {:?} ", project);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }

    #[test]
    fn should_throw_error_deleting_project() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|key| Ok(false));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let project_manager = ProjectManager::new(&store);

        match project_manager.delete_project("random-project-id".to_string()) {
            
            Ok(_) => {

                panic!("it should have thrown error");

            },

            Err(err) => {

                info!("Expected error received {:?}", err);

            }

        }


    }

    #[test]
    fn should_delete_project() {

        let mut store: StoreManger<Project, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|key| Ok(true));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let project_manager = ProjectManager::new(&store);

        match project_manager.delete_project("random-project-id".to_string()) {
            
            Ok(project) => {

                info!("successfully deleted project");

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

}