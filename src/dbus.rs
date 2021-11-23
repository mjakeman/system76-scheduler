// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::Event;
use postage::mpsc::Sender;
use postage::prelude::*;
use zvariant::derive::Type;

#[derive(Copy, Clone, Deserialize, Serialize, Type)]
pub enum CpuMode {
    Auto = 0,
    Custom = 1,
    Default = 2,
    Responsive = 3,
}

pub(crate) struct Server {
    pub cpu_mode: CpuMode,
    pub cpu_profile: String,
    pub tx: Sender<Event>,
}

#[dbus_proxy(
    default_service = "com.system76.Scheduler",
    interface = "com.system76.Scheduler",
    default_path = "/com/system76/Scheduler"
)]
pub trait Client {
    fn cpu_mode(&self) -> zbus::fdo::Result<CpuMode>;

    #[dbus_proxy(property)]
    fn cpu_profile(&self) -> zbus::fdo::Result<String>;

    fn set_cpu_mode(&mut self, cpu_mode: CpuMode) -> zbus::fdo::Result<()>;

    fn set_cpu_profile(&mut self, profile: &str) -> zbus::fdo::Result<()>;
}

#[dbus_interface(name = "com.system76.Scheduler")]
impl Server {
    fn cpu_mode(&self) -> CpuMode {
        self.cpu_mode
    }

    #[dbus_interface(property)]
    fn cpu_profile(&self) -> &str {
        match self.cpu_mode {
            CpuMode::Auto => "auto",
            CpuMode::Custom => &self.cpu_profile,
            CpuMode::Default => "default",
            CpuMode::Responsive => "responsive",
        }
    }

    async fn set_cpu_mode(&mut self, cpu_mode: CpuMode) {
        self.cpu_mode = cpu_mode;

        let _ = self.tx.send(Event::SetCpuMode(cpu_mode)).await;
    }

    async fn set_cpu_profile(&mut self, profile: String) {
        match profile.as_str() {
            "auto" => self.set_cpu_mode(CpuMode::Auto).await,
            "default" => self.set_cpu_mode(CpuMode::Default).await,
            "responsive" => self.set_cpu_mode(CpuMode::Responsive).await,
            "" => (),
            _ => {
                self.cpu_mode = CpuMode::Custom;
                self.cpu_profile = profile.clone();

                let _ = self.tx.send(Event::SetCustomCpuMode(profile)).await;
            }
        }
    }
}