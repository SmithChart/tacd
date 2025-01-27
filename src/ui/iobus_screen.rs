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

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*, text::Text,
};

use crate::broker::{Native, SubscriptionHandle};
use crate::iobus::{LSSState, Nodes, ServerInfo};

use super::widgets::*;
use super::{draw_border, ButtonEvent, MountableScreen, Screen, Ui};

const SCREEN_TYPE: Screen = Screen::IoBus;

pub struct IoBusScreen {
    widgets: Vec<Box<dyn AnyWidget>>,
    buttons_handle: Option<SubscriptionHandle<ButtonEvent, Native>>,
}

impl IoBusScreen {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            buttons_handle: None,
        }
    }
}

#[async_trait]
impl MountableScreen for IoBusScreen {
    fn is_my_type(&self, screen: Screen) -> bool {
        screen == SCREEN_TYPE
    }

    async fn mount(&mut self, ui: &Ui) {
        draw_border("IOBus", SCREEN_TYPE, &ui.draw_target).await;

        {
            let mut draw_target = ui.draw_target.lock().await;

            let ui_text_style: MonoTextStyle<BinaryColor> =
                MonoTextStyle::new(&UI_TEXT_FONT, BinaryColor::On);

            Text::new("Power/Fault:", Point::new(0, 26), ui_text_style)
                .draw(&mut *draw_target)
                .unwrap();

            Text::new("Scan/CAN OK:", Point::new(0, 40), ui_text_style)
                .draw(&mut *draw_target)
                .unwrap();
        }

        self.widgets.push(Box::new(
            DynamicWidget::locator(ui.locator_dance.clone(), ui.draw_target.clone()).await,
        ));

        self.widgets.push(Box::new(
            DynamicWidget::indicator(
                ui.res.dig_io.iobus_pwr_en.clone(),
                ui.draw_target.clone(),
                Point::new(80, 26 - 7),
                Box::new(|state: &bool| match *state {
                    true => IndicatorState::On,
                    false => IndicatorState::Off,
                }),
            )
            .await,
        ));

        self.widgets.push(Box::new(
            DynamicWidget::indicator(
                ui.res.dig_io.iobus_flt_fb.clone(),
                ui.draw_target.clone(),
                Point::new(101, 26 - 7),
                Box::new(|state: &bool| match *state {
                    true => IndicatorState::On,
                    false => IndicatorState::Off,
                }),
            )
            .await,
        ));

        self.widgets.push(Box::new(
            DynamicWidget::indicator(
                ui.res.iobus.server_info.clone(),
                ui.draw_target.clone(),
                Point::new(80, 40 - 7),
                Box::new(|info: &ServerInfo| match info.lss_state {
                    LSSState::Scanning => IndicatorState::On,
                    LSSState::Idle => IndicatorState::Off,
                }),
            )
            .await,
        ));

        self.widgets.push(Box::new(
            DynamicWidget::indicator(
                ui.res.iobus.server_info.clone(),
                ui.draw_target.clone(),
                Point::new(101, 40 - 7),
                Box::new(|info: &ServerInfo| match info.can_tx_error {
                    false => IndicatorState::On,
                    true => IndicatorState::Off,
                }),
            )
            .await,
        ));

        self.widgets.push(Box::new(
            DynamicWidget::text(
                ui.res.iobus.nodes.clone(),
                ui.draw_target.clone(),
                Point::new(0, 54),
                Box::new(move |nodes: &Nodes| format!("Nodes: {}", nodes.result.len())),
            )
            .await,
        ));

        let (mut button_events, buttons_handle) = ui.buttons.clone().subscribe_unbounded().await;
        let iobus_pwr_en = ui.res.dig_io.iobus_pwr_en.clone();
        let screen = ui.screen.clone();

        spawn(async move {
            while let Some(ev) = button_events.next().await {
                if let ButtonEvent::ButtonOne(_) = *ev {
                    iobus_pwr_en
                        .modify(|prev| Some(Arc::new(!prev.as_deref().copied().unwrap_or(true))))
                        .await
                }

                if let ButtonEvent::ButtonTwo(_) = *ev {
                    screen.set(SCREEN_TYPE.next()).await
                }
            }
        });

        self.buttons_handle = Some(buttons_handle);
    }

    async fn unmount(&mut self) {
        if let Some(handle) = self.buttons_handle.take() {
            handle.unsubscribe().await;
        }

        for mut widget in self.widgets.drain(..) {
            widget.unmount().await
        }
    }
}
