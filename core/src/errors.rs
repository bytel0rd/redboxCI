use core::fmt::Debug;

pub trait EngineError {
    
    fn get_message(&self) -> Box<&String>;

    fn get_error_code(&self) -> Box<&String>;

    // fn set_message(& mut self, message: String);

    // fn set_error_code(& mut self, error_code: String) -> Self;

}



impl Debug for dyn EngineError {    

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        
        f.debug_tuple("EngineError")
         .field(&self.get_message())
         .finish()

    }

}



#[derive(Debug)]
pub struct BaseEngineError {

    message: String,

    error_code: String,

}

impl BaseEngineError {
    
    pub fn new(message: Into<String>, error_code: Into<String>) -> Self {
        
        BaseEngineError { message, error_code}

    }

}


impl EngineError for BaseEngineError {
    
    fn get_message(&self) -> Box<&String> {
        
        Box::new(&self.message)

    }

    fn get_error_code(&self) -> Box<&String> {
        
        Box::new(&self.error_code)

    }
}
