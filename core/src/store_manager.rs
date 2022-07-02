use std::ops::Add;

use super::persistence::{AddToStore, UpdateInStore, RetrieveFromStore, RemoveFromStore};
use super::errors;


pub struct StoreManger<DataType, PrimaryKeyType> {

    add: Option<Box<dyn AddToStore<DataType, PrimaryKeyType>>>,

    retrieve: Option<Box<dyn RetrieveFromStore<DataType, PrimaryKeyType>>>,

    delete: Option<Box<dyn RemoveFromStore<PrimaryKeyType>>>,

    update: Option<Box<dyn UpdateInStore<DataType, PrimaryKeyType>>>,

}

impl <DataType, PrimaryKeyType> StoreManger<DataType, PrimaryKeyType> {
    
    pub  fn new() -> Self {

        StoreManger { add: None, retrieve: None, delete: None, update: None }

    }

    pub fn set_add_operator(&mut self, operator: Box<dyn AddToStore<DataType, PrimaryKeyType>>) {

        self.add = Some(operator);

    }

    
    pub fn set_retrieve_operator(&mut self, operator: Box<dyn RetrieveFromStore<DataType, PrimaryKeyType>>) {

        self.retrieve = Some(operator);

    }

    pub fn set_update_operator(&mut self, operator: Box<dyn UpdateInStore<DataType, PrimaryKeyType>>) {

        self.update = Some(operator);

    }

    pub fn set_delete_operator(&mut self, operator: Box<dyn RemoveFromStore<PrimaryKeyType>>) {

        self.delete = Some(operator);

    }
    

}

impl  <DataType, PrimaryKeyType> AddToStore<DataType, PrimaryKeyType> for StoreManger<DataType, PrimaryKeyType> {
    
    fn add(&self,update:DataType) -> Result<DataType, Box<dyn errors::EngineError>> {
        
        match self.add {
            
            None => {
                
               Err(errors::BaseEngineError::new("add functionality not configured for data type on store manager", "RBX_PSM_001"))

            },

            Some(operator) => operator.add(update)

        }


    }
    

}

impl  <DataType, PrimaryKeyType> UpdateInStore<DataType, PrimaryKeyType> for StoreManger<DataType, PrimaryKeyType> {
    
    fn update(&self,update:DataType) -> Result<DataType, Box<dyn errors::EngineError>> {
        
        match self.update {
            
            None => {
                
               Err(errors::BaseEngineError::new("update functionality not configured for data type on store manager", "RBX_PSM_002"))

            },

            Some(operator) => operator.update(update)

        }


    }
    

}


impl  <DataType, PrimaryKeyType> RetrieveFromStore<DataType, PrimaryKeyType> for StoreManger<DataType, PrimaryKeyType> {

    fn retrieve(&self,id:PrimaryKeyType) -> Result<Option<DataType>,Box<dyn errors::EngineError>> {
        
        match self.retrieve {
            
            None => {
                
               Err(errors::BaseEngineError::new("retrieve by primary key not configured for data type on store manager", "RBX_PSM_003"))

            },

            Some(operator) => operator.retrieve(id)

        }

    }
    
    

}

impl  <DataType, PrimaryKeyType> RemoveFromStore<PrimaryKeyType> for StoreManger<DataType, PrimaryKeyType> {
    

    fn delete(&self,key: &PrimaryKeyType) -> Result<bool,Box<dyn errors::EngineError>> {
        
        match self.delete {
            
            None => {
                
               Err(errors::BaseEngineError::new("delete functionality not configured for data type on store manager", "RBX_PSM_004"))

            },

            Some(operator) => operator.delete(key)

        }

    }
    

}