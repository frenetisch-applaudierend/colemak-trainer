use std::{io::Result, time::Duration};

use crossterm::event::{self, Event, KeyEvent};
use ratatui::{widgets::Widget, Frame, Terminal};

pub struct App<State> {
    state: State,
    current_screen: Box<dyn Screen<AppState = State>>,
}

pub trait Screen {
    type AppState;

    fn render(&mut self, ctx: &mut RenderContext<'_, '_, '_, Self::AppState>);

    fn handle_event(&mut self, ctx: &mut EventContext<'_, Self::AppState>, event: KeyEvent);
}

pub struct EventContext<'a, State> {
    state: &'a State,
    next_screen: Option<Box<dyn Screen<AppState = State>>>,
    should_quit: bool,
}

pub struct RenderContext<'a, 'b, 'c, State> {
    state: &'a State,
    frame: &'b mut Frame<'c>,
}

impl<State> App<State> {
    pub fn new(state: State, initial_screen: impl Screen<AppState = State> + 'static) -> Self {
        let current_screen = Box::new(initial_screen);
        Self {
            state,
            current_screen,
        }
    }

    pub fn main_loop<Backend>(&mut self, terminal: &mut Terminal<Backend>) -> Result<()>
    where
        Backend: ratatui::backend::Backend,
    {
        let state = &mut self.state;
        loop {
            let screen = &mut self.current_screen;

            terminal.draw(|frame| {
                let mut ctx = RenderContext { state, frame };
                screen.render(&mut ctx);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(event) = event::read()? {
                    let screen = &mut self.current_screen;
                    let mut ctx = EventContext {
                        state,
                        next_screen: None,
                        should_quit: false,
                    };
                    screen.handle_event(&mut ctx, event);

                    if ctx.should_quit {
                        break;
                    }

                    if let Some(next_screen) = ctx.next_screen {
                        self.current_screen = next_screen;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, State> EventContext<'a, State> {
    pub fn replace_screen(&mut self, screen: impl Screen<AppState = State> + 'static) {
        self.next_screen = Some(Box::new(screen));
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl<State> RenderContext<'_, '_, '_, State> {
    pub fn render_widget(&mut self, widget: impl Widget) {
        self.frame.render_widget(widget, self.frame.size());
    }
}
