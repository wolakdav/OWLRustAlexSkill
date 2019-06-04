extern crate chrono;
extern crate json;
extern crate reqwest;
use std::io;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Welcome to the Overwatch League Tracker! While we're unfornately still terminal based theres plan to change that soon so stay tuned!");
    println!("--------------");
    let error = print_current_and_next_match();
    let mut user_choice = 1000000000;
    while user_choice != 4 {
        while user_choice == 1000000000 {
            user_choice = menu_choice();
        }
        match user_choice {
            1 => {
                let next_match = get_next_match().unwrap();
                println!("{}", next_match);
            }
            2 => {
                let team_info = get_team_info().unwrap();
                println!("{}", team_info);
            }
            3 => {
                let rankings = get_all_rankings().unwrap();
                println!("{}", rankings);
            }
            4 => {
                let _user_choice = 4;
            }
            _ => println!("Error:Invalid selections"),
        };
    }
    Ok(())
}

fn get_all_rankings() -> Result<(String), Box<std::error::Error>> {
    let rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();

    let rankings = rankings["content"].clone();
    let amt_competitors = rankings.len();
    let mut result = "Current Overwatch League Rankings:".to_string();
    let mut team_name = String::new();
    let mut rank = String::new();
    for i in 0..amt_competitors {
        team_name = rankings[i]["competitor"]["name"].to_string();
        rank = rankings[i]["placement"].to_string();
        result = format!("{}\n {}:  Rank {}", result, team_name, rank);
    }
    Ok(result)
}

fn get_team_info() -> Result<(String), Box<std::error::Error>> {
    let team_ids = [
        7698, 4402, 7692, 4523, 4407, 7699, 7693, 4525, 4410, 4406, 4405, 4403, 7694, 4524, 4404,
        4409, 4408, 7695, 7696, 7697,
    ];
    println!("Select which team you like to know more about:\n");
    println!(
        "
    1. Atlanta Reign
    2.Boston Uprising	
    3.Chengdu Hunters 
    4. Dallas Fuel 
    5.Florida Mayhem 
    6.Guangzhou Charge	
    7.Hangzhou Spark	
    8.Houston Outlaws	
    9.London Spitfire	
    10.Los Angeles Gladiators	
    11.Los Angleles Valiant	
    12.New York Excelsior	
    13.Paris Eternal	
    14.Philadelphia Fusion	
    15.San Francisco Shock	
    16.Seoul Dynasty	
    17.Shanghai Dragons	
    18.Toronto Defiant	
    19.Vancouver Titans	
    20.Washington Justice"
    );
    let mut api_url = String::new();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let input = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1000000000,
    };
    if input <= 0 || input > 20 {
        return Ok("That is not a valid team".to_string());
    }
    api_url = format!(
        "https://api.overwatchleague.com/teams/{}",
        team_ids[(input - 1) as usize].to_string()
    ); //This always give back the same data so this function is bugged =/, looks like ranking call has the correct stats thou so that might be worth a look

    let team_info = json::parse(&(reqwest::get(&api_url[..])?.text()?).to_string()).unwrap();
    let team_name = team_info["description"].to_string();
    let team_rank = get_ranking(team_name.clone()).unwrap();
    let match_wins = team_info["ranking"]["matchWin"].to_string();
    let match_loss = team_info["ranking"]["match_loss"].to_string();
    let match_draw = team_info["ranking"]["match_draw"].to_string();
    let game_win = team_info["ranking"]["game_win"].to_string();
    let game_loss = team_info["ranking"]["game_loss"].to_string();
    let game_tie = team_info["ranking"]["game_tie"].to_string();

    let result = format!(
        "{} is currently {} place in the league.
    During the current/last stage they had {} match wins,{} match losses, and {} match ties.
    During that time they had won {} maps, lost {} maps, and tied on {} maps",
        team_name, team_rank, match_wins, match_loss, match_draw, game_win, game_loss, game_tie
    );

    Ok(result)
}

fn print_current_and_next_match() -> Result<(), Box<std::error::Error>> {
    let live_match_data = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/live-match")?.text()?).to_string(),
    )
    .unwrap();
    let live_match = live_match_data["data"]["live_match"].clone();
    let next_match = live_match_data["data"]["next_match"].clone();

    let current_match = get_current_match(live_match);
    println!("{}", current_match);
    let next_match = get_todays_next_match(next_match);
    println!("{}", next_match);
    Ok(())
}

fn menu_choice() -> u32 {
    println!("What would you like to do:");
    println!("1. Get the next match");
    println!("2. Get more information about a team");
    println!("3. See the league ranks");
    println!("4. Close the app");
    println!("Enter your choice of what you would like to do:");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1000000000,
    }
}

