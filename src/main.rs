#[macro_use]
extern crate hello_world_derive;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct FrenchToast {
    // #[arg]
    test: String,

    #[arg]
    val: i32,

}

// #[derive(HelloWorld)]
// struct Waffles;

fn main() {
    // FrenchToast::hello_world();
    // Waffles::hello_world();

    let mut a = FrenchToast { test: String::from("berries"), val: 12 };
    println!("--- {}", a.val);
    a.getTest();
}

