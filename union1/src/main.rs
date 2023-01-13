#[allow(dead_code)]
#[allow(unused_variables)]

union IntOrFloat {
    i:i32,
    f:f32
}

fn process_value(iof:IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat{i:44} => {
                println!("degerimiz 44 dur");
            }
            IntOrFloat{f} => {
                println!("2. durumu boyle ifade ettik deger: {}", f);
            }
        }
    }
}

fn main() {
    let mut iof = IntOrFloat{i:122};
    iof.i = 221;
    let value:i32 = unsafe{iof.i};
    println!("Value: {}", value);

    let iof2 = IntOrFloat{f:55.0};
    process_value(iof2);

    let iof3 = IntOrFloat{i:44};
    process_value(iof3);

    let iof4 = IntOrFloat{i:1};
    process_value(iof4);
}
