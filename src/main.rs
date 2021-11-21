mod java;

fn main() {
    let jar = crate::java::Jar::new("./Test.jar").unwrap();

    let vm = crate::java::VirtualMachine::new(jar);

    vm.run();
}
