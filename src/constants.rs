// The name of the application (put in .env file) ?
pub const APP_NAME: &str = "bevy_game";

/// Constants for the game
pub mod window {
  use bevy::window::{PresentMode, WindowMode};

  pub const CANVAS_ID: &str = "#canvas";
  pub const PRESENT_MODE: PresentMode = PresentMode::AutoVsync;
  pub const MODE: WindowMode = WindowMode::Windowed;
  pub const RESOLUTIONS: [(f32, f32); 5] = [
    (800.0, 600.0),
    (1024.0, 768.0),
    (1280.0, 720.0),
    (1366.0, 768.0),
    (1920.0, 1080.0),
  ];
}

#[allow(dead_code)]
pub mod colors {
  use bevy::color::{Color, Hsla};

  pub const GRAY_100: Color = Color::Hsla(Hsla::new(180., 2.2 / 100., 91.2 / 100., 1.));
  pub const GRAY_200: Color = Color::Hsla(Hsla::new(210., 2. / 100., 80.8 / 100., 1.));
  pub const GRAY_300: Color = Color::Hsla(Hsla::new(220., 1.5 / 100., 61.4 / 100., 1.));
  pub const GRAY_400: Color = Color::Hsla(Hsla::new(210., 2.8 / 100., 42. / 100., 1.));
  pub const GRAY_500: Color = Color::Hsla(Hsla::new(205.7, 6.1 / 100., 22.5 / 100., 1.));
  pub const GRAY_600: Color = Color::Hsla(Hsla::new(214.3, 11.9 / 100., 11.6 / 100., 1.));
  pub const GRAY_700: Color = Color::Hsla(Hsla::new(210., 13.6 / 100., 8.6 / 100., 1.));
  pub const GRAY_800: Color = Color::Hsla(Hsla::new(200., 10.3 / 100., 5.7 / 100., 1.));
  pub const GRAY_900: Color = Color::Hsla(Hsla::new(210., 14.3 / 100., 2.7 / 100., 1.));

  pub const RED_100: Color = Color::Hsla(Hsla::new(0., 93. / 100., 94. / 100., 1.));
  pub const RED_200: Color = Color::Hsla(Hsla::new(0., 96. / 100., 89. / 100., 1.));
  pub const RED_300: Color = Color::Hsla(Hsla::new(0., 94. / 100., 82. / 100., 1.));
  pub const RED_400: Color = Color::Hsla(Hsla::new(0., 91. / 100., 71. / 100., 1.));
  pub const RED_500: Color = Color::Hsla(Hsla::new(0., 84. / 100., 60. / 100., 1.));
  pub const RED_600: Color = Color::Hsla(Hsla::new(0., 80. / 100., 49. / 100., 1.));
  pub const RED_700: Color = Color::Hsla(Hsla::new(0., 74. / 100., 42. / 100., 1.));
  pub const RED_800: Color = Color::Hsla(Hsla::new(0., 70. / 100., 35. / 100., 1.));
  pub const RED_900: Color = Color::Hsla(Hsla::new(0., 63. / 100., 31. / 100., 1.));
}
