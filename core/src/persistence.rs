use crate::errors::EngineError;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
pub trait AddToStore<DataType, PrimaryKeyType> {

    fn update(&self, update: DataType ) -> Result<DataType, Box<dyn EngineError>>;    

}

#[cfg_attr(test, automock)]
pub trait UpdateInStore<DataType, PrimaryKeyType> {

    fn update(&self, update: DataType ) -> Result<DataType, Box<dyn EngineError>>;    

}

#[cfg_attr(test, automock)]
pub trait RetrieveFromStore<DataType, PrimaryKeyType> {

    fn retrieve(&self, id: PrimaryKeyType ) -> Result<Option<DataType>, Box<dyn EngineError>>;
    
}

#[cfg_attr(test, automock)]
pub trait RemoveFromStore<PrimaryKeyType> {
    
    fn delete(&self, key: &PrimaryKeyType ) -> Result<bool, Box<dyn EngineError>>;


}