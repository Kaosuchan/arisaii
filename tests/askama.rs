use askama::Template;

#[derive(Template)]
#[template(path = "test1.txt")]
struct Aaa<'aaa> {
    name: &'aaa str,
}

#[test]
fn askama_test1() {
    println!("{}", Aaa { name: "aaa" }.render().unwrap());
}
