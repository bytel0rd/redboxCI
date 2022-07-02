pub struct ProjectWorkspace {

    id: String,

    project_id: String,

    last_build_version: usize,

    project_dir: Option<String>

}

impl ProjectWorkspace {
    

    pub fn new(id: String, project_id: String) -> Self {

        ProjectWorkspace { id, project_id, last_build_version: 0, project_dir: None }

    }


    pub fn set_id(mut self, id: String) -> Self {

        self.id = id;

        return self;

    }

    pub fn get_id(&self) -> String {    

        return self.id.clone();

    }

    pub fn set_project_id(mut self, project_id: String) -> Self {

        self.id = project_id;

        return self;

    }

    pub fn get_project_id(&self) -> String {

        return self.project_id.clone();

    }

    pub fn set_latest_build_version(mut self, version: usize) -> Self {

        self.last_build_version = version;

        return self;

    }

    pub fn get_latest_build_version(&self) -> usize {

        return self.last_build_version;

    }

    pub fn set_base_dir(mut self, dir: Option<String>) -> Self {

        self.project_dir = dir;

        return self;

    }

    pub fn get_base_dir(&self) -> &Option<String> {

        return &self.project_dir;

    }

}