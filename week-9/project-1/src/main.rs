use std::io::Write;


fn main() {
    let mut file = std::fs::File::create("project1.csv").expect("Couldnt create file because you are paying 100k school fees");
    file.write_all("lager,stout,non-alcoholic".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    file.write_all("\n 33 Export,Legend,Maltina".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
     file.write_all("\n Desperados,Turba King,Amstel Malt".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    file.write_all("\n Goldberg,Williams,Malta Gold".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    file.write_all("\n Guider, ,Fayrouz".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    file.write_all("\n Heineken, , ".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    file.write_all("\n Star, , ".as_bytes()).expect("Couldnt Write because you are paying 100k school fees");
    
    
}
