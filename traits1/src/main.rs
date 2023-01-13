#[allow(dead_code)]
#[allow(unused_variables)]

trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    isim:&'static str
}

struct Cat {
    name:&'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.isim
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

fn main() {
    let h = Human{isim:"Alper"};
    h.talk();
    let c = Cat{name:"Karaca"};
    c.talk();
}
