use std::string::ToString;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;
use crate::bot_utils;

fn get_plus_two() -> ReactionType {
    let plus_two: ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(924536822472802337),
        name: Some("p2".to_string()),
    };
    return plus_two;
}

fn get_minus_two() -> ReactionType {
    let minus_two:ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(924536784191365120),
        name: Some("m2".to_string()),
    };
    return minus_two;
}

fn get_manny() -> ReactionType {
    let manny: ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(929987409360343051),
        name: Some("manny".to_string())
    };
    return manny;

}
fn get_doot() -> ReactionType {
   let doot: ReactionType = ReactionType::Custom {
       animated: false,
       id: EmojiId::new(929985012554682469),
       name: Some("doot".to_string())
   } ;
    return doot;
}

fn get_winner() -> ReactionType {
    let winner: ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(1348181039779938366),
        name: Some("winner".to_string())
    } ;
    winner
}


pub fn get_emoji(emoji_name: &str) -> ReactionType {
    let current_env = bot_utils::get_env();
    if String::from("live").eq(&current_env) {
        match emoji_name {
            "minus_two" => get_minus_two(),
            "plus_two" => get_plus_two(),
            "winner" => get_winner(),
            _ => get_plus_two(),
        }
    } else {
        match emoji_name {
            "minus_two" => get_doot(),
            "plus_two" => get_manny(),
            "winner" => get_doot(),
            _ => get_manny(),
        }
    }
}
