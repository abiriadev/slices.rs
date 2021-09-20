fn main() {
    {
        let _hello = "_hello";
    }

    {
        let str = String::from("hello");

        let str2 = str;

        // println!("{}", str);
        println!("{:?}", str2);

        let str3 = str2.clone();

        println!("{:?}", str2);
        println!("{:?}", str3);
    }

    let string = some_function();

    let len = get_length(&string);

    println!("the length of {} is {}", string, len);

    let mut string_m = string;

    add_world(&mut string_m);

    println!("{:?}", string_m);

    {
        let mut a = String::from("ARGU");

        let a_ref = &a;
        let a_ref2 = &a;

        println!("{} {} {}", a, a_ref, a_ref2);

        // let a_refm = &mut a;
        //
        // a_refm.push_str("jakjson");
        //
        // println!("{}", a_ref);
    }

    let d = dead_ref();

    let mut lorem = String::from("Itineris tramitems cadunt in noster divio!");

    let f_len = first_word(&lorem[..]);

    println!("{}", f_len);

    lorem.clear();

    // println!("{}", f_len);

    {
        let a = [1, 2, 3, 4, 5];

        let sl = &a[1..3];

        println!("{:?}", sl);

        for element in sl {
            println!("{}", element);
        }
    }
}

fn first_word(str: &str) -> &str {
    for (i, item) in str.as_bytes().iter().enumerate() {
        if *item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}

fn dead_ref() -> String {
    let dd = String::from("hello I will dead soon!");

    dd
}

fn add_world(str: &mut String) -> () {
    str.push_str(", world!");
}

fn some_function() -> String {
    String::from("yeeee")
}

fn get_length(str: &String) -> usize {
    str.len()
}