fn get_current_match(live_match: json::JsonValue) -> String {
    let first_contendor = (&live_match["competitors"][0]["name"]).to_string();
    let second_contendor = (&live_match["competitors"][1]["name"]).to_string();
    let first_contendor_score = (&live_match["scores"][0]["value"]).to_string();
    let second_contendor_score = (&live_match["scores"][1]["value"]).to_string();
    let round = (&live_match["round"]).to_string();
    let map = (&live_match["games"][2]["attributes"]["map"]).to_string();
    let current_state = (&live_match["liveStatus"]).to_string();
    let mut result = String::new();
    let mut current_winner = String::new();
    let mut leader_score = String::new();
    let mut loser_score = String::new();

    if current_state == "LIVE" {
        if first_contendor_score == second_contendor_score {
            current_winner = ("the teams being tied").to_string();
            leader_score = first_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        } else if first_contendor_score > second_contendor_score {
            current_winner = first_contendor.clone();
            leader_score = first_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        } else {
            current_winner = second_contendor.clone();
            leader_score = second_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        }
        result = format!(
            "The current match is {} vs {} on {} round {} \n {} is currently winning with {} points to {}",
            first_contendor, second_contendor,map,round, current_winner, leader_score, loser_score
        );
    } else {
        let time_to_match_string = (&live_match["timeToMatch"]).to_string();
        if (time_to_match_string != "null") && (first_contendor != "null") & (second_contendor != "null")
        {
            let time_to_match_float = time_to_match_string.parse::<f64>().map(|n| n + 1.5).unwrap();
            let time_to_next_converted = convert_milli_seconds(time_to_match_float);
            result = format!(
                "The next match is {} agasint {} in {}",
                first_contendor, second_contendor, time_to_next_converted
            );
        }
    }
    let next_match = get_next_match().unwrap();
    return format!("{} \n {} ", result, next_match);
}

fn get_todays_next_match(next_match: json::JsonValue) -> String {
    let next_first_contendor = (&next_match["competitors"][0]["name"]).to_string();
    let next_second_contendor = (&next_match["competitors"][1]["name"]).to_string();
    if next_first_contendor != "null" && next_second_contendor != "null" {
        let first_contendor_ranking = get_ranking(next_first_contendor.to_string()).unwrap();
        let second_contendor_ranking = get_ranking(next_second_contendor.to_string()).unwrap();

        return format!(
            "The next match will be between {}(League Rank {}) and {}(League Rank {})",
            next_first_contendor.to_string(),
            first_contendor_ranking.to_string(),
            next_second_contendor.to_string(),
            second_contendor_ranking.to_string()
        );
    } else {
        return format!("There are no more matches today,check back in tomorrow!");
    }
}
fn get_ranking(team: String) -> Result<(usize), Box<std::error::Error>> {
    let rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();

    let mut position = 0;
    let mut found = false;
    let mut compare_to = &rankings["content"][position]["competitor"]["name"];
    while position < 21 && found == false {
        compare_to = &rankings["content"][position]["competitor"]["name"];
        if team == compare_to.to_string() {
            found = true;
        } else {
            position = position + 1;
        }
    }
    return Ok(position + 1);
}

fn convert_milli_seconds(time_to_next: f64) -> String {
    if time_to_next > 60000.0 {
        let converted = time_to_next / (3.6 * (1000000.0));
        let hours = converted.floor();
        let minutes = 60.0 * (converted - hours);
        if hours > 1.0 {
            format!("{:.0} hours and {:.0} minutes", hours, minutes)
        } else {
            format!("{:.0} minutes, time to get hyped!", minutes)
        }
    } else if (time_to_next <= 60000.0) && (time_to_next >= 0.0) {
        format!("Match is starting in less then a minute!")
    } else if time_to_next < 0.0 {
        format!("A error has occured getting the time until next match")
    } else {
        format!("The match has started!")
    }
}

fn get_next_match() -> Result<(String), Box<std::error::Error>> {
    let schedule = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/schedule")?.text()?).to_string(),
    )
    .unwrap();
    let current_stage = get_current_stage(schedule["data"]["stages"].clone());
    let match_id = get_next_match_in_schedule(schedule["data"]["stages"][current_stage].clone());
    return Ok(match_id);
}

