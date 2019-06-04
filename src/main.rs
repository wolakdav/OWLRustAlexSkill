extern crate chrono;
extern crate json;
extern crate reqwest;
use std::collections::HashMap;
use std::io;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Welcome to the Overwatch League Tracker! While we're unfornately still terminal based theres plan to change that soon so stay tuned!");
    println!("--------------");
    printCurrentAndNextMatch();
    let mut userChoice = 1000000000;
    while userChoice != 4 {
        while userChoice == 1000000000 {
            userChoice = menuChoice();
        }
        match userChoice {
            1 => {
                let nextMatch = getNextMatch().unwrap();
                println!("{}", nextMatch);
            }
            2 => {
                let teamInfo = getTeamInfo().unwrap();
                println!("{}", teamInfo);
            }
            3 => {
                let rankings = getAllRankings().unwrap();
                println!("{}", rankings);
            }
            4 => {
                let userChoice = 4;
            }
            _ => println!("Error:Invalid selections"),
        };
    }
    Ok(())
}

fn getAllRankings() -> Result<(String), Box<std::error::Error>> {
    let mut rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();

    let rankings = rankings["content"].clone();
    let amtCompetitors = rankings.len();
    let mut result = "Current Overwatch League Rankings:".to_string();
    let mut teamName = String::new();
    let mut rank = String::new();
    for mut i in 0..amtCompetitors {
        teamName = rankings[i]["competitor"]["name"].to_string();
        rank = rankings[i]["placement"].to_string();
        result = format!("{}\n {}:  Rank {}", result, teamName, rank);
    }
    Ok(result)
}

fn getTeamInfo() -> Result<(String), Box<std::error::Error>> {
    let teamIDs = [
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
    let mut apiURL = String::new();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let input = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1000000000,
    };
    if input < 0 || input > 20 {
        return Ok("That is not a valid team".to_string());
    }
    apiURL = format!(
        "https://api.overwatchleague.com/teams/{}",
        teamIDs[(input - 1) as usize].to_string()
    ); //This always give back the same data so this function is bugged =/, looks like ranking call has the correct stats thou so that might be worth a look

    let teamInfo = json::parse(&(reqwest::get(&apiURL[..])?.text()?).to_string()).unwrap();
    let teamName = teamInfo["description"].to_string();
    let teamRank = getRanking(teamName.clone()).unwrap();
    let matchWins = teamInfo["ranking"]["matchWin"].to_string();
    let matchLoss = teamInfo["ranking"]["matchLoss"].to_string();
    let matchDraw = teamInfo["ranking"]["matchDraw"].to_string();
    let gameWin = teamInfo["ranking"]["gameWin"].to_string();
    let gameLoss = teamInfo["ranking"]["gameLoss"].to_string();
    let gameTie = teamInfo["ranking"]["gameTie"].to_string();

    let result = format!(
        "{} is currently {} place in the league.
    During the current/last stage they had {} match wins,{} match losses, and {} match ties.
    During that time they had won {} maps, lost {} maps, and tied on {} maps",
        teamName, teamRank, matchWins, matchLoss, matchDraw, gameWin, gameLoss, gameTie
    );

    Ok(result)
}

fn printCurrentAndNextMatch() -> Result<(), Box<std::error::Error>> {
    let liveMatchData = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/live-match")?.text()?).to_string(),
    )
    .unwrap();
    let liveMatch = liveMatchData["data"]["liveMatch"].clone();
    let nextMatch = liveMatchData["data"]["nextMatch"].clone();

    let currentMatch = getCurrentMatch(liveMatch);
    println!("{}", currentMatch);
    let nextMatch = getTodaysNextMatch(nextMatch);
    println!("{}", nextMatch);
    Ok(())
}

