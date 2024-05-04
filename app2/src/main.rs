#[allow(warnings)]
mod bindings;

fn main() {
    let p = bindings::test::plugins::factory::create();
    dbg!(p.name());
    let p2 = bindings::test::plugins::factory2::create();
    dbg!(p2.name());
}
