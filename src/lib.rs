use ethers_solc::{project_util::TempProject, ConfigurableArtifacts, ProjectCompileOutput};

#[derive(Clone)]
pub enum ProjectFile {
    Contract(String, String),
    Library(String, String),
}

/// Similar to [`compile_project`], but this helper only allows one contract source
pub fn compile_contract(
    contract: String,
) -> (TempProject<ConfigurableArtifacts>, ProjectCompileOutput) {
    let files = vec![ProjectFile::Contract(String::from("A"), contract)];
    compile_project(files)
}

/// Create a temporary folder on your filesystem including a dapptools project with the provided contracts and libraries
pub fn compile_project(
    files: Vec<ProjectFile>,
) -> (TempProject<ConfigurableArtifacts>, ProjectCompileOutput) {
    let project = TempProject::<ConfigurableArtifacts>::dapptools().unwrap();

    files.iter().for_each(|f| match f {
        ProjectFile::Contract(name, content) => {
            project.add_source(name, content).unwrap();
        }
        ProjectFile::Library(name, content) => {
            project.add_lib(name, content).unwrap();
        }
    });
    let compiled = project.compile().unwrap();

    if compiled.has_compiler_errors() {
        compiled.output().errors.iter().for_each(|err| {
            println!("{:#?}", err.message);
        });
        panic!("Please fix compiler errors first");
    }

    (project, compiled)
}