fn menuChoice() -> u32 {
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

fn getCurrentMatch(liveMatch: json::JsonValue) -> String {
    let firstContendor = (&liveMatch["competitors"][0]["name"]).to_string();
    let secondContendor = (&liveMatch["competitors"][1]["name"]).to_string();
    let firstContendorScore = (&liveMatch["scores"][0]["value"]).to_string();
    let secondContendorScore = (&liveMatch["scores"][1]["value"]).to_string();
    let round = (&liveMatch["round"]).to_string();
    let map = (&liveMatch["games"][2]["attributes"]["map"]).to_string();
    let currentState = (&liveMatch["liveStatus"]).to_string();
    let mut result = String::new();
    let mut currentWinner = String::new();
    let mut leaderScore = String::new();
    let mut loserScore = String::new();

    if currentState == "LIVE" {
        if firstContendorScore == secondContendorScore {
            currentWinner = ("the teams being tied").to_string();
            leaderScore = firstContendorScore.clone();
            loserScore = secondContendorScore.clone();
        } else if firstContendorScore > secondContendorScore {
            currentWinner = firstContendor.clone();
            leaderScore = firstContendorScore.clone();
            loserScore = secondContendorScore.clone();
        } else {
            currentWinner = secondContendor.clone();
            leaderScore = secondContendorScore.clone();
            loserScore = secondContendorScore.clone();
        }
        result = format!(
            "The current match is {} vs {} with {} winning at {} points to {}",
            firstContendor, secondContendor, currentWinner, leaderScore, loserScore
        );
    } else {
        let mut timeToMatchString = ((&liveMatch["timeToMatch"]).to_string());
        if (timeToMatchString != "null") && (firstContendor != "null") & (secondContendor != "null")
        {
            let timeToMatchFloat = timeToMatchString.parse::<f64>().map(|n| n + 1.5).unwrap();
            let timeToNextConverted = convertMilliSeconds(timeToMatchFloat);
            result = format!(
                "The next match is {} agasint {} in {}",
                firstContendor, secondContendor, timeToNextConverted
            );
        }
    }
    let nextMatch = getNextMatch().unwrap();
    return format!("{} \n {} ", result, nextMatch);
}

fn getTodaysNextMatch(nextMatch: json::JsonValue) -> String {
    let nextFirstContendor = (&nextMatch["competitors"][0]["name"]).to_string();
    let nextSecondContendor = (&nextMatch["competitors"][1]["name"]).to_string();
    if (nextFirstContendor != "null" && nextSecondContendor != "null") {
        let firstContendorRanking = getRanking(nextFirstContendor.to_string()).unwrap();
        let secondContendorRanking = getRanking(nextSecondContendor.to_string()).unwrap();

        return format!(
            "The next match will be between {}(League Rank {}) and {}(League Rank {})",
            nextFirstContendor.to_string(),
            firstContendorRanking.to_string(),
            nextSecondContendor.to_string(),
            secondContendorRanking.to_string()
        );
    } else {
        return format!("There are no more matches today,check back in tomorrow!");
    }
}
fn getRanking(team: String) -> Result<(usize), Box<std::error::Error>> {
    let rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();

    let mut position = 0;
    let mut found = false;
    let mut compareTo = &rankings["content"][position]["competitor"]["name"];
    while (position < 21 && found == false) {
        compareTo = &rankings["content"][position]["competitor"]["name"];
        if (team == compareTo.to_string()) {
            found = true;
        } else {
            position = position + 1;
        }
    }
    return Ok(position + 1);
}

fn convertMilliSeconds(timeToNext: f64) -> String {
    if timeToNext > 60000.0 {
        let mut converted = timeToNext / (3.6 * (1000000.0));
        let hours = converted.floor();
        let minutes = 60.0 * (converted - hours);
        if (hours > 1.0) {
            format!("{:.0} hours and {:.0} minutes", hours, minutes)
        } else {
            format!("{:.0} minutes, time to get hyped!", minutes)
        }
    } else if (timeToNext <= 60000.0) && (timeToNext >= 0.0) {
        format!("Match is starting in less then a minute!")
    } else if timeToNext < 0.0 {
        format!("A error has occured getting the time until next match")
    } else {
        format!("The match has started!")
    }
}

fn getNextMatch() -> Result<(String), Box<std::error::Error>> {
    let schedule = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/schedule")?.text()?).to_string(),
    )
    .unwrap();
    let currentStage = getCurrentStage(schedule["data"]["stages"].clone());
    let matchId = getNextMatchInSchedule(schedule["data"]["stages"][currentStage].clone());
    return Ok(matchId);
}

