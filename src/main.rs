/*
David Wolak
CS 410P Rust Programming
My program is a CLI app designed to get quick and easy information about the overwatch league without having to go its site which tends to be bloaty and can take long to load.

I turned off warnings for snakecase since the api calls use camel case and there was a lot of warnings
Unused assigments is turned off because there are many times where I assign something so I dont have to create the object multiple times in different parts of a if statement
*/
#![allow(non_snake_case)]
#![allow(unused_assignments)]
extern crate chrono;
extern crate json;
extern crate reqwest;
use std::io;

fn main() -> Result<(), Box<std::error::Error>> { //Main program is basiclly the menu, prints out a quick blurb about the next match before prompting a menu.
    println!("Welcome to the Overwatch League Tracker! While we're unfornately still terminal based theres plan to change that soon so stay tuned!");
    println!("--------------");
    let _error = print_current_and_next_match(); //Has it assigned to a unused varible because the compiler was angry about it. Serves no purpose
    let mut user_choice = 1000000000;
    while user_choice != 4 { //4 means the user wants to quit
    user_choice = 1000000000;
        while user_choice == 1000000000 { //Until a valid option is present it'll keep looping
            user_choice = menu_choice();
        }
        match user_choice { //Pairs the user choice with the correct function to print the correct information.
            1 => {
                let next_match = get_next_match().unwrap();//These all need unwraps because the https calls return results forcing the funciton to return a result too.
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
            _ => println!("Error:Invalid selections"),//This shouldn't happen but if it does your covered
        };
        
    }
    Ok(())
}

fn get_all_rankings() -> Result<(String), Box<std::error::Error>> {//This gets and displays all the teams and their league rank
    let rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();//Does the http request and gets a json back with the rankings

    let rankings = rankings["content"].clone(); //Clones the data because otherwise I was getting funky behavoir
    let amt_competitors = rankings.len();
    let mut result = "Current Overwatch League Rankings:".to_string();//Just a introductory sentence 
    let mut team_name = String::new();//team name and rank are declared outside the for loop, the compiler gets mad but I think this makes it easier to manupulate the memory
    let mut rank = String::new();
    for i in 0..amt_competitors { //This for loop gets the data and puts it in a string thats formated with each line showing the team name and their rank
        team_name = rankings[i]["competitor"]["name"].to_string();
        rank = rankings[i]["placement"].to_string();
        result = format!("{}\n {}:  Rank {}", result, team_name, rank);
    }
    Ok(result)//If all is good returns Ok
}

fn get_team_info() -> Result<(String), Box<std::error::Error>> {//This function gets some basic info on each team like their current win to loss in matches and maps as well as their rank.
// Due to the API call being bugged this function is also bugged. All stats returned at time of writing this comment are the exact same.
    let team_ids = [
        7698, 4402, 7692, 4523, 4407, 7699, 7693, 4525, 4410, 4406, 4405, 4403, 7694, 4524, 4404,
        4409, 4408, 7695, 7696, 7697,
    ];//This is hardcoded and not my favorite thing but I couldnt figure a better way to grab all the id without a extra API call
    println!("Select which team you like to know more about:\n");//Prompt + lists out the teams. TIL rust will print things out just like you write them!
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
    let mut api_url = String::new();//This will store the final api url to be called
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();//This trims the string down to the acutal input
    let input = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1000000000,
    };
    if input <= 0 || input > 20 {//Checks to make sure the number is valid 
        return Ok("That is not a valid team".to_string());
    }
    api_url = format!(
        "https://api.overwatchleague.com/teams/{}",
        team_ids[(input - 1) as usize].to_string()
    ); //Creates the url needed to call the correct teams information. The info back is bugged as the map stats are all the same regardless of team

    let team_info = json::parse(&(reqwest::get(&api_url[..])?.text()?).to_string()).unwrap();//Makes the call and turns it into a string
    let team_name = team_info["description"].to_string();//The next couple lines just grab the information and get the rankings
    let team_rank = get_ranking(team_name.clone()).unwrap();
    let match_wins = team_info["ranking"]["matchWin"].to_string();
    let match_loss = team_info["ranking"]["matchLoss"].to_string();
    let match_draw = team_info["ranking"]["matchDraw"].to_string();
    let game_win = team_info["ranking"]["gameWin"].to_string();
    let game_loss = team_info["ranking"]["gameLoss"].to_string();
    let game_tie = team_info["ranking"]["gameTie"].to_string();

    let result = format!(
        "{} is currently {} place in the league.
    During the current/last stage they had {} match wins,{} match losses, and {} match ties.
    During that time they had won {} maps, lost {} maps, and tied on {} maps",
        team_name, team_rank, match_wins, match_loss, match_draw, game_win, game_loss, game_tie
    );//Creates the string that gets printed out.

    Ok(result)
}

