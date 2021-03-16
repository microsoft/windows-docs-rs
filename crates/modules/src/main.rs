fn main() {
    let reader = gen::TypeReader::get();

    for namespace in reader.namespaces() {
        print!("        ");

        for namespace in namespace.split('.') {
            print!("{}::", gen::to_snake(namespace));
        }

        println!("*,");
    }
}
