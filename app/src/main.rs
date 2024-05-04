#[allow(warnings)]
mod bindings;

fn main() {
    let p = bindings::test::plugins::factory::create();
    dbg!(p.name());
}
