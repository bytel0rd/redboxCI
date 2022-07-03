#[derive(Debug)]
pub struct Workspace {

    project_id: String,

    last_build_version: usize,

    project_dir: Option<String>

}

impl Workspace {
    

    pub fn new(project_id: String) -> Self {

        Workspace { project_id, last_build_version: 0, project_dir: None }

    }

    pub fn set_project_id(mut self, project_id: String) -> Self {

        self.project_id = project_id;

        return self;

    }

    pub fn get_project_id(&self) -> &String {

        &self.project_id

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