fn getNextMatchInSchedule(stage: json::JsonValue) -> String {
    let matches = &stage["matches"];
    let numberOfMatches = matches.len();
    for mut i in 0..numberOfMatches {
        if (matches[i + 3]["state"] == "CONCLUDED")
        //All previous matches for that day are concluded
        {
            i = i + 3;
        } else {
            if (matches[i]["state"] == "PENDING") {
                let firstContendor = (&matches[i]["competitors"][0]["name"]).to_string();
                let secondContendor = (&matches[i]["competitors"][1]["name"]).to_string();
                let firstContendorRanking = getRanking(firstContendor.clone()).unwrap();
                let secondContendorRanking = getRanking(secondContendor.clone()).unwrap();
                let mut startDate = (&matches[i]["startDate"]).to_string();
                if (startDate != "null") && (firstContendor != "null") & (secondContendor != "null")
                {
                    startDate = formatDate(startDate);

                    return format!(
                        "The next match is {}(League Rank {}) vs {}(League Rank {}) on {} ",
                        firstContendor,
                        firstContendorRanking,
                        secondContendor,
                        secondContendorRanking,
                        startDate
                    );
                } else {
                    return "Error getting new match information".to_string();
                }
            }
        }
    }
    return "Failed to find matchId".to_string();;
}

fn formatDate(mut date: String) -> String {
    date = date.replacen("-", " ", 3);
    date = date.replacen("T", " ", 1);
    let foo: Vec<&str> = date.split(" ").collect();
    let mut month = String::new();
    month = getMonth(foo[1].to_string());
    let day = formatDay(foo[2].to_string());
    let year = foo[0].to_string();
    return format!("{} {}, {}", month, day, year);
}

fn formatDay(day: String) -> String {
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
fn getMonth(month: String) -> String {
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
fn getCurrentStage(stages: json::JsonValue) -> usize {
    for i in 0..4 {
        let stageToCheck = &stages[i];
        let stageMatchesAmount = stageToCheck["matches"].len();
        let lastMatchConclussion = &stageToCheck["matches"][stageMatchesAmount - 1]["state"];
        if (lastMatchConclussion != "CONCLUDED") {
            return i;
        }
    }
    return 1000000;
}

#[cfg(test)]
mod tests {
    use crate::convertMilliSeconds;
    use crate::formatDate;
    use crate::formatDay;
    use crate::getMonth;
    #[test]
    fn test_milli_second_conversion() {
        let valid_time = convertMilliSeconds(1002313231.0);
        let valid_time_2 = convertMilliSeconds(30000.0);
        let valid_time_3 = convertMilliSeconds(1.0);
        let valid_time_4 = convertMilliSeconds(0.0);
        let invalid_time = convertMilliSeconds(-10231.0);
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
        let valid_time_stamp = formatDate("2019-02-15T00:00:00.000Z".to_string());
        let invald_time_stamp = formatDate("Invalid TimeStamp".to_string());
        assert_eq!(valid_time_stamp, "February 15th, 2019");
        assert_eq!(invald_time_stamp, "Unknown month Error imeStampth, Invalid");
    }

    #[test]
    fn format_day_test() {
        let first = formatDay("01".to_string());
        let second = formatDay("02".to_string());
        let third = formatDay("03".to_string());
        let seventh = formatDay("07".to_string());
        let thirtyth = formatDay("30".to_string());
        assert_eq!(first, "1st".to_string());
        assert_eq!(second, "2nd".to_string());
        assert_eq!(third, "3rd".to_string());
        assert_eq!(seventh, "7th".to_string());
        assert_eq!(thirtyth, "30th".to_string());
    }

    #[test]
    fn test_get_month() {
        let jan = getMonth("01".to_string());
        let july = getMonth("07".to_string());
        let invalid_month = getMonth("123210".to_string());
        assert_eq!(jan, "January".to_string());
        assert_eq!(july, "July".to_string());
        assert_eq!(invalid_month, "Unknown month Error".to_string());
    }
}
