use crate::geometry::*;

use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use winit::{monitor::MonitorHandle, platform::unix::MonitorHandleExtUnix};

#[derive(Serialize, Deserialize, Debug)]
pub struct SavedSetups {
    setups: Vec<Setup>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setup {
    monitors: Vec<MonitorSetup>,
}

impl Setup {
    pub fn signature(&self) -> HashSet<u32> {
        self.monitors.iter().map(|monitor_setup| monitor_setup.id).collect()
    }

    pub fn from_monitor_handles(handles: impl Iterator<Item = MonitorHandle>) -> Self {
        Setup {
            monitors: handles.map(|handle| MonitorSetup::from_monitor_handle(handle)).collect()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorSetup {
    id: u32,
    name: Option<String>,
    dpi: f64,
    virtual_x: i32,
    virtual_y: i32,
    virtual_width: i32,
    virtual_height: i32,
    physical_x: f64,
    physical_y: f64,
    physical_width: f64,
    physical_height: f64,
}

impl MonitorSetup {
    fn from_monitor_handle(handle: MonitorHandle) -> Self {
        let dpi = handle.scale_factor();
        let virtual_x = handle.position().x;
        let virtual_y = handle.position().y;
        let virtual_width = handle.size().width as i32;
        let virtual_height = handle.size().height as i32;

        MonitorSetup {
            id: handle.native_id(),
            name: handle.name(),
            dpi,
            virtual_x,
            virtual_y,
            virtual_width,
            virtual_height,
            physical_x: virtual_x as f64 / dpi,
            physical_y: virtual_y as f64 / dpi,
            physical_width: virtual_width as f64 / dpi,
            physical_height: virtual_height as f64 / dpi,
        }
    }
}

pub struct SetupDescriptor {
    pub monitors: Vec<MonitorDescriptor>,
}

impl From<&Setup> for SetupDescriptor {
    fn from(setup: &Setup) -> Self {
        SetupDescriptor {
            monitors: setup.monitors.iter().map(|monitor| MonitorDescriptor::from(monitor)).collect()
        }
    }
}

impl SetupDescriptor {
    pub fn virtual_to_physical(&self, p: Point) -> Point {
        for monitor in &self.monitors {
            if monitor.virtual_rect.includes(&p) {
                return monitor.virtual_to_physical(p)
            }
        }

        unreachable!()
    }

    pub fn physical_to_virtual(&self, p: Point) -> Point {
        for monitor in &self.monitors {
            if monitor.physical_rect.includes(&p) {
                return monitor.physical_to_virtual(p)
            }
        }

        unreachable!()
    }
}

pub struct MonitorDescriptor {
    pub virtual_rect: Rect,
    pub physical_rect: Rect,
    virtual_position: Point,
    virtual_size: Point,
    physical_position: Point,
    physical_size: Point,
}

impl From<&MonitorSetup> for MonitorDescriptor {
    fn from(setup: &MonitorSetup) -> Self {
        let virtual_position  = Point {
            x: setup.virtual_x as f64,
            y: setup.virtual_y as f64,
        };
        let virtual_size = Point {
            x: setup.virtual_width as f64,
            y: setup.virtual_height as f64,
        };
        let physical_position  = Point {
            x: setup.physical_x as f64,
            y: setup.physical_y as f64,
        };
        let physical_size = Point {
            x: setup.physical_width as f64,
            y: setup.physical_height as f64,
        };
        MonitorDescriptor {
            virtual_rect: Rect::from_points(virtual_position, virtual_position + virtual_size),
            physical_rect: Rect::from_points(physical_position, physical_position + physical_size),
            virtual_position,
            virtual_size,
            physical_position,
            physical_size,
        }
    }
}

impl MonitorDescriptor {
    pub fn virtual_to_physical(&self, p: Point) -> Point {
        (p - self.virtual_position) / self.virtual_size * self.physical_size + self.physical_position
    }

    pub fn physical_to_virtual(&self, p: Point) -> Point {
        (p - self.physical_position) / self.physical_size * self.virtual_size + self.virtual_position
    }
}