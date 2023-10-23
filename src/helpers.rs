pub mod js {
    pub fn vanilla_js(project_name: &str) -> String {
        format!("alert(\"Hello {project_name} from JS!\");")
    }

    pub fn vanilla_js_wasm(project_name: &str) -> String {
        format!(
            "
import init, {{hello_wasm}} from \"./{project_name}_wasm.js\";
await init();

alert(hello_wasm(\"{project_name}\"));\n"
        )
    }

    pub fn vanilla_js_wasm_worker_main(project_name: &str) -> String {
        format!(
            "
let worker = new Worker(\"{project_name}_worker.js\");

await new Promise(r => setTimeout(r, 250)); // let wasm load

worker.onmessage = (e) => {{
    alert(e.data)
}}

worker.postMessage(true);
"
        )
    }

    pub fn vanilla_js_wasm_worker_sub(project_name: &str) -> String {
        format!(
            "
importScripts(\"{project_name}_wasm.js\");

// not ordinarily necessary, but for streaming WASM compilation to
// work it needs to be served with a content-type of application/wasm,
// which isn't always the case (eg with php -S), so we remove for now:
delete WebAssembly.instantiateStreaming;

// init the wasm file
wasm_bindgen(\"{project_name}_wasm_bg.wasm\").then((wasm) => {{
  // bing wasm functions into scope
  const {{ hello_wasm }} = wasm_bindgen;

  onmessage = (e) => {{
    postMessage(hello_wasm(\"{project_name} js worker\"));

  }};
}});
"
        )
    }
}

pub mod jsx {
    pub fn wasm_worker_jsx(project_name: &str) -> String {
        format!(
            "
let worker = new Worker(\"{project_name}_worker.js\");

await new Promise(r => setTimeout(r, 250)); // let wasm load

worker.onmessage = (e) => {{
    alert(e.data)
}}

worker.postMessage(true);

function App() {{
    return(
        <h1>Hello {project_name} from react</h1>
    )
}}

ReactDOM.render(<App />, document.querySelector(\"body\"));

        "
        )

    }

    pub fn wasm_jsx(project_name: &str) -> String {
        format!(
            "

import init, {{hello_wasm}} from \"./{project_name}_wasm.js\";
await init();

alert(hello_wasm(\"{project_name}\"));

function App() {{
    return(
        <h1>Hello {project_name} from react</h1>
    )
}}

ReactDOM.render(<App />, document.querySelector(\"body\"));

        "
        )
    }

    pub fn vanilla_jsx(project_name: &str) -> String {
        format!(
            "

function App() {{
    return(
        <h1>Hello {project_name} from react</h1>
    )
}}

ReactDOM.render(<App />, document.querySelector(\"body\"));

        "
        )
    }
}

pub mod babel {
    pub fn scripts(project_name: &str) -> String {
        format!("\n\t\"scripts\": {{\n\t\t\"build\": \"babel {project_name}.jsx -d pkg\"\n\t}},\n")
    }

    pub fn scripts_wasm(project_name: &str) -> String {
        format!(
            "
            \"scripts\": {{
                \"build\": \"babel {project_name}.jsx -d pkg\",
                \"wasm\": \"wasm-pack build --target web --no-typescript --no-pack\"
            }},
            "
        )
    }

    pub fn scripts_wasm_worker(project_name: &str) -> String {
        format!(
            "
            \"scripts\": {{
                \"build\": \"babel {project_name}.jsx -d pkg\",
                \"wasm\": \"wasm-pack build --target no-modules --no-pack --no-typescript\"
            }},
            "
        )
    }

    pub fn config() -> String {
        "{
    \"presets\": [
        [
        \"@babel/preset-env\",
        {
            \"modules\": false
        }
        ],
        [
        \"@babel/preset-react\",
        {
            \"runtime\": \"classic\"
        }
        ]
    ]
    }"
        .to_string()
    }
}

pub mod css {
    pub fn css() -> String {
        "
h1 {
    color: pink;
}
   "
        .to_string()
    }
}

pub mod html {
    pub fn html(project_name: &str) -> String {
        format!(
            "
<!DOCTYPE html>
<html lang=\"en\">

<head>
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\">
    <title>{project_name}</title>
    <link rel=\"stylesheet\" href=\"{project_name}_styles.css\">
</head>

<body>
    <h1>Hello {project_name}</h1>
    <script type=\"module\" src=\"{project_name}.js\"></script>
</body>

</html>
"
        )
    }

    #[allow(unused)]
    pub fn html_react(project_name: &str) -> String {
        format!(
        "
<!DOCTYPE html>
<html lang=\"en\">

<head>
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\">
    <title>{project_name}</title>
    <link rel=\"stylesheet\" href=\"{project_name}_styles.css\">
    <script src=\"https://unpkg.com/react@18/umd/react.production.min.js\" type=\"text/javascript\" crossorigin></script>
    <script src=\"https://unpkg.com/react-dom@18/umd/react-dom.production.min.js\" type=\"text/javascript\" crossorigin></script>
   
</head>

<body>
    <script type=\"module\" src=\"{project_name}.js\"></script>
</body>

</html>
"
    )
    }
}

pub mod rust {
    pub fn wasm() -> String {
        "
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_wasm(name: &str) -> String {
    format!(\"Hello {name} from WASM\")
}
"
        .to_string()
    }

    pub fn toml() -> String {
        "wasm-bindgen = \"0.2\"\n\n[lib]\ncrate-type = [\"cdylib\"]\n".to_string()
    }
}

pub mod commands {

    pub fn wasm_build_command() -> String {
        "build command:\nwasm-pack build --target web --no-typescript --no-pack\n".to_string()
    }

    pub fn wasm_build_command_no_mod() -> String {
        "build command:\nwasm-pack build --target no-modules --no-pack --no-typescript\n".to_string()
    }
}

pub mod readme {
    pub fn react(project_name: &str) -> String {
        format!(
            "Build command using NPM:
        npm run build
        
        Alt:
        babel {project_name}.jsx -d pkg
        "
        )
    }

    pub fn react_wasm(project_name: &str) -> String {
        format!(
            "Build commands using NPM:
        npm run build
        npm run wasm
        
        Alt:
        babel {project_name}.jsx -d pkg
        wasm-pack build --target web --no-typescript --no-pack
        "
        )
    }

    pub fn react_wasm_worker(project_name: &str) -> String {
        format!(
            "Build commands using NPM:
        npm run build
        npm run wasm
        
        Alt:
        babel {project_name}.jsx -d pkg
        wasm-pack build --target no-modules --no-typescript --no-pack
        "
        )
    }
}
