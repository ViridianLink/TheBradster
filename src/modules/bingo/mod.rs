use std::collections::HashMap;

use async_trait::async_trait;
use rand::rng;
use rand::seq::IndexedRandom;
use serenity::all::{
    ActionRow, ActionRowComponent, ButtonKind, ButtonStyle, CommandInteraction,
    ComponentInteraction, Context, CreateActionRow, CreateButton, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
    EditInteractionResponse, ResolvedOption, UserId,
};
use serenity::prelude::TypeMapKey;
use sqlx::{PgPool, Postgres};
use zayden_core::{Component, SlashCommand};

use crate::{Error, Result};

const SPACES: [&str; 70] = [
    "Brad finds a red border chest room and spends way too much time in it",
    "Raid Exotic dropped for someone",
    "Falls off the map in exhibition with a relic",
    "Rhulk kicks somebody off the map",
    "Secret chest found",
    "5+ minutes spent in the lore puzzle room",
    "Brad missing his tcrash",
    "Challenge done at rulk",
    "Brad yells at the mods",
    "Brad says the team is throwing",
    "'Yeah ngl it's gonna be a looong raid'",
    "Finished an encounter without hints/Sherpa",
    "Falls during Caretaker",
    "'Clean dick' is said",
    "'Big Shaq boom'",
    "Dies by Screeb",
    "Create his own symbol call outs",
    "Bypass jumping puzzle",
    "5 minutes or more to open a friendship door",
    "1 second left time extension",
    "Wiped by enrage",
    "Failed a Final Stand",
    "Chat claims it's not blind",
    "Wrong glyph shot",
    "Brad vs number 1 hater",
    "Someone dies in transition",
    "Dies to Savathun in opening",
    "Wrong room in first",
    "Ask someone to shoot the nut when the door was open",
    "Pervading darkness kills Brad at Caretaker",
    "Hits immune with tcrash at Caretaker ",
    "Falls of the map in Exhibition",
    "Wrong call in Exhibition",
    "Doesn't cleanse teammates Exhibition",
    "Dies to the beam in Rhulk",
    "Dunks the wrong pillar",
    "Forgets to split",
    "Full blind clear 1 encounter",
    "Brad mispronounces Rhulk.",
    "Uses the left wall cheese at Exhibition",
    "Brad wears glasses",
    "Brad wears the wig",
    "Brad throws something across the room",
    "Cigarette crayon",
    "Voice crack",
    "Div used",
    "Brad rages and switches to bolt charge",
    "Argues about what a symbol should be called",
    "Brad screams about a relic to someone",
    "Death to Caretaker stomp",
    "Veteran leader is asked for a hint",
    "Brad talks about hating hunters",
    "Brad tells a warlock to get on well",
    "Time runs out in the last room of Exhibition",
    "Someone blows themself up",
    "Someone starts an encounter before the team is ready",
    "Someone names a symbol that doesn't exist",
    "'What symbol was it?'",
    "'I'm stuck!'",
    "'This doesn't seem that hard'",
    "Someone disconnects",
    "Brad screams, 'Where is the door?'",
    "Shield swipe into screebs on Exhibition",
    "Someone calls Rhulk hot",
    "'Short fuck'",
    "Someone dies to the forcefield at Rhulk",
    "Someone mentions feet",
    "Forgets to rally",
    "Misses a T Crash",
    "Someone misses a golden gun",
];

