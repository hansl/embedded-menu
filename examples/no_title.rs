//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::simulator::Simulator,
    items::{select::SelectValue, NavigationItem, Select},
    Menu, MenuStyle,
};

#[derive(Copy, Clone, PartialEq)]
pub enum TestEnum {
    A,
    B,
    C,
}

impl SelectValue for TestEnum {
    fn next(&self) -> Self {
        match self {
            TestEnum::A => TestEnum::B,
            TestEnum::B => TestEnum::C,
            TestEnum::C => TestEnum::A,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut menu = Menu::with_style(
        "",
        MenuStyle::default()
            .with_input_adapter(Simulator::default())
            .with_details_delay(100),
    )
    .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
    .add_item(Select::new("Check this 1", false).with_detail_text("Description"))
    .add_item(Select::new("Check this 2", false).with_detail_text("Description"))
    .add_item(Select::new("Check this 3", TestEnum::A).with_detail_text("Description"))
    .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    'running: loop {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
        menu.update(&display);
        menu.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            menu.interact(event);

            match event {
                SimulatorEvent::Quit => break 'running,
                _ => continue,
            }
        }
    }

    Ok(())
}