#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();
    server.get("/", middleware! {|_, response|
                                let mut a = HashMap::new();
                                let acdc = "Hello Lorenzo";
                                a.insert("ab", acdc.len());
                                return response.render("assets/index.tpl", &a);
    });
    server.get("/:name", middleware! {|request, response|
                                      let mut b = HashMap::new();
                                      let bcbc = request.param("name").unwrap();
                                      if bcbc == "lorenzo" {
                                          b.insert("name", "Creator");
                                      } else {
                                          b.insert("name", bcbc);
                                      }
                                      return response.render("assets/lang.tpl", &b);
    });
    server.listen("127.0.0.1:6767");
}
