use std::sync::atomic::Ordering;

use super::Tabs;
use crate::{
    gui::{controls::Controls, message::Message},
    package::Package,
    releases::UpdateCount,
    settings::{get_setting, FETCHING, TEXT_SIZE},
};
use iced::{
    button, scrollable, Align, Button, Column, Container, Element, Length, Row, Scrollable, Text,
};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct PackagesState {
    pub packages_scroll: scrollable::State,
    pub open_default_button: button::State,
    pub open_default_with_file_button: button::State,
}

impl<'a> Tabs {
    pub fn packages_body(
        &'a mut self,
        packages: &'a mut Vec<Package>,
        file_path: Option<String>,
        update_count: UpdateCount,
        file_exists: bool,
        controls: &'a mut Controls,
    ) -> Element<'a, Message> {
        // TODO: Use real icons for the buttons.
        // TODO: Add tooltips.
        let button = |label, package_message: Option<Message>, state| {
            let button = Button::new(state, Text::new(label)).style(get_setting().theme);

            match package_message {
                Some(package_message) => button.on_press(package_message),
                None => button,
            }
        };

        let info: Element<'_, Message> = Container::new(
            Column::new()
                .padding(10)
                .spacing(5)
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Align::Center)
                        .push(button(
                            "[=]",
                            get_setting()
                                .default_package
                                .clone()
                                .map(Message::OpenBlender),
                            &mut self.packages_state.open_default_button,
                        ))
                        .push(Text::new("Default package:"))
                        .push(
                            Text::new(match get_setting().default_package.clone() {
                                Some(package) => package.name,
                                None => String::from("not set"),
                            })
                            .color(get_setting().theme.highlight_text()),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Align::Center)
                        .push(button(
                            "[+]",
                            if file_path.is_some() && get_setting().default_package.is_some() {
                                Some(Message::OpenBlenderWithFile(
                                    get_setting().default_package.clone().unwrap(),
                                ))
                            } else {
                                None
                            },
                            &mut self.packages_state.open_default_with_file_button,
                        ))
                        .push(Text::new("File:"))
                        .push(
                            Text::new(match &file_path {
                                Some(file_path) => file_path,
                                None => "none",
                            })
                            .color(get_setting().theme.highlight_text()),
                        ),
                ),
        )
        .width(Length::Fill)
        .style(get_setting().theme.info_container())
        .into();

        let packages: Element<'_, Message> = {
            let mut package_count: u16 = 0;
            let filtered_packages = Container::new(
                packages
                    .iter_mut()
                    .filter(|package| get_setting().filters.matches(package))
                    .sorted_by(|a, b| get_setting().sort_by.get_ordering(a, b))
                    .fold(Column::new(), |column, package| {
                        package_count += 1;
                        let index = package.index;
                        let element = package.view(file_exists, package_count & 1 != 0);
                        column.push(
                            element.map(move |message| Message::PackageMessage((index, message))),
                        )
                    })
                    .width(Length::Fill),
            );

            let scrollable =
                Scrollable::new(&mut self.packages_state.packages_scroll).push(filtered_packages);

            if package_count == 0 {
                Container::new(
                    Text::new({
                        if FETCHING.load(Ordering::Relaxed) {
                            "Fetching..."
                        } else {
                            "No packages"
                        }
                    })
                    .size(TEXT_SIZE * 2),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y()
                .style(get_setting().theme)
                .into()
            } else {
                Container::new(scrollable)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .style(get_setting().theme)
                    .into()
            }
        };

        Container::new(
            Column::new()
                .push(info)
                .push(Row::new().push(controls.view(update_count)).push(packages)),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .style(get_setting().theme)
        .into()
    }
}
