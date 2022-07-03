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

    pub fn create_workspace (&self, workspace: Workspace) -> Result<Workspace, WorkspaceError> {

        trace!("creating Workspace {}", workspace.get_project_id());

        let saved_workspace =  self.store.retrieve(workspace.get_project_id().to_string());

        if let Err(store_error) = saved_workspace {

            error!("retrieve Workspace store error {:?}", &store_error);
        
            let error = BaseEngineError::new(store_error.get_message().to_string(), "RBX_CPE_0001".to_string());
    
            return Err(WorkspaceError::PersistenceError(error));
        
        }

        if saved_workspace.unwrap().is_some() {

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


    pub fn retrieve_workspace (&self, workspace_id: String) -> Result<Workspace, WorkspaceError> {

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

    pub fn update_workspace (&self, workspace: Workspace) -> Result<Workspace, WorkspaceError> {

        trace!("updating Workspace {}", workspace.get_project_id());

        self.retrieve_workspace(workspace.get_project_id().into())?;

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


    pub fn delete_workspace (&self, workspace_id: String) -> Result<(), WorkspaceError> {

        trace!("deleting Workspace {}", workspace_id);

        match self.store.delete(&workspace_id) {
            
            Ok(is_deleted) => {

                if !is_deleted {

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
    
    use log::{info};

    #[test]
    fn should_throw_duplicate_workspace_error() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|_| Ok(Some(Workspace::new("project_id".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.create_workspace(Workspace::new("test".to_string())) {
            
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
    fn should_create_workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|_| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut create_operator: MockAddToStore<Workspace, String> = MockAddToStore::new();

        create_operator.expect_add().return_once(|workspace| Ok(workspace));

        store.set_add_operator(Box::new(create_operator));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.create_workspace(Workspace::new("test".to_string())) {
            
            Ok(workspace) => {

                info!("created Workspace {:?} ", workspace);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }


    #[test]
    fn should_throw_workspace_not_found() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|_| Ok(None));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.retrieve_workspace("random-Workspace-id".to_string()) {
            
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
    fn should_retrieve_workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|_| Ok(Some(Workspace::new("a".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.retrieve_workspace("random-Workspace-id".to_string()) {
            
            Ok(workspace) => {

                info!("retreved Workspace {:?} ", workspace);

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }


    #[test]
    fn should_update_workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut retrieve_operator_mock: MockRetrieveFromStore<Workspace, String> = MockRetrieveFromStore::new();

        retrieve_operator_mock.expect_retrieve().return_once(|_| Ok(Some(Workspace::new("name".to_string()))));

        store.set_retrieve_operator(Box::new(retrieve_operator_mock));

        let mut update_operator: MockUpdateInStore<Workspace, String> = MockUpdateInStore::new();

        update_operator.expect_update().return_once(|workspace| Ok(workspace));

        store.set_update_operator(Box::new(update_operator));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.update_workspace(Workspace::new("test".to_string())) {
            
            Ok(workspace) => {

                info!("updated Workspace {:?} ", workspace);

            },

            Err(err) => {

                panic!("unexpected error: {:?}", err);

            }

        }


    }

    #[test]
    fn should_throw_error_deleting_workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|_| Ok(false));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.delete_workspace("random-Workspace-id".to_string()) {
            
            Ok(_) => {

                panic!("it should have thrown error");

            },

            Err(err) => {

                info!("Expected error received {:?}", err);

            }

        }


    }

    #[test]
    fn should_delete_workspace() {

        let mut store: StoreManger<Workspace, String> = StoreManger::new();

        let mut delete_operator_mock: MockRemoveFromStore<String> = MockRemoveFromStore::new();

        delete_operator_mock.expect_delete().return_once(|_| Ok(true));

        store.set_delete_operator(Box::new(delete_operator_mock));

        let workspace_manager = WorkspaceManager::new(&store);

        match workspace_manager.delete_workspace("random-Workspace-id".to_string()) {
            
            Ok(_) => {

                info!("successfully deleted Workspace");

            },

            Err(err) => {

                panic!("unexpected error type provided: {:?}", err);

            }

        }


    }

}