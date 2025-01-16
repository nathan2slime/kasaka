use std::io;

use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Margin, Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::text;
use ratzilla::ratatui::widgets::{BorderType, Wrap};
use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};
use ratzilla::widgets::Hyperlink;
use ratzilla::{DomBackend, RenderOnWeb};

const BANNER: &str = r#"
▄▄▄▄  ▗▞▀▜▌   ■  ▐▌▗▞▀▜▌▄▄▄▄  ▄▄▄▄ ▗▖    ▄▄▄   ▄▄▄  ▄▄▄ 
█   █ ▝▚▄▟▌▗▄▟▙▄▖▐▌▝▚▄▟▌█   █    █ ▐▌   █   █ ▀▄▄  ▀▄▄  
█   █        ▐▌  ▐▛▀▚▖  █   █ ▀▀▀█ ▐▛▀▚▖▀▄▄▄▀ ▄▄▄▀ ▄▄▄▀ 
             ▐▌  ▐▌ ▐▌        ▄▄▄█ ▐▙▄▞▘                
             ▐▌                                         
"#;

// const BANNER:&str = r#"
// _     _                     ____  _
// _ _    __ _   | |_  | |_    __ _   _ _    |__ / | |__    ___    ___    ___
// | ' \  / _` |  |  _| | ' \  / _` | | ' \    |_ \ | '_ \  / _ \  (_-<   (_-<
// |_||_| \__,_|  _\__| |_||_| \__,_| |_||_|  |___/ |_.__/  \___/  /__/_  /__/_
// _|"""""_|"""""_|"""""_|"""""_|"""""_|"""""_|"""""_|"""""_|"""""_|"""""_|"""""|
// "`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-"`-0-0-'
// "#;

const TITLE: &str = r#"
A túreo nathan3boss: Aranelo enniëo au Daugeom!
"#;

const TEXT_IM: &str = r#"
Num ambar yassë lómë moiquain matalyar, ar sindanátar tecelyar lávatyë atan arcollo, mahtar aranwa orya, nathan3boss, iquë na tamalya Daugeom.
Yassë yanta Daugeom, nai lómë mahtatyë ulundo i natië—i Nauco Eldaror, ar taura Ipa Mahta Naucor, nerantevë i anion, lúmë yúla, tulu telpë nathan tecë; sír quanta súrë, atan (Compile Valaina) i auto, nathan3boss, quetë Tári amille; lumeva valaina, ecco yanta.
"#;

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/nathan2slime"),
];

fn main() -> io::Result<()> {
    let backend = DomBackend::new();
    let terminal = Terminal::new(backend)?;

    terminal.render_on_web(move |frame| {
        let vertical = Layout::vertical([Constraint::Percentage(80)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);
        let block = Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(Color::Rgb(64, 61, 82))
            .style(
                Style::default()
                    .fg(Color::Rgb(196, 167, 231))
                    .bg(Color::Rgb(31, 29, 46)),
            )
            .title_style(
                Style::new()
                    .bg(Color::Rgb(33, 32, 46))
                    .fg(Color::Rgb(196, 167, 231))
                    .bold()
                    .italic(),
            )
            .title_bottom("|nathan3boss.dev|".not_bold())
            .title_alignment(Alignment::Right);
        frame.render_widget(block, area);

        let title = String::from(TITLE);
        let text_in = textwrap::wrap(TEXT_IM.trim(), area.width as usize - 10)
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let banner_area = Rect::new(30, 0, 80, 8);

        let [_margin, title_area, text_in_area, links_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(text_in.lines().count() as u16 + 2),
            Constraint::Length(LINKS.len() as u16 + 2),
        ])
        .areas(area.offset(Offset { x: -1, y: 0 }).inner(Margin {
            horizontal: 5,
            vertical: 1,
        }));

        frame.render_widget(
            Paragraph::new(BANNER)
                .alignment(Alignment::Center)
                .style(Style::new().fg(Color::Rgb(235, 111, 146))),
            banner_area,
        );
        frame.render_widget(
            Paragraph::new(title)
                .style(Style::new())
                .wrap(Wrap { trim: false })
                .left_aligned()
                .block(Block::new()),
            title_area,
        );
        frame.render_widget(
            Paragraph::new(text_in)
                .style(Style::new().fg(Color::Rgb(224, 222, 244)))
                .wrap(Wrap { trim: false })
                .left_aligned()
                .block(Block::new()),
            text_in_area,
        );
       
        frame.render_widget(Block::bordered().title("Links".bold()), links_area);

        for (i, (_, url)) in LINKS.iter().enumerate() {
            let link = Hyperlink::new(url);
            frame.render_widget(
                link,
                links_area.offset(Offset {
                    x: 1,
                    y: i as i32 + 1,
                }),
            );
        }
    });

    Ok(())
}
