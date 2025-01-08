#[macro_use]
extern crate hello_world_derive;

// trait HelloWorld {
//     fn hello_world();
// }

#[derive(HelloWorld, Hola, Debug)]
struct FrenchToast {
    // #[arg]
    test: String,

    #[arg]
    val: i32,

}

impl FrenchToast {
    fn p(&self) {
        println!("---- p");
    }
}

impl FrenchToast {
    fn t(&self) {
        println!("---- t");
    }
}

// #[derive(HelloWorld)]
// struct Waffles;

fn main() {
    // FrenchToast::hello_world();
    // Waffles::hello_world();

    let mut a = FrenchToast { test: String::from("berries"), val: 12 };
    println!("--- {}", a.val);
    a.getTest();

    let b = FrenchToast::parse();
    println!("--- {:#?}", b);
    b.call();
    b.p();
    b.t();
}

