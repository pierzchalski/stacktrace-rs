#[macro_use] extern crate stacktrace;

trace!();

fn layer1() -> Trace<String> { Trace::new("a message".into()) }
fn layer2() -> Trace<String> { layer1() }
fn layer3() -> Trace<String> { layer2() }

fn main() {
    println!("{:?}", layer3());
}
