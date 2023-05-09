// This file is part of tacd, the LXA TAC system daemon
// Copyright (C) 2022 Pengutronix e.K.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use async_std::prelude::*;
use async_std::sync::Arc;
use async_std::task::spawn;
use async_trait::async_trait;
use embedded_graphics::prelude::*;

use super::widgets::*;
use super::{
    ActivatableScreen, ActiveScreen, AlertList, AlertScreen, Alerter, Display, InputEvent, Screen,
    Ui,
};
use crate::broker::Topic;
use crate::dbus::rauc::Progress;

const SCREEN_TYPE: AlertScreen = AlertScreen::UpdateInstallation;

pub struct UpdateInstallationScreen;

impl UpdateInstallationScreen {
    pub fn new(alerts: &Arc<Topic<AlertList>>, operation: &Arc<Topic<String>>) -> Self {
        let (mut operation_events, _) = operation.clone().subscribe_unbounded();
        let alerts = alerts.clone();

        spawn(async move {
            while let Some(ev) = operation_events.next().await {
                match ev.as_str() {
                    "installing" => alerts.assert(SCREEN_TYPE),
                    _ => alerts.deassert(SCREEN_TYPE),
                };
            }
        });

        Self
    }
}

struct Active {
    widgets: WidgetContainer,
}

impl ActivatableScreen for UpdateInstallationScreen {
    fn my_type(&self) -> Screen {
        Screen::Alert(SCREEN_TYPE)
    }

    fn activate(&mut self, ui: &Ui, display: Display) -> Box<dyn ActiveScreen> {
        let mut widgets = WidgetContainer::new(display);

        widgets.push(|display| {
            DynamicWidget::text_center(
                ui.res.rauc.progress.clone(),
                display,
                Point::new(120, 100),
                Box::new(|progress: &Progress| {
                    let (_, text) = progress.message.split_whitespace().fold(
                        (0, String::new()),
                        move |(mut ll, mut text), word| {
                            let word_len = word.len();

                            if (ll + word_len) > 15 {
                                text.push('\n');
                                ll = 0;
                            } else {
                                text.push(' ');
                                ll += 1;
                            }

                            text.push_str(word);
                            ll += word_len;

                            (ll, text)
                        },
                    );

                    text
                }),
            )
        });

        widgets.push(|display| {
            DynamicWidget::bar(
                ui.res.rauc.progress.clone(),
                display,
                Point::new(20, 180),
                200,
                18,
                Box::new(|progress: &Progress| progress.percentage as f32 / 100.0),
            )
        });

        Box::new(Active { widgets })
    }
}

#[async_trait]
impl ActiveScreen for Active {
    fn my_type(&self) -> Screen {
        Screen::Alert(SCREEN_TYPE)
    }

    async fn deactivate(mut self: Box<Self>) -> Display {
        self.widgets.destroy().await
    }

    fn input(&mut self, _ev: InputEvent) {}
}
