fn main() {
    let reader = windows_winmd::TypeReader::get();

    for namespace in reader.namespaces() {
        if !namespace.starts_with("Windows.") {
            continue;
        }

        print!("        ");

        for namespace in namespace.split('.') {
            print!("{}::", windows_gen::to_snake(namespace));
        }

        println!("*");
    }
}
