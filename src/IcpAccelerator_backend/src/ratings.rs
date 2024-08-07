use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::storage::{stable_save, self, stable_restore};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::caller;
use std::fmt;
use ic_cdk_macros::query;


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct RatingTypes {
    peer: Vec<TimestampedRating>,
    own: Vec<TimestampedRating>,
    mentor: Vec<TimestampedRating>,
    vc: Vec<TimestampedRating>,
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Rating {
    level_name: String,
    level_number: f64,
    sub_level: String,
    sub_level_number: f64,
    rating: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct RatingInternal{
    params: Vec<Rating>,
    timestamp: u64,
    current_role: String,
    project_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TimestampedRating {
    rating: Rating,
    timestamp: u64,
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RatingAverages {
    pub mentor_average: Vec<f64>,
    pub vc_average: Vec<f64>,
    pub peer_average: Vec<f64>,
    pub own_average: Vec<f64>,
    pub overall_average: Vec<f64>,
}

impl RatingAverages {
    pub fn new() -> Self {
        RatingAverages {
            mentor_average: Vec::new(),
            vc_average: Vec::new(),
            peer_average: Vec::new(),
            own_average: Vec::new(),
            overall_average: Vec::new(),
        }
    }
}

pub type RatingAverageStorage = HashMap<String, RatingAverages>;

pub type RatingSystem = HashMap<String, HashMap<Principal, Vec<(String, TimestampedRating)>>>;

type LastRatingTimestamps = HashMap<String, HashMap<Principal, u64>>;

thread_local! {
    pub static RATING_SYSTEM: RefCell<RatingSystem> = RefCell::new(RatingSystem::new());
    pub static LAST_RATING_TIMESTAMPS: RefCell<LastRatingTimestamps> = RefCell::new(LastRatingTimestamps::new());
    pub static AVERAGE_STORAGE: RefCell<RatingAverageStorage> = RefCell::new(RatingAverageStorage::new());
}

pub fn pre_upgrade_rating_system() {
    RATING_SYSTEM.with(|data| {
        match storage::stable_save((data.borrow().clone(),)) {
            Ok(_) => ic_cdk::println!("RATING_SYSTEM saved successfully."),
            Err(e) => ic_cdk::println!("Failed to save RATING_SYSTEM: {:?}", e),
        }
    });
    
    LAST_RATING_TIMESTAMPS.with(|data| {
        match storage::stable_save((data.borrow().clone(),)) {
            Ok(_) => ic_cdk::println!("LAST_RATING_TIMESTAMPS saved successfully."),
            Err(e) => ic_cdk::println!("Failed to save LAST_RATING_TIMESTAMPS: {:?}", e),
        }
    });
}

pub fn post_upgrade_rating_system() {
    match stable_restore::<(RatingSystem, LastRatingTimestamps)>() {
        Ok((restored_rating_system, restored_last_rating_timestamps)) => {
            RATING_SYSTEM.with(|data| *data.borrow_mut() = restored_rating_system);
            LAST_RATING_TIMESTAMPS.with(|data| *data.borrow_mut() = restored_last_rating_timestamps);
            ic_cdk::println!("Rating system modules restored successfully.");
        },
        Err(e) => ic_cdk::println!("Failed to restore rating system modules: {:?}", e),
    }
}


impl RatingTypes {
    fn new() -> Self {
        RatingTypes {
            peer: Vec::new(),
            own: Vec::new(),
            mentor: Vec::new(),
            vc: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseMainLevelError {}

impl fmt::Display for ParseMainLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid MainLevel")
    }
}


// impl FromStr for MainLevel {
//     type Err = ParseMainLevelError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Team" => Ok(MainLevel::Team),
//             "ProblemAndVision" => Ok(MainLevel::ProblemAndVision),
//             "ValueProp" => Ok(MainLevel::ValueProp),
//             "Product" => Ok(MainLevel::Product),
//             "Market" => Ok(MainLevel::Market),
//             "BusinessModel" => Ok(MainLevel::BusinessModel),
//             "Scale" => Ok(MainLevel::Scale),
//             "Exit" => Ok(MainLevel::Exit),
//             _ => Err(ParseMainLevelError {}),
//         }
//     }
// }


fn can_rate_again(system: &RatingSystem, project_id: &String, user_id: &Principal, current_timestamp: u64, interval: u64) -> bool {
    let last_rating_time = LAST_RATING_TIMESTAMPS.with(|timestamps| {
        let timestamps_borrowed = timestamps.borrow();
        if !timestamps_borrowed.contains_key(project_id) {
            ic_cdk::println!("Debug: No previous ratings found for project_id: {}", project_id);
        }
        timestamps_borrowed
            .get(project_id)
            .and_then(|project_map| {
                if !project_map.contains_key(user_id) {
                    ic_cdk::println!("Debug: No previous ratings found for user_id: {:?} in project_id: {}", user_id, project_id);
                }
                project_map.get(user_id)
            })
            .cloned()
    });

    match last_rating_time {
        Some(last_time) => {
            let can_rate_again = current_timestamp - last_time >= interval;
            ic_cdk::println!("Debug: Last rating time for user_id: {:?} in project_id: {} was at {}. Can rate again: {}", user_id, project_id, last_time, can_rate_again);
            can_rate_again
        },
        None => {
            ic_cdk::println!("Debug: No previous rating time found for user_id: {:?} in project_id: {}. User can rate.", user_id, project_id);
            true
        },
    }
}

fn update_last_rating_time(project_id: &String, user_id: &Principal, timestamp: u64) {
    LAST_RATING_TIMESTAMPS.with(|timestamps| {
        let mut timestamps = timestamps.borrow_mut();
        if timestamps.contains_key(project_id) {
            ic_cdk::println!("Debug: Found existing project map for project_id: {}", project_id);
        } else {
            ic_cdk::println!("Debug: Creating new project map for project_id: {}", project_id);
        }

        let project_map = timestamps.entry(project_id.clone()).or_insert_with(HashMap::new);

        if let Some(existing_timestamp) = project_map.get(user_id) {
            ic_cdk::println!("Debug: Updating existing timestamp for user_id: {:?} in project_id: {} from {} to {}", user_id, project_id, existing_timestamp, timestamp);
        } else {
            ic_cdk::println!("Debug: Inserting new timestamp for user_id: {:?} in project_id: {} as {}", user_id, project_id, timestamp);
        }

        project_map.insert(*user_id, timestamp);

        ic_cdk::println!("Debug: Updated timestamp for user_id: {:?} in project_id: {}. New timestamp: {}", user_id, project_id, timestamp);
    });
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RatingUpdate {
    project_id: String, 
    current_role: String, 
    ratings: Vec<Rating>,
}

pub fn update_rating_api(rating_data: RatingUpdate) -> String {
    if rating_data.ratings.is_empty() {
        ic_cdk::println!("Debug: No ratings provided.");
        return "No ratings provided, nothing updated.".to_string();
    }

    let principal_id = caller();
    ic_cdk::println!("Debug: Starting updates for Principal ID: {} with {} ratings for project ID: {}", principal_id, rating_data.ratings.len(), rating_data.project_id);

    let current_timestamp = time();
    let thirteen_days_in_seconds: u64 = 13 * 24 * 60 * 60 * 1000000000;
    let mut response_message = "No ratings updated.".to_string();
    let can_rate = RATING_SYSTEM.with(|system| {
        can_rate_again(&system.borrow(), &rating_data.project_id, &principal_id, current_timestamp, thirteen_days_in_seconds)
    });
    ic_cdk::println!("CAN RATE AGAIN FUNTION RETURNED {}", can_rate);

    if !can_rate {
        ic_cdk::println!("Debug: User cannot rate again due to the 13-day rule.");
        return "You cannot rate this project for the next 13 days".to_string();
    }

    RATING_SYSTEM.with(|system| {
        let mut system = system.borrow_mut();
        let project_ratings = system.entry(rating_data.project_id.clone()).or_insert_with(HashMap::new);
        let role_ratings = project_ratings.entry(principal_id).or_insert_with(Vec::new);
        
        for rating in rating_data.ratings {
            let new_rating = (rating_data.current_role.clone(), TimestampedRating {
                rating,
                timestamp: current_timestamp,
            });

            match rating_data.current_role.as_str() {
                "vc" => role_ratings.push(new_rating),
                "mentor" => role_ratings.push(new_rating),
                "project" => role_ratings.push(new_rating),
                _ => println!("Debug: Encountered unknown role: '{}'.", rating_data.current_role),
            }
            response_message = "Ratings updated successfully".to_string();
        }

        update_last_rating_time(&rating_data.project_id, &principal_id, current_timestamp);
    });

    response_message
}


fn round_to_one_decimal(value: f64) -> f64 {
    format!("{:.1}", value).parse::<f64>().unwrap()
}


pub fn calculate_average_api_storage(project_id: &str){

    let total_levels = 8;
    
    let mut averages = AVERAGE_STORAGE.with(|storage| {
        storage.borrow().get(project_id).cloned().unwrap_or_default()
    });

    RATING_SYSTEM.with(|system| {
        let system = system.borrow();
        if let Some(project_ratings) = system.get(project_id) {
            for (_, ratings) in project_ratings {
                let mut mentor_sum = 0.0;
                let mut mentor_count = 0;
                let mut vc_sum = 0.0;
                let mut vc_count = 0;
                let mut own_sum = 0.0;
                let mut own_count = 0;

                // Calculate cumulative sums and counts for each role
                for (role, timestamped_rating) in ratings {
                    let sub_level_num = timestamped_rating.rating.sub_level_number as f64;

                    match role.as_str() {
                        "mentor" => {
                            mentor_sum += sub_level_num;
                            mentor_count += 1;
                        },
                        "vc" => {
                            vc_sum += sub_level_num;
                            vc_count += 1;
                        },
                        "project" => {
                            own_sum += sub_level_num;
                            own_count += 1;
                        },
                        _ => (),
                    }
                }

                // Calculate and store new averages, appending to the history
                if mentor_count > 0 {
                    let new_mentor_average = mentor_sum / total_levels as f64;
                    averages.mentor_average.push(round_to_one_decimal(new_mentor_average));
                }
                if vc_count > 0 {
                    let new_vc_average = vc_sum / total_levels as f64;
                    averages.vc_average.push(round_to_one_decimal(new_vc_average));
                }
                if own_count > 0 {
                    let new_own_average = own_sum / total_levels as f64;
                    averages.own_average.push(round_to_one_decimal(new_own_average));
                }
            }

            // Calculate the overall weighted average from the latest values
            let latest_mentor = averages.mentor_average.last().unwrap_or(&0.0);
            let latest_vc = averages.vc_average.last().unwrap_or(&0.0);
            let latest_own = averages.own_average.last().unwrap_or(&0.0);
            let combined_mentor_vc = (latest_mentor + latest_vc) / 2.0;
            let new_overall_average = 0.6 * combined_mentor_vc + 0.4 * latest_own;
            averages.overall_average.push(round_to_one_decimal(new_overall_average));
        }
    });

    AVERAGE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(project_id.to_string(), averages);
    });
}



pub fn calculate_average_api(project_id: &str) -> RatingAverages{
    calculate_average_api_storage(project_id);
    AVERAGE_STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get(project_id).cloned().unwrap_or_default()
    })
}


#[query]
pub fn get_ratings_by_principal(project_id: String) -> Result<Vec<RatingView>, String> {
    let caller_id = caller();
    println!("Retrieving ratings for Principal ID: {} on project ID: {}", caller_id, project_id);

    let mut ratings_by_principal: Vec<RatingView> = Vec::new();

    let result = RATING_SYSTEM.with(|system| {
        let system = system.borrow();
        if let Some(project_ratings) = system.get(&project_id) {
            if let Some(role_ratings) = project_ratings.get(&caller_id) {
                for (role, timestamped_rating) in role_ratings {
                    let view = RatingView {
                        level_name: timestamped_rating.rating.level_name.clone(),
                        sub_level_name: timestamped_rating.rating.sub_level.clone(),
                        rating: timestamped_rating.rating.sub_level_number as u32,
                        timestamp: timestamped_rating.timestamp,
                    };
                    ratings_by_principal.push(view);
                }
                Ok(())
            } else {
                Err("No ratings found by this Principal ID for the specified project.".to_string())
            }
        } else {
            Err("Project ID not found in the rating system.".to_string())
        }
    });

    result.map(|_| ratings_by_principal)
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct RatingView {
    pub level_name: String,
    pub sub_level_name: String,
    pub rating: u32,
    pub timestamp: u64,
}



#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct MainLevels {
    id: i32,
    name: String,
}

pub fn get_main_levels() -> Vec<MainLevels> {
    vec![
        MainLevels {
            id: 1,
            name: "Team".to_string(),
        },
        MainLevels {
            id: 2,
            name: "Problem And Vision".to_string(),
        },
        MainLevels {
            id: 3,
            name: "Value Prop".to_string(),
        },
        MainLevels {
            id: 4,
            name: "Product".to_string(),
        },
        MainLevels {
            id: 5,
            name: "Market".to_string(),
        },
        MainLevels {
            id: 6,
            name: "Business Model".to_string(),
        },
        MainLevels {
            id: 7,
            name: "Scale".to_string(),
        },
        MainLevels {
            id: 8,
            name: "Exit".to_string(),
        },
    ]
}


#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct SubLevels {
    id: i32,
    name: String,
}

pub fn get_sub_levels() -> Vec<SubLevels> {
    vec![
        SubLevels {
            id: 1,
            name: "Establishing The Founding Team".to_string(),
        },
        SubLevels {
            id: 2,
            name: "Setting The Vision".to_string(),
        },
        SubLevels {
            id: 3,
            name: "Solidifying The Value Proposition".to_string(),
        },
        SubLevels {
            id: 4,
            name: "Validating An Investable Market".to_string(),
        },
        SubLevels {
            id: 5,
            name: "Proving A Profitable Business Model".to_string(),
        },
        SubLevels {
            id: 6,
            name: "Moving Beyond Early Adopters".to_string(),
        },
        SubLevels {
            id: 7,
            name: "Hitiing Product Market Fit".to_string(),
        },
        SubLevels {
            id: 8,
            name: "Scaling Up".to_string(),
        },
        SubLevels {
            id: 8,
            name: "Exit In Sight".to_string(),
        },
    ]
}
