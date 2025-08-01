mod confirm;
mod model;
use chrono::Utc;
pub use confirm::BingoConfirm;

mod spaces;
use spaces::SPACES;

use std::str::FromStr;

use async_trait::async_trait;
use rand::rng;
use rand::seq::IndexedRandom;
use serenity::all::{
    ActionRow, ActionRowComponent, AutoArchiveDuration, ButtonKind, ButtonStyle, ChannelId,
    ChannelType, CommandInteraction, CommandOptionType, ComponentInteraction, Context,
    CreateActionRow, CreateAttachment, CreateButton, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, CreateThread, EditInteractionResponse, Mentionable, ResolvedOption,
    ResolvedValue, RoleId,
};
use serenity::prelude::TypeMapKey;
use sqlx::{PgPool, Postgres};
use zayden_core::{Component, SlashCommand};

use crate::modules::bingo::model::{BingoRow, BingoTable};
use crate::{Error, Result};

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
- Prizes: Winner(s) will get to pick from a selection of Destiny 2 Emblems! 🏆
- Ties: If the bot detects multiple BINGOs from the same action/phrase simultaneously, the winner will be the first person the bot registered and sent to the mods.
- Your first click will highlight a square (aka a button) in blue. The second click will then lock it in — that means the item has been confirmed and the button will turn green.
- Once a button is green, there's no going back — if it's a false mark, your card will become null and void.
- Highlighting a button in blue can help you keep track — for example, if Brad enters the third encounter and you know a square can only happen there, you can highlight it to help remember. It's also a safety feature to prevent misclicks.";

const GRID_SIZE: u8 = 5;

pub fn register(ctx: &Context) -> [CreateCommand; 2] {
    [
        Bingo::register(ctx).unwrap(),
        BingoConfirm::register(ctx).unwrap(),
    ]
}

pub struct Bingo;

#[async_trait]
impl SlashCommand<Error, Postgres> for Bingo {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut row = match BingoTable::row(pool, interaction.user.id).await {
            Ok(Some(row)) if row.day == Utc::now().date_naive() => {
                return Err(Error::BingoCardAlreadySent)
            }
            Ok(Some(row)) => row,
            _ => BingoRow::new(interaction.user.id),
        };

        SPACES
            .iter()
            .filter(|space| space.len() > 38) // 38 recommended, 80 max
            .for_each(|space| println!("Warning: Space '{space}' is longer than 38 characters"));

        let format = match options.pop().map(|opt| opt.value) {
            Some(ResolvedValue::String(format)) => format,
            _ => "small",
        };

        let info_embed = CreateEmbed::new()
            .title(TITLE)
            .description(DESCRIPTION)
            .field("Your card", YOUR_CARD, false)
            .field("How to play", HOW_TO_PLAY, false)
            .field("How to Win", HOW_TO_WIN, false)
            .field("Important Notes", NOTES, false);

        interaction
            .user
            .direct_message(ctx, CreateMessage::new().embed(info_embed))
            .await?;

        let spaces = rand_spaces();

        row.spaces = spaces.clone();

        interaction
            .user
            .direct_message(
                ctx,
                CreateMessage::new()
                    .embed(live_embed(None, spaces.clone()))
                    .components(components(format, spaces)),
            )
            .await?;

        BingoTable::insert(pool, row).await.unwrap();

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
        let cmd = CreateCommand::new("bingo")
            .description("PLACEHOLDER")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "style", "PLACEHOLDER")
                    .add_string_choice("New", "small")
                    .add_string_choice("Old", "big"),
            );

        Ok(cmd)
    }
}

#[async_trait]
impl Component<Error, Postgres> for Bingo {
    async fn run(ctx: &Context, interaction: &ComponentInteraction, pool: &PgPool) -> Result<()> {
        let mut components = interaction.message.components.clone();

        let spaces = match BingoTable::spaces(pool, interaction.user.id).await {
            Ok(Some(spaces)) => spaces,
            _ => interaction
                .message
                .as_ref()
                .embeds
                .first()
                .unwrap()
                .fields
                .last()
                .unwrap()
                .value
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| &s[3..])
                .map(String::from)
                .collect::<Vec<_>>(),
        };

