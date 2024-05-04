#[allow(warnings)]
mod bindings;

use bindings::exports::test::plugins::factory2::Guest;
use bindings::exports::test::plugins::plugin_i::{Guest as IGuest, GuestPlugin};

struct Component;

struct Plugin;

impl IGuest for Component {
    type Plugin = Plugin;
}

impl Guest for Component {
    fn create() -> bindings::exports::test::plugins::factory2::Plugin {
        bindings::exports::test::plugins::factory2::Plugin::new(Plugin{})
    }
}

impl GuestPlugin for Plugin {
    fn name(&self) -> String {
        "plugin1".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
