use core::fmt::Debug;

use super::errors;


pub trait SourceControlCFG {
    
    fn get_source_control_name(&self) -> String;

    fn clone_source(&self) -> Result<(), Box<dyn errors::EngineError>>;

    fn get_pipeline(&self) -> ();

}

impl Debug for dyn SourceControlCFG {    

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        
        f.debug_tuple("SourceControlCFG")
         .field(&self.get_source_control_name())
         .finish()


    }

}