fn print_current_and_next_match() -> Result<(), Box<std::error::Error>> {//Gets the information for the current(live if there is one) and next match(regardless if there is a live match)
    let live_match_data = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/live-match")?.text()?).to_string(),
    )
    .unwrap();
    let live_match = live_match_data["data"]["live_match"].clone();//Live match comes with both next and live match! Very convienent
    let next_match = live_match_data["data"]["next_match"].clone();

    let current_match = get_current_match(live_match);//Sends the data off to be processed. Splitting it like this means one API call for both live and current
    println!("{}", current_match);
    let next_match = get_todays_next_match(next_match);//Process info the next match for today
    println!("{}", next_match);
    Ok(())
}

fn menu_choice() -> u32 {//Just a basic menu
    println!("What would you like to do:");
    println!("1. Get the next match");
    println!("2. Get more information about a team");
    println!("3. See the league ranks");
    println!("4. Close the app");
    println!("Enter your choice of what you would like to do:");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");//Acutally gets the input

    let trimmed = input_text.trim();//Trims the garbage
    match trimmed.parse::<u32>() {//Turns it into either a number or errors and returns a huge number
        Ok(i) => i,
        Err(..) => 1000000000,
    }
}

fn get_current_match(live_match: json::JsonValue) -> String {//One of the chunkier function, splits the information of live match and makes it useful instead of a gaint JSON
    let first_contendor = (&live_match["competitors"][0]["name"]).to_string();//Gets the name/score of the two contendors in the match
    let second_contendor = (&live_match["competitors"][1]["name"]).to_string();
    let first_contendor_score = (&live_match["scores"][0]["value"]).to_string();
    let second_contendor_score = (&live_match["scores"][1]["value"]).to_string();
    let round = (&live_match["round"]).to_string();//Gets what round the match is one, then what map the round is, then checks to see if the match is even live.
    let map = (&live_match["games"][2]["attributes"]["map"]).to_string();
    let current_state = (&live_match["liveStatus"]).to_string();
    let mut result = String::new();//Declaring some varibles
    let mut current_winner = String::new();
    let mut leader_score = String::new();
    let mut loser_score = String::new();

    if current_state == "LIVE" {//Its possible for the live-match to contain stuff but not be live so we need to check. If it is we need to figure out whos winning and input data correctly
        if first_contendor_score == second_contendor_score {//If their tied
            current_winner = ("the teams being tied").to_string();
            leader_score = first_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        } else if first_contendor_score > second_contendor_score {//If the first team is winning
            current_winner = first_contendor.clone();
            leader_score = first_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        } else {
            current_winner = second_contendor.clone();//If the second team is winning
            leader_score = second_contendor_score.clone();
            loser_score = second_contendor_score.clone();
        }
        result = format!(
            "The current match is {} vs {} on {} round {} \n {} is currently winning with {} points to {}",
            first_contendor, second_contendor,map,round, current_winner, leader_score, loser_score
        );//Creates a string to return and print
    } else {//If the match is not currently live we need to figure out when it wil be and who
        let time_to_match_string = (&live_match["timeToMatch"]).to_string();//Since we might not acutally need it we only bother grabbing the time to match if we know
        if (time_to_match_string != "null") && (first_contendor != "null") & (second_contendor != "null")//If the api is bugged and gives null data(it happens a lot not a amazing api) it doesnt do anything
        {
            let time_to_match_float = time_to_match_string.parse::<f64>().map(|n| n + 1.5).unwrap();//If there is a time to match it needs to be converted into a f64 
            let time_to_next_converted = convert_milli_seconds(time_to_match_float);//Then passed off to a helper function to figure out how lnog that acutally is
            result = format!(
                "The next match is {} agasint {} in {}",
                first_contendor, second_contendor, time_to_next_converted
            );
        }
    }
    let next_match = get_next_match().unwrap();//Since next match usually isnt acutally in the live match its easier to grab the next match on the schedule this way
    return format!("{} \n {} ", result, next_match);
}

