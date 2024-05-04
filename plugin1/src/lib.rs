#[allow(warnings)]
mod bindings;

use bindings::exports::test::plugins::factory::{Guest, GuestPlugin};

struct Component;

struct Plugin;

impl Guest for Component {
    type Plugin = Plugin;

    fn create() -> bindings::exports::test::plugins::factory::Plugin {
        bindings::exports::test::plugins::factory::Plugin::new(Plugin{})
    }
}

impl GuestPlugin for Plugin {
    fn name(&self) -> String {
        "plugin1".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
