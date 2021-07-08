fn main() {
    let reader = gen::TypeReader::get();
    let mut namespaces: Vec::<&'static str> = reader.namespaces();
    namespaces.sort();

    for namespace in namespaces {
        print!("        ");

        for namespace in namespace.split('.') {
            print!("{}::", namespace);
        }

        println!("*,");
    }
}
