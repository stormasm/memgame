mod assets;
mod colors;
mod components;
mod memory_game;
mod styled;
mod utils;

use assets::Assets;
use gpui::prelude::*;
use gpui::{App, Application, Bounds, KeyBinding, TitlebarOptions, WindowBounds, WindowKind, WindowOptions};
use gpui::{actions, px, size};
use memory_game::MemoryGame;

actions!(gpui_shadcn, [Quit, Open, CloseWindow]);

fn main() {
  let app = Application::new().with_assets(Assets);

  app.run(|cx| {
    cx.bind_keys([KeyBinding::new("cmd-w", Quit, None)]);
    cx.on_window_closed(|cx| {
      if cx.windows().is_empty() {
        cx.quit();
      }
    })
    .detach();

    cx.on_action(quit);
    cx.activate(true);

    let titlebar = TitlebarOptions {
      title: Some("Memory Match Game".into()),
      ..Default::default()
    };
    let min_size = size(px(800.0), px(850.0));
    let bounds = Bounds::centered(None, min_size, cx);
    let options = WindowOptions {
      window_bounds: Some(WindowBounds::Windowed(bounds)),
      window_min_size: Some(min_size),
      titlebar: Some(titlebar),
      kind: WindowKind::Normal,
      ..Default::default()
    };
    cx.open_window(options, |_window, cx| cx.new(|_cx| MemoryGame::new()))
      .expect("failed to open window");
  });
}

fn quit(_: &Quit, cx: &mut App) {
  cx.quit();
}