const TITLE: &str = "🎉 Bingo Card 🎉";
const DESCRIPTION: &str = "Below is the key information for taking part in the event. If you have any further questions please contact a mod in the Discord server.";
const YOUR_CARD: &str = "- Your bingo card is randomly generated.
- Below this message you'll see a 5x5 grid of clickable buttons.
- The center square is a FREE SPACE - you can click that button right away to mark it!";
const HOW_TO_PLAY: &str = "- As I stream, watch and listen carefully!
- Your bingo card is filled with specific actions I might do or phrases I'm likely to say.
- If you see me do an action or hear me say a phrase that matches one of the squares on your card, click the corresponding button on your card to mark it. It should turn green to show it's selected!";
const HOW_TO_WIN: &str = "- To win, you need to be the first to complete a winning pattern by clicking the correct buttons on your card.
- When your clicks complete a row, column, diagonal, or a full board, the bot will notify the Mod Team with your card to be verified.
- Once a Mod confirms your win, I'll announce you as the BINGO winner on stream! 📢
- Incorrect BINGO will be discarded so, focus on accurately clicking your card!";
const NOTES: &str = "- Your card is randomly generated from a list of spaces created by the Mod team.
- Accurate Clicks Only: Please only click a square if the action/phrase has actually happened on stream. Mods will verify every BINGO!
- Prizes: Today's winner(s) will get {PRIZE_STR}! 🏆
- Ties: If the bot detects multiple BINGOs from the same action/phrase simultaneously, the winner will be the first person the bot registered and sent to the mods.
- Your first click will highlight a square (aka a button) in blue. The second click will then lock it in — that means the item has been confirmed and the button will turn green.
- Once a button is green, there's no going back — if it's a false mark, your card will become null and void.
- Highlighting a button in blue can help you keep track — for example, if Brad enters the third encounter and you know a square can only happen there, you can highlight it to help remember. It's also a safety feature to prevent misclicks.";

pub struct Bingo;

#[async_trait]
impl SlashCommand<Error, Postgres> for Bingo {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        {
            let data = ctx.data.read().await;
            if let Some(messages) = data.get::<BingoMessages>() {
                if messages.contains_key(&interaction.user.id) {
                    return Err(Error::BingoCardAlreadySent);
                }
            }
        }

        // SPACES
        //     .iter()
        //     .filter(|space| space.len() > 80)
        //     .for_each(|space| println!("Warning: Space '{space}' is longer than 80 characters"));

        let embed = CreateEmbed::new()
            .title(TITLE)
            .description(DESCRIPTION)
            .field("Your card", YOUR_CARD, false)
            .field("How to play", HOW_TO_PLAY, false)
            .field("How to Win", HOW_TO_WIN, false)
            .field("Important Notes", NOTES, false);

        let msg = interaction
            .user
            .direct_message(
                ctx,
                CreateMessage::new().embed(embed).components(components()),
            )
            .await
            .unwrap();

        {
            let mut data = ctx.data.write().await;
            data.entry::<BingoMessages>()
                .or_insert_with(HashMap::new)
                .insert(interaction.user.id, msg.components);
        }

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content("Bingo card and instructions have been sent via DM."),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("bingo").description("Test Command");

        Ok(cmd)
    }
}

#[async_trait]
impl Component<Error, Postgres> for Bingo {
    async fn run(ctx: &Context, interaction: &ComponentInteraction, _pool: &PgPool) -> Result<()> {
        let mut components = interaction.message.components.clone();

        let changed = update_button(&mut components, &interaction.data.custom_id);
        let condition = if changed {
            let win_state: HashMap<WinCondition, bool> = {
                let data = ctx.data.read().await;
                data.get::<BingoWinState>()
                    .expect("WinState should be present")
                    .clone()
            };

            check_grid_conditions(&components, &win_state)
        } else {
            None
        };

        println!("{:?}", condition);

        {
            let mut data = ctx.data.write().await;
            let messages = data.get_mut::<BingoMessages>().unwrap();
            messages.insert(interaction.user.id, components.clone());
        }

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new().components(
                        components
                            .into_iter()
                            .map(|row| {
                                let buttons = row
                                    .components
                                    .into_iter()
                                    .map(|component| match component {
                                        ActionRowComponent::Button(button) => {
                                            CreateButton::from(button)
                                        }
                                        _ => unreachable!(),
                                    })
                                    .collect::<Vec<_>>();

                                CreateActionRow::Buttons(buttons)
                            })
                            .collect(),
                    ),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }
}

struct BingoMessages;

