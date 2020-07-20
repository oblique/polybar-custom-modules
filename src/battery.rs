use anyhow::Result;
use battery::units::ratio;
use battery::{Battery, Manager, State};
use std::thread;
use std::time::Duration;

use crate::BatteryArgs;

const BAT_FULL: u32 = 98;
const BAT_GOOD: u32 = 90;
const BAT_WARN: u32 = 40;
const BAT_CRIT: u32 = 20;

const BAT_FULL_COLOR: &str = "#2ecc71";
const BAT_GOOD_COLOR: &str = "#5f87ff";
const BAT_WARN_COLOR: &str = "#ff5f87";
const BAT_CRIT_COLOR: &str = "#d81860";
const BACKGROUND_COLOR: &str = "#101010";

pub fn cmd_battery(_args: BatteryArgs) -> Result<()> {
    let manager = Manager::new()?;
    let mut bats: Vec<_> = manager
        .batteries()?
        .filter_map(|bat| bat.ok())
        .map(PolybarBat::new)
        .collect();

    loop {
        let mut blinking = false;

        for (i, bat) in bats.iter_mut().enumerate() {
            if i > 0 {
                print!(" ");
            }

            bat.refresh(&manager);
            bat.print();
            blinking |= bat.blink_enabled();
        }

        println!();

        if blinking {
            thread::sleep(Duration::from_millis(500));
        } else {
            thread::sleep(Duration::from_secs(2));
        }
    }
}

struct PolybarBat {
    inner: Battery,
    blink: Blink,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Blink {
    Disabled,
    Show,
    Hide,
}

impl PolybarBat {
    fn new(inner: Battery) -> PolybarBat {
        PolybarBat {
            inner,
            blink: Blink::Disabled,
        }
    }

    fn refresh(&mut self, manager: &Manager) {
        let _ = manager.refresh(&mut self.inner);
    }

    fn percent(&self) -> u32 {
        self.inner.state_of_charge().get::<ratio::percent>() as u32
    }

    fn blink_enabled(&self) -> bool {
        self.blink != Blink::Disabled
    }

    fn print(&mut self) {
        let percent = self.percent();

        self.print_color();
        print!("{}%", percent);
        self.print_icon();

        // End of color
        print!("%{{F-}}");
    }

    fn print_color(&mut self) {
        let percent = self.percent();
        let state = self.inner.state();

        if percent <= BAT_CRIT {
            if state == State::Charging {
                // Critical battery level, but charging
                print!("%{{F{}}}", BAT_CRIT_COLOR);
                self.blink = Blink::Disabled;
            } else {
                // Critical battery level, enable blinking
                if self.blink == Blink::Disabled {
                    self.blink = Blink::Show;
                }

                if self.blink == Blink::Show {
                    print!("%{{F{}}}", BAT_CRIT_COLOR);
                    self.blink = Blink::Hide;
                } else if self.blink == Blink::Hide {
                    print!("%{{F{}}}", BACKGROUND_COLOR);
                    self.blink = Blink::Show;
                }
            }
        } else if percent <= BAT_WARN {
            print!("%{{F{}}}", BAT_WARN_COLOR);
            self.blink = Blink::Disabled;
        } else if percent <= BAT_GOOD {
            print!("%{{F{}}}", BAT_GOOD_COLOR);
            self.blink = Blink::Disabled;
        } else {
            print!("%{{F{}}}", BAT_FULL_COLOR);
            self.blink = Blink::Disabled;
        }
    }

    fn print_icon(&self) {
        let percent = self.percent();
        let state = self.inner.state();

        let icons = &["", "", "", "", ""];
        let mut idx = (percent / 20) as usize;

        if idx >= icons.len() {
            idx = icons.len() - 1;
        }

        print!(" {}", icons[idx]);

        if state == State::Full
            || (state == State::Charging && percent >= BAT_FULL)
        {
            print!(" ");
        } else if state == State::Charging {
            print!(" ");
        }
    }
}
