enum Team {
    NYG,
    DAL,
    PHI,
    WAS,
}

fn print_team_name(team: Team) {
    match team {
        Team::NYG => {
            println!("New York Football Giants.");
        }
        Team::DAL => {
            println!("Dallas Cowboys.");
        },
        Team::PHI => {
            println!("Philadelphia Eagles.");
        },
        Team::WAS => {
            println!("Washington Football Team.");
        },
    }
}

fn main() {
    print_team_name(Team::NYG);
    print_team_name(Team::DAL); 
    print_team_name(Team::PHI); 
    print_team_name(Team::WAS);  
}
