#![allow(dead_code)]

trait MyInterface {
    fn do_something(&self);
}

// fn f(x: &impl MyInterface) {
//     x.do_something();
// }

fn f(x: &dyn MyInterface) {
    x.do_something();
}
