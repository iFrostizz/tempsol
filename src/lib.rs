use ethers_solc::artifacts::BytecodeObject;
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

/// Create a temp project and return the deployed bytecode of a single contract
pub fn get_deploy_code(contract: String) -> Vec<u8> {
    let (_project, output) = compile_contract(contract);

    let artifacts = output.compiled_artifacts();
    let artifact = artifacts.iter().next().unwrap().1;
    let contract_artifact = &artifact["FSM"];
    if let BytecodeObject::Bytecode(bytecode) = contract_artifact[0]
        .artifact
        .deployed_bytecode
        .clone()
        .unwrap()
        .bytecode
        .unwrap()
        .object
    {
        bytecode.to_vec()
    } else {
        panic!("unliked bytecode not supported");
    }
}
