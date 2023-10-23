use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[allow(special_module_name)]
mod helpers;

#[derive(Clone, Debug)]
enum ProjectTypes {
    VanillaJs,
    VanillajsWasm,
    VanillaJsWamnWorker,
    React,
    ReactWasm,
    ReactWasmWorker,
}

fn main() {
    let project_name = get_string("Project Name");
    //let project_name = "new_project".to_string();

    println!("{:?}", project_name);
    let project_promt = "Select Project Type:
    \t1) vanilla js
    \t2) vanilla js with wasm
    \t3) vanilla js with wasm worker
    \t4) react
    \t5) react with wasm
    \t6) react with wasm worker";

    let project_type: ProjectTypes;
    loop {
        let project_type_str = get_string(project_promt);
        //println!("{project_type_str}");
        project_type = match project_type_str.as_str() {
            "1" => ProjectTypes::VanillaJs,
            "2" => ProjectTypes::VanillajsWasm,
            "3" => ProjectTypes::VanillaJsWamnWorker,
            "4" => ProjectTypes::React,
            "5" => ProjectTypes::ReactWasm,
            "6" => ProjectTypes::ReactWasmWorker,
            _ => continue,
        };
        break;
    }

    //project_type = ProjectTypes::VanillaJs;
    let init_val = project_type.init(project_name.as_str());
    if let Err(e) = init_val {
        println!("{:?}", format!("Error: {e}"));
    }
}

fn get_string(prompt: &str) -> String {
    let mut line = String::new();
    println!("\n{prompt}\n");
    let _ = std::io::stdout();
    match std::io::stdin().read_line(&mut line) {
        Ok(_ok) => {
            let line = line.trim().to_string();
            if line.len() > 0 {
                line
            } else {
                get_string(prompt)
            }
        }
        Err(_err) => get_string(prompt),
    }
}

impl ProjectTypes {
    fn init(self, project_name: &str) -> Result<(), String> {
        println!("starting initiation of project: {}", project_name);
        match self {
            ProjectTypes::VanillaJs => self.init_vanilla_js(project_name)?,
            ProjectTypes::VanillajsWasm => self.init_vanilla_js_wasm(project_name)?,
            ProjectTypes::VanillaJsWamnWorker => self.init_vanilla_js_wasm_worker(project_name)?,
            ProjectTypes::React => self.init_react(project_name)?,
            ProjectTypes::ReactWasm => self.init_react_wasm(project_name)?,
            ProjectTypes::ReactWasmWorker => self.init_react_wasm_worker(project_name)?,
        }
        Ok(())
    }

    fn init_react_wasm_worker(self, project_name: &str) -> Result<(), String> {

        mkdir(project_name)?;
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html_react(project_name))?;
        println!("Generating CSS.");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JSX.");
        gen_jsx(project_name, helpers::jsx::wasm_worker_jsx(project_name))?;
        println!("Generating JS Worker");
        gen_js_worker(
            project_name,
            helpers::js::vanilla_js_wasm_worker_sub(project_name),
        )?;
        println!("Installing babel.");
        let babel_cmd = Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("install")
            .arg("@babel/cli")
            .arg("@babel/core")
            .arg("@babel/node")
            .arg("@babel/preset-env")
            .arg("@babel/preset-react")
            .output();
        match babel_cmd {
            Ok(_) => (),
            Err(_) => return Err("NPM failed to download babel".to_string()),
        };

        gen_babel_config(project_name, helpers::babel::config())?;
        mod_npm_package(project_name, helpers::babel::scripts_wasm_worker(project_name))?;
        println!("Transpiling JSX to JS");
        Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("run")
            .arg("build")
            .output()
            .unwrap();

        println!("Generating Rust lib.");
        gen_rust_project(project_name)?;
        println!("Compiling Rust Lib.");
        let cmd_res = Command::new("wasm-pack")
            .current_dir(format!("./{project_name}"))
            .arg("build")
            .arg("--target")
            .arg("no-modules")
            .arg("--no-typescript")
            .arg("--no-pack")
            .output();
        match cmd_res {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        println!("Generating readme.");
        gen_readme(project_name, helpers::readme::react_wasm_worker(project_name))?;
        println!("Done!");
        Ok(())
    }

