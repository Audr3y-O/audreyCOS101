use std::io::Write;



fn main(){

    let announce = "PAU's Student Management Information System\n";
    


    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical","Mechanical"];
    let level = vec!["300", "100", "200", "200","100"];

    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    
    file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("write failed");

    for i in 0..student_name.len(){
        file.write_all(student_name[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(matric_number[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(department[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(level[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }


}