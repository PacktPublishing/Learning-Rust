// 02/addstring-2/main.rs
fn main() {
    let homeTeam = "Liverpool";
    let result = " beat ";
    let awayTeam = "Manchester United";
    
    let fullLine = homeTeam.to_owned() + result + awayTeam;
    
    println!("{}", fullLine);
}
