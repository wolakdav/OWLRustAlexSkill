extern crate reqwest;
extern crate json;
extern crate chrono;
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
 
    let liveMatchData = json::parse(&(reqwest::get("https://api.overwatchleague.com/live-match")?.text()?).to_string()).unwrap();
    let rankings = json::parse(&(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string()).unwrap();
    let liveMatch = liveMatchData["data"]["liveMatch"].clone();
    let nextMatch = liveMatchData["data"]["nextMatch"].clone();

    let currentMatch = getCurrentMatch(liveMatch);
    println!("{}",currentMatch);
    let nextMatch = getTodaysNextMatch(nextMatch,rankings);
    println!("{}", nextMatch);
    Ok(())
}
fn getCurrentMatch(liveMatch:json::JsonValue) -> String{
    let firstContendor =  (&liveMatch["competitors"][0]["name"]).to_string();
    let secondContendor = (&liveMatch["competitors"][1]["name"]).to_string();
    let firstContendorScore = (&liveMatch["scores"][0]["value"]).to_string();
    let secondContendorScore = (&liveMatch["scores"][1]["value"]).to_string();
    let round = (&liveMatch["round"]).to_string();
    let map  = (&liveMatch["games"][2]["attributes"]["map"]).to_string();
    let currentState = (&liveMatch["liveStatus"]).to_string();
    let mut result = String::new();
    let mut currentWinner = String::new();
    let mut leaderScore = String::new();
    let mut loserScore = String::new();
   //println!("{}",firstContendorScore.clone());
    if currentState == "LIVE"
    {
        if firstContendorScore == secondContendorScore
        {
            currentWinner = ("the teams being tied").to_string();
            leaderScore = firstContendorScore.clone();
            loserScore = secondContendorScore.clone();
        }else if firstContendorScore > secondContendorScore{
            currentWinner = firstContendor.clone();
            leaderScore = firstContendorScore.clone();
            loserScore = secondContendorScore.clone();
        }else{
            currentWinner = secondContendor.clone();
            leaderScore = secondContendorScore.clone();
            loserScore = secondContendorScore.clone();
        }
         result = format!("The current match is {} vs {} with {} winning at {} points to {}",firstContendor,secondContendor,currentWinner,leaderScore,loserScore);
    }else{ 
        let mut timeToMatchString = ((&liveMatch["timeToMatch"]).to_string());
        if (timeToMatchString != "null") && (firstContendor != "null") & (secondContendor != "null")
        {
        let timeToMatchFloat = timeToMatchString.parse::<f64>().map(|n| n + 1.5).unwrap();
        let timeToNextConverted = convertMilliSeconds(timeToMatchFloat);
        result = format!("The next match is {} agasint {} in {}",firstContendor,secondContendor,timeToNextConverted);
        }
      
    }
    getNextMatch();
    return result
}

fn getTodaysNextMatch(nextMatch:json::JsonValue,rankings:json::JsonValue) -> String
{
    let nextFirstContendor = (&nextMatch["competitors"][0]["name"]).to_string();
    let nextSecondContendor = (&nextMatch["competitors"][1]["name"]).to_string();
    if(nextFirstContendor != "null" && nextSecondContendor != "null")
    {
    let firstContendorRanking = getRanking(nextFirstContendor.to_string(),rankings.clone());
    let secondContendorRanking  = getRanking(nextSecondContendor.to_string(),rankings.clone());

    return format!("The next match will be between {}(League Rank {}) and {}(League Rank {})",nextFirstContendor.to_string(),firstContendorRanking.to_string(),nextSecondContendor.to_string(),secondContendorRanking.to_string());
    }else{

        return format!("There are no more matches today,check back in tomorrow!");
    }
}
fn getRanking(team: String, rankings:json::JsonValue ) -> usize
{
    let mut position = 0;
    let mut found = false;
    let mut compareTo =  &rankings["content"][position]["competitor"]["name"];
    while(position < 21 && found == false )
    {
        compareTo =  &rankings["content"][position]["competitor"]["name"];
        if(team == compareTo.to_string())
        {
            found = true;
        }else{
            position = position+1;
        }
    }
    return position+1;
}

fn convertMilliSeconds(timeToNext: f64) -> String {
    let mut converted = timeToNext/(3.6*(1000000.0));
    let hours = converted.floor();
    let minutes = 60.0*(converted-hours);
    if(hours > 1.0)
    {
        format!("{:.0} hours and {:.0} minutes",hours,minutes)
    }else{
        format!("{:.0} minutes, time to get hyped!",minutes)
    }
}

fn getNextMatch()->  Result<(), Box<std::error::Error>>{
      let schedule = json::parse(&(reqwest::get("https://api.overwatchleague.com/schedule")?.text()?).to_string()).unwrap();
      let currentStage = getCurrentStage(schedule["data"]["stages"].clone());
      let matchId = getNextMatchInSchedule(schedule["data"]["stages"][currentStage].clone());
      println!("{}",matchId);
      Ok(())
      
}


fn getNextMatchInSchedule(stage:json::JsonValue)->String{
    let matches = &stage["matches"];
    let numberOfMatches = matches.len();
     for mut i in 0..numberOfMatches
    {
        if(matches[i+3]["state"] == "CONCLUDED")//All previous matches for that day are concluded
        {
            i = i + 3;
        }else{
            
            if(matches[i]["state"] == "PENDING")
            {
                let firstContendor =  (&matches[i]["competitors"][0]["name"]).to_string();
                let secondContendor = (&matches[i]["competitors"][1]["name"]).to_string();
                let mut startDate = (&matches[i]["startDate"]).to_string();
                if (startDate != "null") && (firstContendor != "null") & (secondContendor != "null")
                {
                    startDate = formateDate(startDate);
                    return format!("The next match is {} vs {} on {} ",firstContendor,secondContendor,startDate);
                }else{
                    return "Error getting new match information".to_string();
                }
            }
        }
    }
    return "Failed to find matchId".to_string();;
}


fn formateDate(mut date:String )->String{
    date = date.replacen("-"," ",3);
    date = date.replacen("T"," ",1);
    let foo:Vec<&str> = date.split(" ").collect();
    let mut month =  String::new();
    month = getMonth(foo[1].to_string());
    let day = formatDay(foo[2].to_string());
    let year = foo[0].to_string();
    return format!("{} {}, {}",month,day,year);
}

fn formatDay(day:String) -> String{
    match day.as_ref()
    {
        "01" => "1st".to_string(),
        "02" => "2nd".to_string(),
        "03" =>  "3rd".to_string(),
        "04" => "4th".to_string(),
        "05" => "5th".to_string(),
        "06" => "6th".to_string(),
        "07" => "7th".to_string(),
        "08" => "8th".to_string(),
        "9" => "9th".to_string(),
        _ => (day+"th").to_string(),
            }
}
fn getMonth(month:String) -> String {
    match month.as_ref()
    {
        "01" =>  "January",
        "02" =>  "February",
        "03" =>  "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" =>  "July",
        "08" =>  "September",
        "09" => "September",
        "10" =>  "October",
        "11" =>"November",
        "12" =>  "December",
        _ => "Unknown month Error",
    }.to_string()
}

fn getCurrentStage(stages:json::JsonValue)->usize{
    for i in 0..4
    {
        let stageToCheck = &stages[i];
        let stageMatchesAmount = stageToCheck["matches"].len() ;
        let lastMatchConclussion = &stageToCheck["matches"][stageMatchesAmount - 1]["state"];
        if(lastMatchConclussion != "CONCLUDED")
        {
            return i;
        }
    }
    return 1000000;
}