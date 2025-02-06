fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let document = String::from("* f");
    // let ast = rust_norg::parse_tree(&document);
    // println!("{:#?}", ast.unwrap());

    let res = norg_lib::search("file:///placeholder", "test");
    println!("{res:?}");

    Ok(())
}
