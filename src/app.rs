use crate::game::{GameState, Layer, Tile};
use chargrid::{render::ViewCell, Size};
use direction::CardinalDirection;
use rgb24::Rgb24;

struct AppData {
    game_state: GameState,
}

impl AppData {
    fn new(screen_size: Size) -> Self {
        Self {
            game_state: GameState::new(screen_size),
        }
    }

    fn handle_input(&mut self, input: chargrid::input::Input) {
        use chargrid::input::{Input, KeyboardInput};
        match input {
            Input::Keyboard(key) => match key {
                KeyboardInput::Left => self.game_state.maybe_move_player(CardinalDirection::West),
                KeyboardInput::Right => self.game_state.maybe_move_player(CardinalDirection::East),
                KeyboardInput::Up => self.game_state.maybe_move_player(CardinalDirection::North),
                KeyboardInput::Down => self.game_state.maybe_move_player(CardinalDirection::South),
                _ => {}
            },
            _ => (),
        }
    }
}

struct AppView {}

impl AppView {
    fn new() -> Self {
        Self {}
    }
}

impl<'a> chargrid::render::View<&'a AppData> for AppView {
    fn view<F: chargrid::app::Frame, C: chargrid::app::ColModify>(
        &mut self,
        data: &'a AppData,
        context: chargrid::app::ViewContext<C>,
        frame: &mut F,
    ) {
        for entity_to_render in data.game_state.entities_to_render() {
            let view_cell = match entity_to_render.tile {
                Tile::Player => ViewCell::new()
                    .with_character('@')
                    .with_foreground(Rgb24::new_grey(255)),
                Tile::Floor => ViewCell::new()
                    .with_character('.')
                    .with_foreground(Rgb24::new_grey(63))
                    .with_background(Rgb24::new(0, 0, 63)),
                Tile::Wall => ViewCell::new()
                    .with_character('#')
                    .with_foreground(Rgb24::new(0, 63, 63))
                    .with_background(Rgb24::new(63, 127, 127)),
            };
            let depth = match entity_to_render.location.layer {
                None => -1,
                Some(Layer::Floor) => 0,
                Some(Layer::Feature) => 1,
                Some(Layer::Character) => 2,
            };
            frame.set_cell_relative(entity_to_render.location.coord, depth, view_cell, context);
        }
    }
}

pub struct App {
    data: AppData,
    view: AppView,
}

impl App {
    pub fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new(),
        }
    }
}

impl chargrid::app::App for App {
    fn on_input(&mut self, input: chargrid::app::Input) -> Option<chargrid::app::ControlFlow> {
        use chargrid::input::{keys, Input};
        match input {
            Input::Keyboard(keys::ETX) | Input::Keyboard(keys::ESCAPE) => {
                Some(chargrid::app::ControlFlow::Exit)
            }
            other => {
                self.data.handle_input(other);
                None
            }
        }
    }

    fn on_frame<F, C>(
        &mut self,
        _since_last_frame: chargrid::app::Duration,
        view_context: chargrid::app::ViewContext<C>,
        frame: &mut F,
    ) -> Option<chargrid::app::ControlFlow>
    where
        F: chargrid::app::Frame,
        C: chargrid::app::ColModify,
    {
        use chargrid::render::View;
        self.view.view(&self.data, view_context, frame);
        None
    }
}
