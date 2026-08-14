// use std::io;

fn main() {
    let mut my_string: String = "My String".into();
    my_string = my_string.to_owned() + " " + &my_string;

    let var;
    {
        var = 4096;
        println!("{}", var);
    }
    println!("{}", var);

    println!("{}", my_string);

    let mut var = "Value1";

    let mut data: Vec<&str> = Vec::new();
    data.push(var);
    var = "Value2";
    data.push(var);

    println!("{:?}", data);

    // let data: Data = decode_it(&data);

    let data8 = ('D', 'a', 't', 'a');

    println!("{:?}", data8);

    #[derive(Debug)]
    struct DataStructure {
        _data: (char, char, char, char),
        _manual: (u8, u8, u8, u8),
    }

    let instance = DataStructure {
        _data: data8,
        _manual: (1, 2, 3, 4),
    };

    println!("{:?}", instance);

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Foo {
        Bar,
        Baz(u32, u64),
        Zoo { _flag: bool },
    }

    println!("{:?}", Foo::Bar);
    println!("{:?}", Foo::Baz(1, 2));
    println!("{:?}", Foo::Zoo { _flag: true });
}