fn get_next_match_in_schedule(stage: json::JsonValue) -> String {
    let matches = &stage["matches"];
    let number_of_matches = matches.len();
    for mut i in 0..number_of_matches {
        if matches[i + 3]["state"] == "CONCLUDED"
        //All previous matches for that day are concluded
        {
            i = i + 3;
        } else {
            if matches[i]["state"] == "PENDING" {
                let first_contendor = (&matches[i]["competitors"][0]["name"]).to_string();
                let second_contendor = (&matches[i]["competitors"][1]["name"]).to_string();
                let first_contendor_ranking = get_ranking(first_contendor.clone()).unwrap();
                let second_contendor_ranking = get_ranking(second_contendor.clone()).unwrap();
                let mut start_date = (&matches[i]["start_date"]).to_string();
                if (start_date != "null") && (first_contendor != "null") & (second_contendor != "null")
                {
                    start_date = format_date(start_date);

                    return format!(
                        "The next match is {}(League Rank {}) vs {}(League Rank {}) on {} ",
                        first_contendor,
                        first_contendor_ranking,
                        second_contendor,
                        second_contendor_ranking,
                        start_date
                    );
                } else {
                    return "Error getting new match information".to_string();
                }
            }
        }
    }
    return "Failed to find match_id".to_string();;
}

fn format_date(mut date: String) -> String {
    date = date.replacen("-", " ", 3);
    date = date.replacen("T", " ", 1);
    let foo: Vec<&str> = date.split(" ").collect();
    let mut month = String::new();
    month = get_month(foo[1].to_string());
    let day = format_day(foo[2].to_string());
    let year = foo[0].to_string();
    return format!("{} {}, {}", month, day, year);
}

fn format_day(day: String) -> String {
    match day.as_ref() {
        "01" => "1st".to_string(),
        "02" => "2nd".to_string(),
        "03" => "3rd".to_string(),
        "04" => "4th".to_string(),
        "05" => "5th".to_string(),
        "06" => "6th".to_string(),
        "07" => "7th".to_string(),
        "08" => "8th".to_string(),
        "9" => "9th".to_string(),
        _ => (day + "th").to_string(),
    }
}
fn get_month(month: String) -> String {
    match month.as_ref() {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "September",
        "09" => "September",
        "10" => "October",
        "11" => "November",
        "12" => "December",
        _ => "Unknown month Error",
    }
    .to_string()
}
fn get_current_stage(stages: json::JsonValue) -> usize {
    for i in 0..4 {
        let stage_to_check = &stages[i];
        let stage_matches_amount = stage_to_check["matches"].len();
        let last_match_conclussion = &stage_to_check["matches"][stage_matches_amount - 1]["state"];
        if last_match_conclussion != "CONCLUDED" {
            return i;
        }
    }
    return 1000000;
}

#[cfg(test)]
mod tests {
    use crate::convert_milli_seconds;
    use crate::format_date;
    use crate::format_day;
    use crate::get_month;
    #[test]
    fn test_milli_second_conversion() {
        let valid_time = convert_milli_seconds(1002313231.0);
        let valid_time_2 = convert_milli_seconds(30000.0);
        let valid_time_3 = convert_milli_seconds(1.0);
        let valid_time_4 = convert_milli_seconds(0.0);
        let invalid_time = convert_milli_seconds(-10231.0);
        assert_eq!(valid_time, "278 hours and 25 minutes");
        assert_eq!(valid_time_2, "Match is starting in less then a minute!");
        assert_eq!(valid_time_3, "Match is starting in less then a minute!");
        assert_eq!(valid_time_4, "Match is starting in less then a minute!");
        assert_eq!(
            invalid_time,
            "A error has occured getting the time until next match"
        );
    }

    #[test]
    fn format_date_test() {
        let valid_time_stamp = format_date("2019-02-15T00:00:00.000Z".to_string());
        let invald_time_stamp = format_date("Invalid TimeStamp".to_string());
        assert_eq!(valid_time_stamp, "February 15th, 2019");
        assert_eq!(invald_time_stamp, "Unknown month Error imeStampth, Invalid");
    }

    #[test]
    fn format_day_test() {
        let first = format_day("01".to_string());
        let second = format_day("02".to_string());
        let third = format_day("03".to_string());
        let seventh = format_day("07".to_string());
        let thirtyth = format_day("30".to_string());
        assert_eq!(first, "1st".to_string());
        assert_eq!(second, "2nd".to_string());
        assert_eq!(third, "3rd".to_string());
        assert_eq!(seventh, "7th".to_string());
        assert_eq!(thirtyth, "30th".to_string());
    }

    #[test]
    fn test_get_month() {
        let jan = get_month("01".to_string());
        let july = get_month("07".to_string());
        let invalid_month = get_month("123210".to_string());
        assert_eq!(jan, "January".to_string());
        assert_eq!(july, "July".to_string());
        assert_eq!(invalid_month, "Unknown month Error".to_string());
    }
}
