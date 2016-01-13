extern crate stacktrace;

use stacktrace::StackInfo;

fn layer1() -> StackInfo { StackInfo::new() }
fn layer2() -> StackInfo { layer1() }
fn layer3() -> StackInfo { layer2() }

fn main() {
    println!("{:?}", layer3());
}
