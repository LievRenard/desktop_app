use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, process::{Command, Stdio},sync::{Arc, atomic::{AtomicUsize, Ordering}}};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use desktop_app::ui::launcher;
use desktop_app::settings::programs;

enum AppPage {
    ApplicationLauncher,
    TaskSwitcher
}

struct App {
    app_select: i32,
    tab: AppPage
}

impl App {
    fn new() -> App {
        App {
            app_select: 0,
            tab: AppPage::ApplicationLauncher
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup signal handler
    let term = Arc::new(AtomicUsize::new(0));

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app, term);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, term: Arc<AtomicUsize>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        signal_hook::flag::register_usize(signal_hook::consts::SIGUSR1, Arc::clone(&term),1)?;
        signal_hook::flag::register_usize(signal_hook::consts::SIGUSR2, Arc::clone(&term),1)?;
        match term.load(Ordering::Relaxed) {
            0 => (),
            1 => {
                app.tab = AppPage::ApplicationLauncher;
                term.store(0,Ordering::Relaxed);
            }
            2 => {
                app.tab = AppPage::ApplicationLauncher;
                term.store(0,Ordering::Relaxed);
            }
            _ => term.store(0,Ordering::Relaxed)
        }

        if let Event::Key(key) = event::read()? {
            match key.code{
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.app_select += 1,
                KeyCode::Left => app.app_select -= 1,
                KeyCode::Down => if app.app_select+6 < programs::PROGRAM_LIST.len() as i32 {app.app_select += 6;},
                KeyCode::Up => if app.app_select-6 >= 0 {app.app_select -= 6;},
                KeyCode::Enter => {
                    Command::new("sh")
                        .args(["-c", programs::PROGRAM_LIST[app.app_select as usize].2, "&" ])
                        .stderr(Stdio::null())
                        .stdout(Stdio::null())
                        .spawn()
                        .expect("failed to execute process");
                },
                _ => (),
            }
        }
        if app.app_select >= programs::PROGRAM_LIST.len() as i32 {
            app.app_select -= programs::PROGRAM_LIST.len() as i32;
        }
        else if app.app_select < 0 {
            app.app_select += programs::PROGRAM_LIST.len() as i32;
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.tab {
        AppPage::ApplicationLauncher => launcher::draw(f, app.app_select),
        AppPage::TaskSwitcher => ()
    }
}

fn get_window_title() -> Vec<String> {
    let windows = wmctrl::get_windows();
    let mut titles: Vec<String> = Vec::new();

    for win in windows.iter() {
        if win.title().contains("Plasma") {
            continue;
        }
        titles.push(win.title().to_string());
    }
    titles
}