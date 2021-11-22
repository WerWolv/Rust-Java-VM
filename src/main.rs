mod java;

fn main() {
    let main_jar = crate::java::Jar::new("./Test.jar").unwrap();
    let java_base_jar  = crate::java::Jar::new("./java.base.jar").unwrap();

    let mut vm = crate::java::VirtualMachine::new(main_jar).unwrap();
    vm.add_library_jar(java_base_jar);

    vm.run();
}
