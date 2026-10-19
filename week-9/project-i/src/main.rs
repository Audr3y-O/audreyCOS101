use::std::io::Write;



fn main(){

    let announce = "Welcome to Nigerian Breweries Plc\n";
    let choice = "Here are the drinks available.\n";



    let larger_drinks = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout_drinks = vec!["Legend","Turbo King", "Williams"," "," "," " ];
    let non_alcoholic_drinks = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"," "," "];

    let mut file = std::fs::File::create("Nigerian-Breweries-Plc.csv").expect("create failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(choice.as_bytes()).expect("write failed");

    file.write_all("Larger drinks,Stout drinks,Non-alcoholic drinks\n".as_bytes()).expect("write failed");

    for i in 0..larger_drinks.len(){
        file.write_all(larger_drinks[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(stout_drinks[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(non_alcoholic_drinks[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }


}