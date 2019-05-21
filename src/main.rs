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

        return format!("There are no more matches today,check back in later");
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

fn getNextMatch()-> String{
      let liveMatchData = json::parse(&(reqwest::get("https://api.overwatchleague.com/schedule")?.text()?).to_string()).unwrap();
}