use assets_manager::AssetCache;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{
    components::{GameMode, GameState, GameUI, Player},
    game::Game,
};

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create asset cache
    let cache: AssetCache = AssetCache::new("assets").expect("Could not load ./assets folder");

    // create GameState
    let mut state = GameState::new();
    state.player = Player::init();

    // create GameUI
    let ui = GameUI::default();

    // create Game
    let game = Game { ui, state, cache };
    run_game(&mut terminal, game, tick_rate)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    mut game: Game,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            match &game.state.game_mode {
                None => game.ui.draw(f, &mut game.state),
                Some(mode) => match mode {
                    GameMode::Edit => {}
                    GameMode::Story => {
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Length(50), Constraint::Ratio(1, 2)].as_ref())
                            .margin(1)
                            .split(f.size());

                        {
                            // allocate spaces for map and map information
                            let chunks = Layout::default()
                                .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                                .split(chunks[0]);

                            game.state.world_grid.draw_info(f, &game.state, chunks[0]);

                            game.state.world_grid.draw_tiles(f, &game.state, chunks[1]);
                        }

                        {
                            let chunks = Layout::default()
                                .constraints(
                                    [Constraint::Percentage(50), Constraint::Percentage(50)]
                                        .as_ref(),
                                )
                                .split(chunks[1]);

                            game.state.messages.draw(f, chunks[0]);

                            game.state.player.draw_basic_info(f, chunks[1]);
                        }
                    }
                },
            }
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                game.on_key(key);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            game.on_tick();
        }

        if game.state.should_quit {
            return Ok(());
        }
    }
}
