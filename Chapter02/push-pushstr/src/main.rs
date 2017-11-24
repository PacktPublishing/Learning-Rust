fn main() {
    let home_team = "Liverpool";
    let result = " beat ";
    let away_team = "Manchester United";
    let home_score = '3'; // single character
    let away_score = "-0";
    
    let mut full_line = format!("{}{}{} ", home_team, result, away_team);
        
     // add the character to the end of the String    
     full_line.push(home_score);
     
     // add the away score to the end of the String
     full_line.push_str(away_score);
        
    println!("{}", full_line);
}
