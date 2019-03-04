// struct Name {
//     _name : String
// }
//
// impl Name {
//     fn pretty_name (&self) -> String {
//         return "shoot".to_string();
//     }
// }
//
// struct Hero {
//     name: Name
//
// }

fn main(){
    // //wtf no default constructors or anything. Rust sucks man. Josh Dec 20, 2018
    // let josh = Hero{name: Name{_name:"".to_string()}};

    let mut my_vector: Vec<i32> = Vec::new();
    my_vector.push(1);
    my_vector.push(2);
    my_vector.push(3);

    for i in my_vector {
        println!("Poop {}", i);
    }
}
