use super::workspace_manager::WorkspaceManager;

use super::project_manager::ProjectManager;

pub struct AppEngine<'pm, 'ws> {

    project_manager: &'pm ProjectManager<'pm>,

    workspace_manager: &'ws WorkspaceManager<'ws>

}


// impl AppEngine {

//     // pub fn new() -> Self {

//     //     AppEngine { }

//     // }

//     pub fn save_configuration(&self) {

//     }

//     pub fn load_configuration(&self) {

//     }

// }

