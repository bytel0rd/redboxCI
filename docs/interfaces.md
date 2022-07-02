# Table
[ class | struct ] Project {

    name: String
    
    sourceInformation: <T extend SourceControlCFG>
    
    pipeline: []Strings

    id String

}


interface SourceControlCFG<AuthType> {
    
    authentication: unknown | interface{} | AuthType

}


#[Table]
[class | strict] BaseWorkspace(project Project) {

    baseDirectory() -> String;

    buildVersions() -> WorkspaceBuild[];

    lastBuildVersion() -> int

    id String

    projectId String

}

#[Table]
[struct | class] WorkspaceBuild(version, directory) {

    version String

    directory String

    build_log String

    isSuccessful boolean

    startTime DateTime

    endTime DateTime

    baseWorkspaceId String

}


[ class | struct ] CloneHistory {

    error Error<String>

    isSuccessful boolean

    startTime DateTime

    endTime DateTime

    workspacePath String

}

[interface | trait] WorkspaceProcessor(project Project, baseWorkspace BaseWorkspace ) {

    generateBuildPath() -> String // {branch}_{build_number} || {build_tag_time}

    clone(baseDirectory, version, sourceControlInformation) -> CloneHistory

    build(project, cloneHistory) -> WorkspaceBuild

    deploy() -> unknown

}

interface SourceClient {

    clone(directoryPath: string) -> { isComplete, Error, path  }

}

Core Engine
- Create & Manage Projects
- Trigger Pipelines
- Deploy Pipelines