impl TypeMapKey for BingoMessages {
    type Value = HashMap<UserId, Vec<ActionRow>>;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum WinCondition {
    FullHouse,
    Row,
    Column,
    Diagonal,
    OneLine,
    TwoLines,
    ThreeLines,
    FourLines,
}

struct BingoWinState;

impl TypeMapKey for BingoWinState {
    type Value = HashMap<WinCondition, bool>;
}

fn components() -> Vec<CreateActionRow> {
    let mut rand_spaces = SPACES
        .choose_multiple(&mut rng(), 24)
        .map(|label| label.chars().take(80).collect::<String>());

    let mut components = Vec::with_capacity(5);
    for r in 0..5 {
        let mut row = Vec::with_capacity(5);

        for c in 0..5 {
            let button = CreateButton::new(format!("bingo_{r}{c}")).style(ButtonStyle::Secondary);

            let label = if r == 2 && c == 2 {
                String::from("FREE SPACE")
            } else {
                rand_spaces.next().unwrap()
            };

            row.push(button.label(label));
        }

        components.push(CreateActionRow::Buttons(row));
    }

    components
}

fn get_button_style(grid: &[ActionRow], r: u8, c: u8) -> ButtonStyle {
    match &grid[r as usize].components[c as usize] {
        ActionRowComponent::Button(button) => match &button.data {
            ButtonKind::NonLink { style, .. } => *style,
            _ => unreachable!("Expected NonLink button data at ({}, {})", r, c),
        },
        _ => unreachable!("Expected Button component at ({}, {})", r, c),
    }
}

fn update_button(components: &mut [ActionRow], button_id: &str) -> bool {
    let (r, c) = button_id
        .strip_prefix("bingo")
        .unwrap()
        .split_once("")
        .unwrap();

    let ActionRowComponent::Button(button) =
        &mut components[r.parse::<usize>().unwrap()].components[c.parse::<usize>().unwrap()]
    else {
        unreachable!("Component must be a button")
    };

    match &mut button.data {
        ButtonKind::NonLink { custom_id, style } => {
            if custom_id.as_str() == button_id {
                if *style == ButtonStyle::Primary {
                    *style = ButtonStyle::Success;
                    return true;
                }

                if *style == ButtonStyle::Secondary {
                    *style = ButtonStyle::Primary;
                }

                return false;
            }
        }
        _ => unreachable!("ButtonKind must be NonLink"),
    }

    false
}

#[derive(Debug)]
pub enum GridCondition {
    RowSuccess(u8),
    ColumnSuccess(u8),
    MainDiagonalSuccess,
    AntiDiagonalSuccess,
    FullGridSuccess,
}

fn check_grid_conditions(
    grid: &[ActionRow],
    _win_states: &HashMap<WinCondition, bool>,
) -> Option<GridCondition> {
    const GRID_SIZE: u8 = 5;

    // 1. Check for any full row of ButtonStyle::Success
    for r_idx in 0..GRID_SIZE {
        if (0..GRID_SIZE).all(|c_idx| get_button_style(grid, r_idx, c_idx) == ButtonStyle::Success)
        {
            return Some(GridCondition::RowSuccess(r_idx));
        }
    }

    // 2. Check for any full column of ButtonStyle::Success
    for c_idx in 0..GRID_SIZE {
        if (0..GRID_SIZE).all(|r_idx| get_button_style(grid, r_idx, c_idx) == ButtonStyle::Success)
        {
            return Some(GridCondition::ColumnSuccess(c_idx));
        }
    }

    // 3. Check diagonals
    let n = GRID_SIZE; // Size of the square grid

    // Main diagonal (top-left to bottom-right)
    if (0..n).all(|i| get_button_style(grid, i, i) == ButtonStyle::Success) {
        return Some(GridCondition::MainDiagonalSuccess);
    }

    // Anti-diagonal (top-right to bottom-left)
    if (0..n).all(|i| get_button_style(grid, i, n - 1 - i) == ButtonStyle::Success) {
        return Some(GridCondition::AntiDiagonalSuccess);
    }

    let mut all_grid_is_success = true;
    for r_idx in 0..GRID_SIZE {
        for c_idx in 0..GRID_SIZE {
            if get_button_style(grid, r_idx, c_idx) != ButtonStyle::Success {
                all_grid_is_success = false;
                break;
            }
        }
        if !all_grid_is_success {
            break;
        }
    }
    if all_grid_is_success {
        return Some(GridCondition::FullGridSuccess);
    }

    None
}
