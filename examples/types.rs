extern crate pest;
extern crate ron;
extern crate skr;

use ron::ser::{to_string_pretty, PrettyConfig};
use skr::namespace::*;
use skr::parse::populate_namespace;

fn main() {
    let pretty: PrettyConfig = PrettyConfig {
        separate_tuple_members: true,
        ..PrettyConfig::default()
    };

    let doc = std::fs::read_to_string("examples/types.skr").expect("cannot read file");
    let mut namespace = Namespace::new("testing");
    let res = populate_namespace(&doc, &mut namespace);
    match res {
        Ok(_) => println!("{}", to_string_pretty(&namespace, pretty.clone()).unwrap()),
        Err(e) => println!("{:?}", e),
    }
}