fn get_todays_next_match(next_match: json::JsonValue) -> String {//If there is another match today it runs this!
    let next_first_contendor = (&next_match["competitors"][0]["name"]).to_string();//Grabs basic info
    let next_second_contendor = (&next_match["competitors"][1]["name"]).to_string();
    if next_first_contendor != "null" && next_second_contendor != "null" {//Checking to see if the api is returning valid data
        let first_contendor_ranking = get_ranking(next_first_contendor.to_string()).unwrap();//If it is it gets the rankings of both teams
        let second_contendor_ranking = get_ranking(next_second_contendor.to_string()).unwrap();

        return format!(
            "The next match will be between {}(League Rank {}) and {}(League Rank {})",
            next_first_contendor.to_string(),
            first_contendor_ranking.to_string(),
            next_second_contendor.to_string(),
            second_contendor_ranking.to_string()
        );//Then it formats then into a good readable sentence
    } else {
        return format!("There are no more matches today,check back in tomorrow!");//Otherwise gives a nice farewell
    }
}
fn get_ranking(team: String) -> Result<(usize), Box<std::error::Error>> {//Grabs the ranking of a team from the api
    let rankings = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/ranking")?.text()?).to_string(),
    )
    .unwrap();//Makes the call

    let mut position = 0;
    let mut found = false;
    let mut compare_to = &rankings["content"][position]["competitor"]["name"];//The first team to get checked! 
    while position < 21 && found == false {//This just loops thourgh the json until it finds the team it was asked to
        compare_to = &rankings["content"][position]["competitor"]["name"];
        if team == compare_to.to_string() {
            found = true;//If it does find the team it breaks the loop and returns the current position + 1 since arrays are 0 indexed
        } else {
            position = position + 1;
        }
    }
    return Ok(position + 1);
}

fn convert_milli_seconds(time_to_next: f64) -> String { //Helper function. Larger then you think so it got exported into its own thing
    if time_to_next > 60000.0 {//If its more then a minute I want to know how acutaly long it is. Otherwise I dont care.
        let converted = time_to_next / (3.6 * (1000000.0));
        let hours = converted.floor();
        let minutes = 60.0 * (converted - hours);
        if hours > 1.0 {
            format!("{:.0} hours and {:.0} minutes", hours, minutes)
        } else {
            format!("{:.0} minutes, time to get hyped!", minutes)
        }
    } else if (time_to_next <= 60000.0) && (time_to_next > 0.0) {//Less then a minute  then I just declare that, you should be getting to your monitor at this point anyways
        format!("Match is starting in less then a minute!")
    } else if time_to_next < 0.0 {//Shouldnt ever happen but I dont trust this API with much
        format!("A error has occured getting the time until next match")
    } else {//If its somehow 0(shouldnt be) then i assume the match has started and is on going
        format!("The match has started!")
    }
}

fn get_next_match() -> Result<(String), Box<std::error::Error>> {//Grabs the next match period, not just next in the day
/*
    Im really happy with how this turned out as it took a lot of thought. The api call grabs ALL the matches in the entire season with no way to narrow it down.
    This is 550 odd matches that I would have to search thourgh. However each season is divded into 4 stages.
    I find out what stage is active in a max of 4 searchs by looking at the last match of each stage.
    Once I find out what stage its in I look at the last match of each day going forward. If the last match is completed then  the day is completed and I can check the next one.
    So instead of looking thourgh potinetally 549 matches to find the one I want I only need to do a worse case of about ~29! Saving me tons of time and searches
*/
    let schedule = json::parse(
        &(reqwest::get("https://api.overwatchleague.com/schedule")?.text()?).to_string(),
    )
    .unwrap();//Grabs the schedule of all 550 odd matches. There is no better way to do this
    let current_stage = get_current_stage(schedule["data"]["stages"].clone());//Figures out the current stage to not look thourgh stages that have finished or havent started. Does so in 4 searches MAX
    let match_id = get_next_match_in_schedule(schedule["data"]["stages"][current_stage].clone());//Gets the acutal next match in about ~25 searches.
    return Ok(match_id);//Returns just the id of the next match
}

