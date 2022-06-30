
struct Userdetails {
    firstname: String,
    lastname: String,
}

fn main() {
    let user = Userdetails {
        firstname : String::from("Ark"),
        lastname : String::from("Pathak")
    };
    println!("My full name is {} {}", user.firstname, user.lastname);
}