        let changed = update_button(&mut components, &interaction.data.custom_id);
        let (condition, labels) = if changed {
            let win_state = {
                let data = ctx.data.read().await;
                data.get::<BingoWinState>()
                    .expect("WinState should be present")
                    .clone()
            };

            check_grid_conditions(&components, &win_state)
        } else {
            (None, Vec::new())
        };

        if let Some(condition) = condition {
            const CHANNEL_ID: ChannelId = ChannelId::new(1328070131380781189);
            const TWITCH_MODS: RoleId = RoleId::new(1275149982701191260);
            const DISCORD_MODS: RoleId = RoleId::new(1275143477654454394);

            let thread = CHANNEL_ID
                .create_thread(
                    ctx,
                    CreateThread::new(format!(
                        "BINGO - {} - {condition:?}",
                        interaction.user.display_name()
                    ))
                    .kind(ChannelType::PrivateThread)
                    .auto_archive_duration(AutoArchiveDuration::OneWeek),
                )
                .await
                .unwrap();

            let spaces = labels
                .into_iter()
                .map(|label| space_from_label(label, &spaces))
                .map(|space| format!("{}\n", space))
                .collect::<String>();

            let embed = CreateEmbed::new()
                .title("BINGO")
                .field("Win Condition", format!("{condition:?}"), false)
                .field("Values", spaces, false)
                .field("Bingo Card", emoji_card(Some(&components)), false)
                .footer(CreateEmbedFooter::new(
                    "Use `/bingoconfirm` to accept this win.",
                ));

            let button = CreateButton::new("support_close")
                .label("Close")
                .style(ButtonStyle::Primary);

            thread
                .send_message(
                    ctx,
                    CreateMessage::new()
                        .content(format!(
                            "{} {}\nPlease verify the following BINGO card from {}",
                            TWITCH_MODS.mention(),
                            DISCORD_MODS.mention(),
                            interaction.user.mention()
                        ))
                        .embed(embed)
                        .button(button),
                )
                .await
                .unwrap();

            let file =
                CreateAttachment::bytes(pretty_print_card(&components).as_bytes(), "bingocard.txt");

            thread
                .send_message(ctx, CreateMessage::new().files([file]))
                .await
                .unwrap();
        }

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(live_embed(Some(&components), spaces))
                        .components(
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

fn space_from_label(label: String, spaces: &[String]) -> &str {
    if label == "FREE" {
        return "FREE";
    }

    if label.len() != 2 {
        return spaces
            .iter()
            .find(|&space| space.starts_with(&label))
            .unwrap();
    }

    let mut chars = label.chars();
    let r_char = chars.next().unwrap();
    let c_char = chars.next().unwrap();

    if let (Some(r), Some(c)) = (r_char.to_digit(10), c_char.to_digit(10)) {
        let mut index = r * 5 + c;

        if r > 2 || (r == 2 && c > 1) {
            index -= 1;
        }

        spaces.get(index as usize).unwrap()
    } else {
        spaces
            .iter()
            .find(|&space| space.starts_with(&label))
            .unwrap()
    }
}

pub struct BingoWinState;

impl TypeMapKey for BingoWinState {
    type Value = Vec<GridCondition>;
}

fn rand_spaces() -> Vec<String> {
    SPACES
        .choose_multiple(&mut rng(), 24)
        .map(|label| label.chars().take(80).collect::<String>())
        .collect()
}

fn components(format: &str, mut spaces: Vec<String>) -> Vec<CreateActionRow> {
    let mut components = Vec::with_capacity(5);

    for r in 0..5 {
        let mut row = Vec::with_capacity(5);

        for c in 0..5 {
            let mut button =
                CreateButton::new(format!("bingo_{r}{c}")).style(ButtonStyle::Secondary);

            button = match format {
                _ if r == 2 && c == 2 => button.emoji('🆓'),
                "big" => button.label(spaces.pop().unwrap()),
                "small" => button.label(format!("{r}{c}")),
                _ => unreachable!("Invalid format"),
            };

            row.push(button);
        }

        components.push(CreateActionRow::Buttons(row));
    }

    components
}

fn emoji_card(grid: Option<&[ActionRow]>) -> String {
    let mut s = String::new();

    let grid = match grid {
        Some(grid) => grid,
        None => return String::from("⬛⬛⬛⬛⬛\n⬛⬛⬛⬛⬛\n⬛⬛⬛⬛⬛\n⬛⬛⬛⬛⬛\n⬛⬛⬛⬛⬛"),
    };

    for row in grid {
        for component in &row.components {
            let ActionRowComponent::Button(button) = component else {
                unreachable!("Component must be a button")
            };

            let ButtonKind::NonLink { style, .. } = button.data else {
                unreachable!("Button data must be of kinda NonLink")
            };

            if style == ButtonStyle::Success {
                s.push('🟩');
                continue;
            }

            s.push('⬛');
        }

        s.push('\n');
    }

    s
}

fn live_embed(components: Option<&[ActionRow]>, mut spaces: Vec<String>) -> CreateEmbed {
    spaces.reverse();

    let card = emoji_card(components);

    let mut spaces_str = String::new();

    for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2 {
                continue;
            }

            spaces_str.push_str(&format!("{i}{j}: {}\n", spaces.pop().unwrap()));
        }

        spaces_str.push('\n');
    }

