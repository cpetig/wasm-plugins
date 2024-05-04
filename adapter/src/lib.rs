#[allow(warnings)]
mod bindings;

use bindings::exports::test::plugins::factory2::{Guest, GuestPlugin};

struct Component;

struct Plugin(bindings::test::plugins::factory::Plugin);

impl Guest for Component {
    type Plugin = Plugin;

    fn create() -> bindings::exports::test::plugins::factory2::Plugin {
        bindings::exports::test::plugins::factory2::Plugin::new(Plugin(
            bindings::test::plugins::factory::create(),
        ))
    }
}

impl GuestPlugin for Plugin {
    fn name(&self) -> String {
        self.0.name()
    }
}

bindings::export!(Component with_types_in bindings);