    fn init_react_wasm(self, project_name: &str) -> Result<(), String> {
        mkdir(project_name)?;
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html_react(project_name))?;
        println!("Generating CSS.");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JSX.");
        gen_jsx(project_name, helpers::jsx::wasm_jsx(project_name))?;
        println!("Installing babel.");
        let babel_cmd = Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("install")
            .arg("@babel/cli")
            .arg("@babel/core")
            .arg("@babel/node")
            .arg("@babel/preset-env")
            .arg("@babel/preset-react")
            .output();
        match babel_cmd {
            Ok(_) => (),
            Err(_) => return Err("NPM failed to download babel".to_string()),
        };

        gen_babel_config(project_name, helpers::babel::config())?;
        mod_npm_package(project_name, helpers::babel::scripts_wasm(project_name))?;
        println!("Transpiling JSX to JS");
        Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("run")
            .arg("build")
            .output()
            .unwrap();

        println!("Generating Rust lib.");
        gen_rust_project(project_name)?;
        println!("Compiling lib");
        let cmd_res = Command::new("wasm-pack")
            .current_dir(format!("./{project_name}"))
            .arg("build")
            .arg("--target")
            .arg("web")
            .arg("--no-typescript")
            .arg("--no-pack")
            .output();

        match cmd_res {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        println!("Generating readme.");
        gen_readme(project_name, helpers::readme::react_wasm(project_name))?;
        println!("Done!");
        Ok(())
    }

    fn init_react(self, project_name: &str) -> Result<(), String> {
        mkdir(project_name)?;
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html_react(project_name))?;
        println!("Generating CSS.");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JSX.");
        gen_jsx(project_name, helpers::jsx::vanilla_jsx(project_name))?;
        println!("Installing babel.");
        let babel_cmd = Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("install")
            .arg("@babel/cli")
            .arg("@babel/core")
            .arg("@babel/node")
            .arg("@babel/preset-env")
            .arg("@babel/preset-react")
            .output();
        match babel_cmd {
            Ok(_) => (),
            Err(_) => return Err("NPM failed to download babel".to_string()),
        };

        gen_babel_config(project_name, helpers::babel::config())?;
        mod_npm_package(project_name, helpers::babel::scripts(project_name))?;
        println!("Transpiling JSX to JS");
        Command::new("npm")
            .current_dir(format!("./{project_name}"))
            .arg("run")
            .arg("build")
            .output()
            .unwrap();
        println!("Generating readme.");
        gen_readme(project_name, helpers::readme::react(project_name))?;
        println!("Done!");
        Ok(())
    }

    fn init_vanilla_js_wasm_worker(self, project_name: &str) -> Result<(), String> {
        mkdir(project_name)?; // project directory
                              // package directory
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html(project_name))?;
        println!("Generating CSS.");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JS.");
        gen_js(
            project_name,
            helpers::js::vanilla_js_wasm_worker_main(project_name),
        )?;
        println!("Generating JS Worker");
        gen_js_worker(
            project_name,
            helpers::js::vanilla_js_wasm_worker_sub(project_name),
        )?;
        println!("Generating Rust lib.");
        gen_rust_project(project_name)?;
        println!("Compiling Rust Lib.");
        let cmd_res = Command::new("wasm-pack")
            .current_dir(format!("./{project_name}"))
            .arg("build")
            .arg("--target")
            .arg("no-modules")
            .arg("--no-typescript")
            .arg("--no-pack")
            .output();
        match cmd_res {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        println!("Generating readme.");
        gen_readme(project_name, helpers::commands::wasm_build_command_no_mod())?;
        println!("Done!");
        Ok(())
    }

    fn init_vanilla_js_wasm(self, project_name: &str) -> Result<(), String> {
        mkdir(project_name)?; // project directory

        // package directory
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html(project_name))?;
        println!("Generating CSS");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JS.");
        gen_js(project_name, helpers::js::vanilla_js_wasm(project_name))?;
        println!("Generating Rust lib.");
        gen_rust_project(project_name)?;
        println!("Compiling lib");
        let cmd_res = Command::new("wasm-pack")
            .current_dir(format!("./{project_name}"))
            .arg("build")
            .arg("--target")
            .arg("web")
            .arg("--no-typescript")
            .arg("--no-pack")
            .output();

        match cmd_res {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        println!("Generating readme.");
        gen_readme(project_name, helpers::commands::wasm_build_command())?;
        println!("Done!");
        Ok(())
    }

    fn init_vanilla_js(self, project_name: &str) -> Result<(), String> {
        mkdir(project_name)?; // project directory

        // package directory
        let pkg_path = format!("{}/pkg", project_name);
        mkdir(pkg_path.as_str())?;
        println!("Generating HTML.");
        gen_html(project_name, helpers::html::html(project_name))?;
        println!("Generating CSS");
        gen_css(project_name, helpers::css::css())?;
        println!("Generating JS.");
        gen_js(project_name, helpers::js::vanilla_js(project_name))?;
        println!("Done!");
        Ok(())
    }
}

