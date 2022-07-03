use super::store_manager::StoreManger;
use super::errors::BaseEngineError;
use super::workspace::Workspace;
use super::persistence::{AddToStore, RetrieveFromStore, UpdateInStore, RemoveFromStore};


use log::{debug, error, trace};

#[derive(Debug)]
pub enum WorkspaceError {
    
    PersistenceError(BaseEngineError),

    DuplicateError(BaseEngineError),

    NotFoundError(BaseEngineError),

}


pub struct WorkspaceManager<'store> {

    store: &'store StoreManger<Workspace, String>

}


impl <'store> WorkspaceManager<'store> {

    pub fn new(store: &'store StoreManger<Workspace, String>) -> Self {

        WorkspaceManager { store }
        
    }

    pub fn create_Workspace (&self, workspace: Workspace) -> Result<Workspace, WorkspaceError> {

        trace!("creating Workspace {}", workspace.get_project_id());

        let savedWorkspace =  self.store.retrieve(workspace.get_project_id().to_string());

        if let Err(store_error) = savedWorkspace {

            error!("retrieve Workspace store error {:?}", &store_error);
        
            let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
            return Err(WorkspaceError::PersistenceError(error));
        
        }

        if (savedWorkspace.unwrap().is_some()) {

            let error = BaseEngineError::new(format!("Workspace with Id: {} already exist", workspace.get_project_id()), "RBX_CPE_0002".to_string());

            return Err(WorkspaceError::DuplicateError(error));

        }

        match self.store.add(workspace) {

            Ok(workspace) => {

               debug!("Successfully created Workspace {:?}", workspace);

               return Ok(workspace)

            },

            Err(error) => {

                error!("app Workspace store error {:?}", error);

                let error = BaseEngineError::new(format!("Error occurred while creating Workspace"), "RBX_CPE_0003_F".to_string());

                return Err(WorkspaceError::PersistenceError(error));

            }

        };


    }


    pub fn retrieve_Workspace (&self, workspace_id: String) -> Result<Workspace, WorkspaceError> {

        trace!("retrieving Workspace {}", workspace_id);

        let saved_workspace =  self.store.retrieve(workspace_id.clone());

        if let Err(store_error) = saved_workspace {

            error!("store error: {:?}", store_error);

            let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
            return Err(WorkspaceError::PersistenceError(error));

        }

        let possible_workspace = saved_workspace.unwrap();

        if possible_workspace.is_none() {

            let error = BaseEngineError::new(format!("Workspace with Id: {} not found", workspace_id), "RBX_CPE_0002".to_string());

            return Err(WorkspaceError::NotFoundError(error));

        }

        Ok(possible_workspace.unwrap())
        


    }

    pub fn update_Workspace (&self, workspace: Workspace) -> Result<Workspace, WorkspaceError> {

        trace!("updating Workspace {}", workspace.get_project_id());

        self.retrieve_Workspace(workspace.get_project_id().into())?;

        match self.store.update(workspace) {

            Ok(workspace) => {

                debug!("successfully created Workspace {:?}", workspace);

               return Ok(workspace)

            },

            Err(error) => {

                error!("update Workspace error {:?}", error);

                let error = BaseEngineError::new(format!("Error occurred while updating Workspace information"), "RBX_CPE_0003_F".to_string());

                return Err(WorkspaceError::PersistenceError(error));

            }

        };


    }


    pub fn delete_Workspace (&self, workspace_id: String) -> Result<(), WorkspaceError> {

        trace!("deleting Workspace {}", workspace_id);

        match self.store.delete(&workspace_id) {
            
            Ok(isDeleted) => {

                if (!isDeleted) {

                    let message = format!("unable to delete Workspace: {:?}", workspace_id);

                    let error = BaseEngineError::new(message.to_string(), "RBX_CPE_0001".to_string());
    
                    return Err(WorkspaceError::PersistenceError(error));

                }

                Ok(())

            },

            Err(store_error) => {

                let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
                return Err(WorkspaceError::PersistenceError(error));

            }

        }

      
    }

}


#[cfg(test)]
mod test {

    use super::super::persistence::*;
    use super::super::store_manager::*;
    use super::super::workspace::*;
    use super::*;
    
    use log::{info, error};

    #[test]
    fn should_throw_duplicate_Workspace_error() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Workspace::new("project_id".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.create_Workspace(Workspace::new("test".to_string())) {
            
            Ok(v) => panic!("{:?}", v),

            Err(err) => {

                if let WorkspaceError::DuplicateError(error_value) = &err {
                    
                    info!("expected error {:?} ", error_value);

                    return ;
                }

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

    #[test]
    fn should_create_Workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut create_operator: MockAddToStore<Workspace, String> = MockAddToStore::new();

        create_operator.expect_add().return_once(|Workspace| Ok(Workspace));

        store.set_add_operator(Box::new(create_operator));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.create_Workspace(Workspace::new("test".to_string())) {
            
            Ok(Workspace) => {

                info!("created Workspace {:?} ", Workspace);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }


    #[test]
    fn should_throw_Workspace_not_found() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.retrieve_Workspace("random-Workspace-id".to_string()) {
            
            Ok(v) => panic!("{:?}", v),

            Err(err) => {

                if let WorkspaceError::NotFoundError(error_value) = &err {
                    
                    info!("expected error {:?} ", error_value);

                    return ;
                }

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

    #[test]
    fn should_retrieve_Workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Workspace::new("a".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.retrieve_Workspace("random-Workspace-id".to_string()) {
            
            Ok(Workspace) => {

                info!("retreved Workspace {:?} ", Workspace);

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }


    #[test]
    fn should_update_Workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|key| Ok(Some(Workspace::new("name".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut update_operator: MockUpdateInStore<Workspace, String> = MockUpdateInStore::new();

        update_operator.expect_update().return_once(|Workspace| Ok(Workspace));

        store.set_update_operator(Box::new(update_operator));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.update_Workspace(Workspace::new("test".to_string())) {
            
            Ok(Workspace) => {

                info!("updated Workspace {:?} ", Workspace);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }

    #[test]
    fn should_throw_error_deleting_Workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|key| Ok(false));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.delete_Workspace("random-Workspace-id".to_string()) {
            
            Ok(_) => {

                panic!("it should have thrown error");

            },

            Err(err) => {

                info!("Expected error received {:?}", err);

            }

        }


    }

    #[test]
    fn should_delete_Workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|key| Ok(true));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let Workspace_manager = WorkspaceManager::new(&store);

        match Workspace_manager.delete_Workspace("random-Workspace-id".to_string()) {
            
            Ok(Workspace) => {

                info!("successfully deleted Workspace");

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

}