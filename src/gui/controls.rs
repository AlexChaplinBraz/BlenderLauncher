use super::{sort_by::SortBy, Message};
use crate::{
    releases::UpdateCount,
    settings::{get_setting, CAN_CONNECT, FETCHING, INSTALLING},
};
use iced::{
    button, pick_list, scrollable, Align, Button, Checkbox, Column, Container, Length, PickList,
    Row, Rule, Scrollable, Space, Text,
};
use std::sync::atomic::Ordering;

#[derive(Debug, Default)]
pub struct Controls {
    pub check_for_updates_button: button::State,
    pub fetch_all_button: button::State,
    pub fetch_daily_latest_button: button::State,
    pub fetch_daily_archive_button: button::State,
    pub fetch_experimental_latest_button: button::State,
    pub fetch_experimental_archive_button: button::State,
    pub fetch_patch_latest_button: button::State,
    pub fetch_patch_archive_button: button::State,
    pub fetch_stable_latest_button: button::State,
    pub fetch_stable_archive_button: button::State,
    pub fetch_lts_button: button::State,
    pub sorting_pick_list: pick_list::State<SortBy>,
    pub scroll: scrollable::State,
    pub check_connection_button: button::State,
    pub checking_connection: bool,
}

impl Controls {
    pub fn view(&mut self, update_count: UpdateCount) -> Container<'_, Message> {
        let update_button = {
            let button = Button::new(
                &mut self.check_for_updates_button,
                Text::new("[C] Check for updates"),
            )
            .style(get_setting().theme);

            if CAN_CONNECT.load(Ordering::Relaxed)
                && !INSTALLING.load(Ordering::Relaxed)
                && !FETCHING.load(Ordering::Relaxed)
            {
                button.on_press(Message::CheckForUpdates)
            } else {
                button
            }
        };

        let filter_row = |filter,
                          label,
                          checkbox_message: fn(bool) -> Message,
                          state,
                          button_message: Option<Message>| {
            let row = Row::new()
                .height(Length::Units(25))
                .align_items(Align::Center)
                .push(
                    Checkbox::new(filter, label, checkbox_message)
                        .width(Length::Fill)
                        .style(get_setting().theme),
                );
            match state {
                Some(state) => {
                    let button = Button::new(state, Text::new("[F]")).style(get_setting().theme);

                    match button_message {
                        Some(button_message) => {
                            if CAN_CONNECT.load(Ordering::Relaxed)
                                && !INSTALLING.load(Ordering::Relaxed)
                                && !FETCHING.load(Ordering::Relaxed)
                            {
                                row.push(button.on_press(button_message))
                            } else {
                                row.push(button)
                            }
                        }
                        None => row.push(button),
                    }
                }
                None => row,
            }
        };

