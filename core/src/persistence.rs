use crate::errors::EngineError;


pub trait PermanentStore<DataType, PrimaryKeyType> {

    fn create(&self, data: DataType ) -> Result<DataType, Box<dyn EngineError>>;


    fn retrieve(&self, id: PrimaryKeyType ) -> Result<Option<DataType>, Box<dyn EngineError>>;


    fn update(&self, update: DataType ) -> Result<DataType, Box<dyn EngineError>>;

    
    fn delete(&self, update: &DataType ) -> Result<bool, Box<dyn EngineError>>;


}