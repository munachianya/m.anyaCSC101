use std::io::Write;

fn code_7(){
    let n = vec!["\nName = Aigbona Juliet, ", "\nName = Akpevwe Iloka, "];
    let d = vec!["\nDepartment = Consulting, ", "\nDepartment = Assurance, "];
    let q = vec!["\nQualification = B.Sc., ", "\nQualification = HND, "];
    let c = ["Departmental Services = Analytics consulting services, \n", "Customer experience, \n", "Cybersecurity, strategy, risk, compliance and resilience, \n", 
    "Digital transformation, \n", "Risk consulting services, \n", "Supply chain and operations, \n", "Technology transformation.\n"];
    
    let a = ["Departmental Services = Audit services, \n", "Climate change and sustainability services, \n", "Financial accounting advisory services, \n", 
    "Forensic and integrity services, \n", "Private client audit experience, \n", "Accounting Link, \n", "Assurance.\n"];

    let mut file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[0].as_bytes()).expect("write failed");
    file.write_all(d[0].as_bytes()).expect("write failed");
    file.write_all(q[0].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(c[i].as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[1].as_bytes()).expect("write failed");
    file.write_all(d[1].as_bytes()).expect("write failed");
    file.write_all(q[1].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(a[i].as_bytes()).expect("write failed");
    }
    println!("Data written to file.");
}

fn code_8(){
    let n = vec!["\nName = Adamu Sagamu, ", "\nName = Gbenga Daniels, "];
    let d = vec!["\nDepartment = Tax, ", "\nDepartment = People and workforce, "];
    let q = vec!["\nQualification = B.Sc., ", "\nQualification = HND, "];
    let tx = ["Departmental Services = Tax planning, \n", "Tax function operations, \n", "Tax policy and controversy, \n", 
    "Global trade, \n", "Tax accounting, \n", "Tax compliance, \n", "Transaction tax.\n"];
    
    let p = ["Departmental Services = Change management and experience, \n", "HR transformation, \n", "Integrated workforce mobility, \n", 
    "Learning and development consulting, \n", "Recognition and reward advisory, \n", "Workforce analytics, \n", "People and workforce.\n"];

    let mut file = std::fs::File::create("adamu_sagamu.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[0].as_bytes()).expect("write failed");
    file.write_all(d[0].as_bytes()).expect("write failed");
    file.write_all(q[0].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(tx[i].as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::create("gbenga_daniels.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[1].as_bytes()).expect("write failed");
    file.write_all(d[1].as_bytes()).expect("write failed");
    file.write_all(q[1].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(p[i].as_bytes()).expect("write failed");
    }
    println!("Data written to file.");
}

fn code_9(){
    let n = vec!["\nName = Ehis Ero, ", "\nName = Maria Akinsola, "];
    let d = vec!["\nDepartment = Strategy, ", "\nDepartment = Transactions and corporate finance, "];
    let q = vec!["\nQualification = M.Sc., ", "\nQualification = M.Sc., "];
    let s = ["Departmental Services = Strategy consulting, \n", "Corporate and growth strategy, \n", "Transaction strategy and execution, \n", 
    "Restructuring and turnaround strategy, \n", "Industry strategy, \n", "Digital business building, \n", "Commercial strategy.\n"];
    
    let tc = ["Departmental Services = Corporate finance, \n", "Divestment and carve-outs, \n", "Sustainability and ESG Services, \n", 
    "M&A advisory, \n", "M&A integration, \n", "M&A technology and tools, \n", "M&A advanced analytics.\n"];

    let mut file = std::fs::File::create("ehis_ero.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[0].as_bytes()).expect("write failed");
    file.write_all(d[0].as_bytes()).expect("write failed");
    file.write_all(q[0].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(s[i].as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::create("maria_akinsola.txt").expect("create failed");
    file.write_all("Welcome to Ernst & Young Global Limited database. \n"
        .as_bytes()).expect("write failed");
    file.write_all(n[1].as_bytes()).expect("write failed");
    file.write_all(d[1].as_bytes()).expect("write failed");
    file.write_all(q[1].as_bytes()).expect("write failed");
    for i in 0..7{
    file.write_all(tc[i].as_bytes()).expect("write failed");
    }
    println!("Data written to file.");
}
    
fn main(){
    let mut input1 = String::new();
    println!("Welcome to Ernst & Young Global Limited database");
    println!("Enter a value of code between (7 - 9)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code:char = input1.trim().parse().expect("Invalid Input");
    if code == '7' {
        code_7();
    }
    else if code == '8'{
        code_8();
    }
    else if code == '9' {
        code_9();
    }
    else {
        println!("Wrong input! Please Pick a value from (7 - 9)");
        return;
    }
}













