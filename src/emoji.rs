use std::string::ToString;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;

fn get_plus_two() -> ReactionType {
    let plus_two: ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId(924536822472802337),
        name: Some("p2".to_string()),
    };
    return plus_two;
}

fn get_minus_two() -> ReactionType {
    let minus_two:ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId(924536784191365120),
        name: Some("m2".to_string()),
    };
    return minus_two;
}

fn get_manny() -> ReactionType {
    let manny: ReactionType = ReactionType::Custom {
        animated: false,
        id: EmojiId(929987409360343051),
        name: Some("manny".to_string())
    };
    return manny;

}
fn get_doot() -> ReactionType {
   let doot: ReactionType = ReactionType::Custom {
       animated: false,
       id: EmojiId(929985012554682469),
       name: Some("doot".to_string())
   } ;
    return doot;
}


pub fn get_emoji(emoji_name: &str) -> ReactionType {
    match emoji_name {
        "manny" => get_manny(),
        "doot" => get_doot(),
        "minus_two" => get_minus_two(),
        "plus_two" => get_plus_two(),
         _ => get_manny(),
    }
}