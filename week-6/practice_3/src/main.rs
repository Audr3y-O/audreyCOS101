fn main() {
    let name1 = "Audrey Opute";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Audery","Random name");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    let school = faculty.replace("Faculty","School");
    println!("I'm a student of the {}",school);
}
