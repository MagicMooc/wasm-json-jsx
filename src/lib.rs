extern crate wasm_bindgen;

use serde_json::Value;
use wasm_bindgen::prelude::*;

const __SPACE__: char = ' ';
const __LEFT_BOUND__: char = '<';
const __RIGHT_BOUND__: char = '>';
const __ENTER__: char = '\n';
const __SPLITTER__: char = '/';
const __BUILTIN_PROPS__: [&str; 5] = [
    "children",
    "innerText",
    "componentName",
    "style",
    "currentId",
];

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
// macro_rules! console_warn {
//     ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen]
    fn encodeURI(s: &str) -> String;
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    console_log!("Hello, {}!", name);
}

#[wasm_bindgen]
pub fn strlen(k: &str) -> i32 {
    k.len() as i32
}

pub fn build_props(props: &serde_json::Map<String, Value>) -> String {
    let mut res = String::new();
    for (key, value) in props {
        if !__BUILTIN_PROPS__.contains(&&**key) {
            res += &format!(" {}={}", encodeURI(key), value.to_string());
        } else if key.eq("style") {
            res += &format!(" style = {{");
            res += &format!("{{");
            for (k, v) in value.as_object().unwrap() {
                res += &format!(" {}={}", encodeURI(k), v.to_string());
            }
            res += &format!("}}");
            res += &format!("}}");
        }
    }
    res
}

pub fn get_jsx_by_jsx_tree(
    component_name: &str,
    props: &serde_json::Map<String, Value>,
    indent: usize,
    children: Option<&Value>,
) -> Option<String> {
    let mut jsx = if component_name.eq("root") {
        format!("")
    } else {
        format!(
            "{}{}{}{}{}{}",
            __SPACE__.to_string().repeat(indent),
            __LEFT_BOUND__,
            component_name,
            build_props(props),
            __RIGHT_BOUND__,
            __ENTER__
        )
    };
    if let Some(children_some) = children {
        if let Some(children) = children_some.as_array() {
            children.iter().for_each(|item| {
                jsx += &*get_jsx_by_jsx_tree(
                    item.get("componentName").unwrap().as_str().unwrap(),
                    item.as_object().unwrap(),
                    indent + 2,
                    item.get("children"),
                )
                .unwrap();
            })
        }
    } else if let Some(text) = props.get("innerText") {
        jsx += text.as_str()?;
    }
    if component_name.eq("root") {
        Some(format!("{}", jsx))
    } else {
        Some(format!(
            "{}{}{}{}{}{}{}",
            jsx,
            __SPACE__.to_string().repeat(indent),
            __LEFT_BOUND__,
            __SPLITTER__,
            component_name,
            __RIGHT_BOUND__,
            __ENTER__,
        ))
    }
}

#[wasm_bindgen]
pub fn json_to_jsx(json: &str) -> Option<String> {
    let json: Value = serde_json::from_str(json).ok()?;
    get_jsx_by_jsx_tree(
        json.get("componentName")?.as_str()?,
        json.as_object()?,
        0,
        json.get("children"),
    )
}