        let filters = Column::new()
            .spacing(5)
            .push(filter_row(
                get_setting().filters.updates,
                match update_count.all {
                    Some(count) => {
                        format!("Updates [{}]", count)
                    }
                    None => String::from("Updates"),
                },
                Message::FilterUpdatesChanged,
                None,
                None,
            ))
            .push(filter_row(
                get_setting().filters.bookmarks,
                String::from("Bookmarks"),
                Message::FilterBookmarksChanged,
                None,
                None,
            ))
            .push(filter_row(
                get_setting().filters.installed,
                String::from("Installed"),
                Message::FilterInstalledChanged,
                None,
                None,
            ))
            .push(Rule::horizontal(5).style(get_setting().theme))
            .push(filter_row(
                get_setting().filters.all,
                String::from("All"),
                Message::FilterAllChanged,
                Some(&mut self.fetch_all_button),
                Some(Message::FetchAll),
            ))
            .push(filter_row(
                get_setting().filters.daily_latest,
                match update_count.daily {
                    Some(count) => {
                        format!("Daily (latest) [{}]", count)
                    }
                    None => String::from("Daily (latest)"),
                },
                Message::FilterDailyLatestChanged,
                Some(&mut self.fetch_daily_latest_button),
                Some(Message::FetchDailyLatest),
            ))
            .push(filter_row(
                get_setting().filters.daily_archive,
                String::from("Daily (archive)"),
                Message::FilterDailyArchiveChanged,
                Some(&mut self.fetch_daily_archive_button),
                Some(Message::FetchDailyArchive),
            ))
            .push(filter_row(
                get_setting().filters.experimental_latest,
                match update_count.experimental {
                    Some(count) => {
                        format!("Experimental (latest) [{}]", count)
                    }
                    None => String::from("Experimental (latest)"),
                },
                Message::FilterExperimentalLatestChanged,
                Some(&mut self.fetch_experimental_latest_button),
                Some(Message::FetchExperimentalLatest),
            ))
            .push(filter_row(
                get_setting().filters.experimental_archive,
                String::from("Experimental (archive)"),
                Message::FilterExperimentalArchiveChanged,
                Some(&mut self.fetch_experimental_archive_button),
                Some(Message::FetchExperimentalArchive),
            ))
            .push(filter_row(
                get_setting().filters.patch_latest,
                match update_count.patch {
                    Some(count) => {
                        format!("Patch (latest) [{}]", count)
                    }
                    None => String::from("Patch (latest)"),
                },
                Message::FilterPatchLatestChanged,
                Some(&mut self.fetch_patch_latest_button),
                Some(Message::FetchPatchLatest),
            ))
            .push(filter_row(
                get_setting().filters.patch_archive,
                String::from("Patch (archive)"),
                Message::FilterPatchArchiveChanged,
                Some(&mut self.fetch_patch_archive_button),
                Some(Message::FetchPatchArchive),
            ))
            .push(filter_row(
                get_setting().filters.stable_latest,
                match update_count.stable {
                    Some(count) => {
                        format!("Stable (latest) [{}]", count)
                    }
                    None => String::from("Stable (latest)"),
                },
                Message::FilterStableLatestChanged,
                Some(&mut self.fetch_stable_latest_button),
                Some(Message::FetchStableLatest),
            ))
            .push(filter_row(
                get_setting().filters.stable_archive,
                String::from("Stable (archive)"),
                Message::FilterStableArchiveChanged,
                Some(&mut self.fetch_stable_archive_button),
                Some(Message::FetchStableArchive),
            ))
            .push(filter_row(
                get_setting().filters.lts,
                match update_count.lts {
                    Some(count) => {
                        format!("Long-term Support [{}]", count)
                    }
                    None => String::from("Long-term Support"),
                },
                Message::FilterLtsChanged,
                Some(&mut self.fetch_lts_button),
                Some(Message::FetchLts),
            ));

        let sorting = Row::new()
            .spacing(8)
            .align_items(Align::Center)
            .push(Text::new("Sort by"))
            .push(
                PickList::new(
                    &mut self.sorting_pick_list,
                    &SortBy::ALL[..],
                    Some(get_setting().sort_by),
                    Message::SortingChanged,
                )
                .width(Length::Fill)
                .style(get_setting().theme),
            );

        let scrollable = Scrollable::new(&mut self.scroll).push(
            Column::new()
                .spacing(5)
                .padding(10)
                .align_items(Align::Center)
                .push(update_button)
                .push(filters)
                .push(sorting),
        );

        if CAN_CONNECT.load(Ordering::Relaxed) {
            Container::new(scrollable)
                // TODO: Can't get it to shrink around its content for some reason.
                // It always fills the whole space unless I set a specific width.
                .width(Length::Units(220))
                .height(Length::Fill)
                .style(get_setting().theme.sidebar_container())
        } else {
            Container::new(
                Column::new().push(scrollable.height(Length::Fill)).push(
                    Container::new(
                        Row::new()
                            .padding(1)
                            .align_items(Align::Center)
                            .push(Space::with_width(Length::Units(9)))
                            .push(Text::new("CANNOT CONNECT").width(Length::Fill))
                            .push({
                                let button = Button::new(
                                    &mut self.check_connection_button,
                                    Text::new("[R]"),
                                )
                                .style(get_setting().theme.tab_button());

                                if self.checking_connection {
                                    button
                                } else {
                                    button.on_press(Message::CheckConnection)
                                }
                            })
                            .push(Space::with_width(Length::Units(9))),
                    )
                    .width(Length::Fill)
                    .style(get_setting().theme.status_container()),
                ),
            )
            .width(Length::Units(190))
            .height(Length::Fill)
            .style(get_setting().theme.sidebar_container())
        }
    }
}