    CreateEmbed::new()
        .field("Card", card, false)
        .field("Spaces", spaces_str, false)
}

fn center_pad(text: &str, width: usize) -> String {
    let text_len = text.len();

    if width <= text_len {
        return text.to_string();
    }

    let total_padding = width - text_len;
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    format!(
        "{}{}{}",
        " ".repeat(left_padding),
        text,
        " ".repeat(right_padding)
    )
}

fn pretty_print_card(grid: &[ActionRow]) -> String {
    let max_item_len = grid
        .iter()
        .flat_map(|row| {
            row.components
                .iter()
                .filter_map(|component| match component {
                    ActionRowComponent::Button(component) => Some(component),
                    _ => None,
                })
                .filter_map(|button| button.label.as_deref())
        })
        .map(|label| label.len())
        .max()
        .unwrap();

    let col_width = max_item_len;

    let mut output_string = String::new();

    let horizontal_segment = "-".repeat(col_width);

    let separator_inner_parts: Vec<&str> = (0..GRID_SIZE)
        .map(|_| horizontal_segment.as_str())
        .collect();
    let separator_line = format!("+{}+\n", separator_inner_parts.join("+"));

    output_string.push_str(&separator_line);

    for row in grid.iter() {
        let centered_cells: Vec<String> = row
            .components
            .iter()
            .filter_map(|component| match component {
                ActionRowComponent::Button(component) => Some(component),
                _ => None,
            })
            .filter_map(|button| match &button.data {
                ButtonKind::NonLink { style, .. } => button.label.as_deref().map(|l| (l, style)),
                _ => None,
            })
            .map(|(label, _style)| center_pad(label, col_width))
            .collect();

        let row_str = format!("|{}|\n", centered_cells.join("|"));

        output_string.push_str(&row_str);
        output_string.push_str(&separator_line); // Add separator after each row
    }

    output_string
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

fn get_button_label(grid: &[ActionRow], r: u8, c: u8) -> &str {
    match &grid[r as usize].components[c as usize] {
        ActionRowComponent::Button(button) => match button.label.as_deref() {
            Some(label) => label,
            None => "FREE",
        },
        _ => unreachable!("Expected Button component at ({}, {})", r, c),
    }
}

fn update_button(components: &mut [ActionRow], button_id: &str) -> bool {
    let mut chars = button_id.strip_prefix("bingo_").unwrap().chars();

    let mut r_buffer = [0; 4];
    let mut c_buffer = [0; 4];

    let r = chars.next().unwrap().encode_utf8(&mut r_buffer);
    let c = chars.next().unwrap().encode_utf8(&mut c_buffer);

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

                if *style == ButtonStyle::Success {
                    *style = ButtonStyle::Secondary;
                    return false;
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GridCondition {
    RowSuccess(u8),
    ColumnSuccess(u8),
    MainDiagonalSuccess,
    AntiDiagonalSuccess,
    FullGridSuccess,
}

impl FromStr for GridCondition {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("RowSuccess(") && s.ends_with(')') {
            let inner = &s[11..s.len() - 1];
            match inner.parse::<u8>() {
                Ok(val) => Ok(GridCondition::RowSuccess(val)),
                Err(_) => Err(()),
            }
        } else if s.starts_with("ColumnSuccess(") && s.ends_with(')') {
            let inner = &s[14..s.len() - 1];
            match inner.parse::<u8>() {
                Ok(val) => Ok(GridCondition::ColumnSuccess(val)),
                Err(_) => Err(()),
            }
        } else {
            match s {
                "MainDiagonalSuccess" => Ok(GridCondition::MainDiagonalSuccess),
                "AntiDiagonalSuccess" => Ok(GridCondition::AntiDiagonalSuccess),
                "FullGridSuccess" => Ok(GridCondition::FullGridSuccess),
                _ => Err(()),
            }
        }
    }
}

fn check_grid_conditions(
    grid: &[ActionRow],
    win_states: &[GridCondition],
) -> (Option<GridCondition>, Vec<String>) {
    // 1. Check for any full row of ButtonStyle::Success
    for r_idx in (0..GRID_SIZE).filter(|idx| !win_states.contains(&GridCondition::RowSuccess(*idx)))
    {
        if (0..GRID_SIZE).all(|c_idx| get_button_style(grid, r_idx, c_idx) == ButtonStyle::Success)
        {
            return (
                Some(GridCondition::RowSuccess(r_idx)),
                (0..GRID_SIZE)
                    .map(|c_idx| get_button_label(grid, r_idx, c_idx).to_string())
                    .collect(),
            );
        }
    }

    // 2. Check for any full column of ButtonStyle::Success
    for c_idx in
        (0..GRID_SIZE).filter(|idx| !win_states.contains(&GridCondition::ColumnSuccess(*idx)))
    {
        if (0..GRID_SIZE).all(|r_idx| get_button_style(grid, r_idx, c_idx) == ButtonStyle::Success)
        {
            return (
                Some(GridCondition::ColumnSuccess(c_idx)),
                (0..GRID_SIZE)
                    .map(|r_idx| get_button_label(grid, r_idx, c_idx).to_string())
                    .collect(),
            );
        }
    }

    // 3. Check diagonals
    // Main diagonal (top-left to bottom-right)
    if !win_states.contains(&GridCondition::MainDiagonalSuccess)
        && (0..GRID_SIZE).all(|i| get_button_style(grid, i, i) == ButtonStyle::Success)
    {
        return (
            Some(GridCondition::MainDiagonalSuccess),
            (0..GRID_SIZE)
                .map(|i| get_button_label(grid, i, i).to_string())
                .collect(),
        );
    }

    // Anti-diagonal (top-right to bottom-left)
    if !win_states.contains(&GridCondition::AntiDiagonalSuccess)
        && (0..GRID_SIZE)
            .all(|i| get_button_style(grid, i, GRID_SIZE - 1 - i) == ButtonStyle::Success)
    {
        return (
            Some(GridCondition::AntiDiagonalSuccess),
            (0..GRID_SIZE)
                .map(|i| get_button_label(grid, i, GRID_SIZE - 1 - i).to_string())
                .collect(),
        );
    }

    if !win_states.contains(&GridCondition::FullGridSuccess) {
        let mut all_grid_is_success = true;
        let mut grid_labels = Vec::with_capacity(5);
        for r_idx in 0..GRID_SIZE {
            for c_idx in 0..GRID_SIZE {
                if get_button_style(grid, r_idx, c_idx) != ButtonStyle::Success {
                    all_grid_is_success = false;
                    break;
                }

                grid_labels.push(get_button_label(grid, r_idx, c_idx).to_string());
            }

            if !all_grid_is_success {
                break;
            }
        }
        if all_grid_is_success {
            return (Some(GridCondition::FullGridSuccess), grid_labels);
        }
    }

    (None, Vec::new())
}
