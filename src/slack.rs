use crate::{challonge::{self, get_participant_from_id}, Config};
use clap::Parser;
use reqwest;
use serde_json::json;

fn create_msg(mtc: &challonge::Match) -> String {
    let winner = get_participant_from_id(mtc.match_field.winner_id.as_i64().unwrap());
    let loser = get_participant_from_id(mtc.match_field.loser_id.as_i64().unwrap());
    let winner_emote = ":trophy:";
    let loser_emote = ":sob:";
    let score = mtc.match_field.scores_csv.clone();
    let msg = json!(
        {
            "text": format!("{} vs {} | {} :table_tennis_paddle_and_ball:",  winner.name, loser.name, score),
            "blocks": [
                {
                    "type": "header",
                    "text": {
                        "type": "plain_text",
                        "text": format!("{} vs {} | {} :table_tennis_paddle_and_ball:",  winner.name, loser.name, score),
                        "emoji": true
                    }
                },
                {
                    "type": "section",
                    "fields": [
                        {
                            "type": "mrkdwn",
                            "text": format!("{} *Winner*\n#{} {}", winner_emote, winner.seed, winner.name)
                        }
                    ]
                },
                {
                    "type": "section",
                    "fields": [
                        {
                            "type": "mrkdwn",
                            "text": format!("{} *Loser*\n#{} {}", loser_emote, loser.seed, loser.name)
                        }
                    ]
                },
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("Better luck next time {}!", loser.name)
                    },
                    "accessory": {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Tournament",
                            "emoji": true
                        },
                        "value": "click_me_123",
                        "url": "https://challonge.com/nonamecomp",
                        "action_id": "button-action"
                    }
                }
            ]
        }
    );
    serde_json::to_string(&msg).unwrap()
}

pub fn send_match_msg(latest_match: challonge::Match) -> String {
    let webhook = Config::parse().webhook;
    let message = create_msg(&latest_match);
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(webhook)
        .header("Content-Type", "application/json")
        .body(message)
        .send()
        .unwrap()
        .text()
        .unwrap();
    return res;
}