fn gen_babel_config(project_name: &str, config: String) -> Result<(), String> {
    let path = format!("./{project_name}/babel.config.json");
    let mut file = match File::create(path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create babel config file".to_string()),
    };
    match file.write_all(config.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write to babel config".to_string()),
    }
}

fn gen_readme(project_name: &str, readme: String) -> Result<(), String> {
    let readme_path = format!("./{project_name}/readme.txt");
    let mut readme_file = match File::create(readme_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to make readme".to_string()),
    };
    match readme_file.write_all(readme.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to write to readme".to_string()),
    }
}

fn gen_rust_project(project_name: &str) -> Result<(), String> {
    // create rust lib
    match Command::new("cargo")
        .arg("init")
        .arg(format!("./{project_name}"))
        .arg("--name")
        .arg(format!("{project_name}_wasm").as_str())
        .arg("--lib")
        .output()
    {
        Ok(_) => (),
        Err(_) => return Err("Failed to make rust lib.".to_string()),
    };

    // modify toml file
    let toml_path = format!("./{project_name}/Cargo.toml");
    let mut toml_file = match std::fs::OpenOptions::new().append(true).open(toml_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to open toml file.".to_string()),
    };
    match toml_file.write_all(helpers::rust::toml().as_bytes()) {
        Ok(_) => (),
        Err(m) => return Err(m.to_string()),
    };

    // make new lib
    let lib_path = format!("./{project_name}/src/lib.rs");
    let mut lib_file = match File::create(lib_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create rust lib.".to_string()),
    };
    match lib_file.write_all(helpers::rust::wasm().as_bytes()) {
        Ok(()) => (),
        Err(_) => return Err("Failed to write to rust lib".to_string()),
    };

    Ok(())
}

fn mod_npm_package(project_name: &str, scripts: String) -> Result<(), String> {
    let path = format!("./{project_name}/package.json");
    let mut file = File::open(&path).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    let mut new_package = String::new();
    for (i, line) in file_string.lines().enumerate() {
        new_package.push_str(line);
        if i == 0 {
            new_package.push_str(&scripts);
        }
    }

    let mut file = File::create(path).unwrap();
    file.write_all(new_package.as_bytes()).unwrap();
    Ok(())
}

fn gen_js_worker(project_name: &str, js: String) -> Result<(), String> {
    let pkg_path = format!("{}/pkg", project_name);
    let js_path = format!("{pkg_path}/{project_name}_worker.js");
    let mut js_file = match File::create(js_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create css file.".to_string()),
    };
    match js_file.write_all(js.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to write to css.".to_string()),
    }
}

fn gen_js(project_name: &str, js: String) -> Result<(), String> {
    let pkg_path = format!("{}/pkg", project_name);
    let js_path = format!("{pkg_path}/{project_name}.js");
    let mut js_file = match File::create(js_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create css file.".to_string()),
    };
    match js_file.write_all(js.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to write to css.".to_string()),
    }
}

fn gen_jsx(project_name: &str, jsx: String) -> Result<(), String> {
    let jsx_path = format!("./{project_name}/{project_name}.jsx");
    let mut jsx_file = match File::create(jsx_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create jsx file.".to_string()),
    };
    match jsx_file.write_all(jsx.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write jsx file.".to_string()),
    }
}

fn gen_css(project_name: &str, css: String) -> Result<(), String> {
    let pkg_path = format!("{}/pkg", project_name);
    let css_path = format!("{pkg_path}/{project_name}_styles.css");
    let mut css_file = match File::create(css_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create css file.".to_string()),
    };
    match css_file.write_all(css.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to write to css.".to_string()),
    }
}

fn gen_html(project_name: &str, html: String) -> Result<(), String> {
    let pkg_path = format!("{}/pkg", project_name);

    // add html
    let html_path = format!("{pkg_path}/{project_name}.html");
    let mut html_file = match File::create(html_path) {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to create html file.".to_string()),
    };
    match html_file.write_all(html.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to write to html.".to_string()),
    }
}

fn mkdir(name: &str) -> Result<(), String> {
    if Path::new(name).exists() {
        return Err(format!("Path already exists {name}."));
    }

    match Command::new("mkdir").arg("-p").arg(name).output() {
        Ok(_) => Ok(()),
        Err(_) => Err("Faild to create directory.".to_string()),
    }
}
