cargo_component_bindings::generate!();
use bindings::exports::org::foo::api;

struct Component;

impl api::Guest for Component {
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
