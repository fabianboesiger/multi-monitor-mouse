mod geometry;
mod setup;

use std::{collections::VecDeque, time::{Instant, Duration}};

use geometry::*;
use setup::*;

use winit::{
    event::{Event, DeviceEvent},
    event_loop::EventLoop,
};
use mouse_rs::Mouse;


fn main() {
    App::new().run();
}


struct App {
    event_loop: EventLoop<()>,
    mouse: Mouse,
    descriptor: SetupDescriptor,
    mouse_positions: VecDeque<(Instant, Point)>,
    last_position: Option<Point>,
}

impl App {
    fn new() -> Self {
        let event_loop = EventLoop::new();
        let mouse = Mouse::new();
        let setup = Setup::from_monitor_handles(event_loop.available_monitors());
        let descriptor = SetupDescriptor::from(&setup);
        let mouse_positions = VecDeque::new();

        App {
            event_loop,
            mouse,
            descriptor,
            mouse_positions,
            last_position: None
        }
    }

    fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();
            
            match event {
                Event::DeviceEvent { device_id: _, event } => match event {
                    DeviceEvent::MouseMotion { delta: _ } => {

                        let virtual_position = self.mouse.get_position().unwrap();
                        let virtual_position = Point {
                            x: virtual_position.x as f64,
                            y: virtual_position.y as f64,
                        };

                        let physical_position = self.descriptor.virtual_to_physical(virtual_position);
                        
                        loop {
                            if let Some((instant, _)) = self.mouse_positions.front() {
                                if Instant::now().duration_since(*instant) > Duration::from_millis(100) {
                                    self.mouse_positions.pop_front();
                                    continue;
                                }
                            }
                            break;
                        }

                        self.mouse_positions.push_back((Instant::now(), physical_position));

                        let acc = self.mouse_positions.iter()
                            .zip(self.mouse_positions.iter().skip(1))
                            .map(|((_, fst), (_, snd))| *snd - *fst)
                            .fold(Point::zero(), |acc, diff| acc + diff);

                        if let Some(last_position) = self.last_position {
                            let movement = BoundedLine {
                                p1: last_position,
                                p2: physical_position,
                            };
                            let unbounded_movement = UnboundedLine {
                                p1: physical_position - acc,
                                p2: physical_position,
                            };


                            let mut exited_monitor = None;
                            for (i, monitor) in self.descriptor.monitors.iter().enumerate() {
                                if monitor.physical_rect.exited_by(&movement) {
                                    exited_monitor = Some(i);
                                }
                            }


                            if let Some(exited_monitor) = exited_monitor {
                                let nearest_point = nearest(self.descriptor.monitors
                                    .iter()
                                    .enumerate()
                                    .filter(|(i, _)| *i != exited_monitor)
                                    .map(|(_, monitor)| monitor.physical_rect.intersection(&unbounded_movement))
                                    .filter_map(|point| point),
                                    last_position);

                                if let Some(nearest_point) = nearest_point {
                                    let virtual_nearest_point = self.descriptor.physical_to_virtual(nearest_point);
                                    self.mouse.move_to(virtual_nearest_point.x as i32, virtual_nearest_point.y as i32).unwrap();
                                }
                            }
                        }

                        self.last_position = Some(physical_position);
                    },
                    _ => ()
                }
                _ => (),
            }
        });
    }
}