fn get_next_match_in_schedule(stage: json::JsonValue) -> String {//Im pretty proud of this one acutally. If I was searching thourgh all the matches it could be real bad but this allows me to skip around on basic assumptions to find them faster
    let matches = &stage["matches"];
    let number_of_matches = matches.len();
    for mut i in 0..number_of_matches {//Loops thourgh all the matches
        if matches[i + 3]["state"] == "CONCLUDED"//Theres 3 matches per day, if I looked at every 3rd I can concluded if the day is finished. If it finished no reason to look thourgh the rest of the day so I skip them.
        {
            i = i + 3;
        } else {//If the 3rd match is not done then is has to look
            if matches[i]["state"] == "PENDING" {//The match should be pending, otherwise it assumes the api screwed up and keeps going
                let first_contendor = (&matches[i]["competitors"][0]["name"]).to_string();//Grabs some basic info
                let second_contendor = (&matches[i]["competitors"][1]["name"]).to_string();
                let first_contendor_ranking = get_ranking(first_contendor.clone()).unwrap();
                let second_contendor_ranking = get_ranking(second_contendor.clone()).unwrap();
                let mut start_date = (&matches[i]["startDate"]).to_string();
                if (start_date != "null") && (first_contendor != "null") & (second_contendor != "null")//Makes sure tha the info is valid
                {
                    start_date = format_date(start_date);//Calls a helper function to format the data into human readable langauge

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

fn format_date(mut date: String) -> String {//Takes a string like 01-01-2019T0.00.00000 and turns it into something more readable. Also has the benefit of being easier to understand if your not from a place that formats the date the same
    date = date.replacen("-", " ", 3);//Replaces all the spaces and T's with spaces
    date = date.replacen("T", " ", 1);
    let foo: Vec<&str> = date.split(" ").collect();//Splits it up into a vector, in theory it should have a month,day, and year entry
    let mut month = String::new();
    month = get_month(foo[1].to_string());//Helper function to get the month
    let day = format_day(foo[2].to_string());//Helper function get the day
    let year = foo[0].to_string();//Just makes the year into a string
    return format!("{} {}, {}", month, day, year);//Formats it into MM,DD, YYYY
}

fn format_day(day: String) -> String {//Given a number it returns that number as a day
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
fn get_month(month: String) -> String {//Given a number it returns the correct month or a error
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
fn get_current_stage(stages: json::JsonValue) -> usize {//Checks all four current stages to find the active one
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
/*
############################################################################
#                               Tests                                      #
#                                                                          #
#                                                                          #
############################################################################

These tests arent great. Since I could not figure out how any mocking crates worked for this(I thought I figured it out but it didn't mock correctly!?)
And I also wasn't able to create test jsons 
And since the data given is mostly dynamic so I will rarely get the same result for less then a week
I could only test functions that didn't require either jsons or https calls, and those are.....limited
*/

#[cfg(test)]
mod tests {
    use crate::convert_milli_seconds;//List out the only four functions I could figure out how to test
    use crate::format_date;
    use crate::format_day;
    use crate::get_month;
    #[test]
    fn test_milli_second_conversion() {//Test the milli second conversions
        let valid_time = convert_milli_seconds(1002313231.0);//Gives it a random valid number
        let valid_time_2 = convert_milli_seconds(30000.0);//Gives it 30s 
        let valid_time_3 = convert_milli_seconds(1.0);//Gives it one second
        let valid_time_4 = convert_milli_seconds(0.0);//Gives it 0 seconds
        let invalid_time = convert_milli_seconds(-10231.0);//Gives it a invalid number that should cause a error
        assert_eq!(valid_time, "278 hours and 25 minutes");
        assert_eq!(valid_time_2, "Match is starting in less then a minute!");
        assert_eq!(valid_time_3, "Match is starting in less then a minute!");
        assert_eq!(valid_time_4, "The match has started!");
        assert_eq!(
            invalid_time,
            "A error has occured getting the time until next match"
        );
    }

    #[test]
    fn format_date_test() {//Test the date formatting
        let valid_time_stamp = format_date("2019-02-15T00:00:00.000Z".to_string());//Gives it a valid timestamp to test
        let invald_time_stamp = format_date("Invalid TimeStamp".to_string());//Gives it a invalid timestamp to test
        assert_eq!(valid_time_stamp, "February 15th, 2019");
        assert_eq!(invald_time_stamp, "Unknown month Error imeStampth, Invalid");
    }

    #[test]
    fn format_day_test() {//Throws random data after 1-3 to verify its adding th
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
    fn test_get_month() {//Verifies the first month, July, and then makes sure errors on a invalid month
        let jan = get_month("01".to_string());
        let july = get_month("07".to_string());
        let invalid_month = get_month("123210".to_string());
        assert_eq!(jan, "January".to_string());
        assert_eq!(july, "July".to_string());
        assert_eq!(invalid_month, "Unknown month Error".to_string());
    }
}
