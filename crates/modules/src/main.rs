fn main() {
    let reader = gen::winmd::TypeReader::get();

    for namespace in reader.namespaces() {
        if !namespace.starts_with("Windows.") {
            continue;
        }

        print!("        ");

        for namespace in namespace.split('.') {
            print!("{}::", gen::to_snake(namespace));
        }

        println!("*,");
    }
}
