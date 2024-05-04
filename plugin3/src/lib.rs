#[allow(warnings)]
mod bindings;

use bindings::exports::test::plugins::factory2::{Guest, GuestPlugin};

struct Component;

struct Plugin;

impl Guest for Component {
    type Plugin = Plugin;

    fn create() -> bindings::exports::test::plugins::factory2::Plugin {
        bindings::exports::test::plugins::factory2::Plugin::new(Plugin{})
    }
}

impl GuestPlugin for Plugin {
    fn name(&self) -> String {
        "plugin3".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
