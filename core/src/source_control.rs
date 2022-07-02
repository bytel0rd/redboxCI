use std::error::Error;
use core::fmt::Debug;


pub trait SourceControlCFG {
    
    fn get_source_control_name(&self) -> String;

    fn clone(&self) -> Result<(), Box<dyn Error>>;

    fn get_pipeline(&self) -> ();

}

impl Debug for dyn SourceControlCFG {    

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        
        f.debug_tuple("SourceControlCFG")
         .field(&self.get_source_control_name())
         .finish()


    }

}
