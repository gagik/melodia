
use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use rust_gpiozero::input_devices::DigitalInputDevice;

pub struct Speaker<'a> {
    selem: Selem<'a>,
    default_channel: SelemChannelId,
    range: (f64, f64)
}

impl<'a> Speaker<'a> {
    pub fn new(mixer: &'a Mixer, selem_name: &'a str, channel: SelemChannelId) -> Self {
        let selem_id = SelemId::new(selem_name, 0);
        let selem = match mixer.find_selem(&selem_id) {
            Some(selem) => selem,
            None => panic!("No salem found."),
        };
        let range = selem.get_playback_volume_range();
        let range = (range.0 as f64, range.1 as f64);
        
        Speaker {
            selem,
            range,
            default_channel: channel,
        }
    }
    pub fn get_volume(&self) -> i64 {
        match self.selem.get_playback_volume(self.default_channel) {
            Ok(vol) => vol,
            Err(err) => panic!("Volume access error. {}", err),
        }
    }
    pub fn get_volume_percent(&self) -> f64 {
        let volume = self.get_volume();
        self.range.0 + volume as f64 / self.range.1 * 100.0
    }
    pub fn set_volume(&self, volume: i64) -> () {
        match self.selem.set_playback_volume_all(volume) {
            Ok(_) => (),
            Err(err) => panic!("Volume set error. {}", err),
        }
    }
    pub fn set_volume_percent(&self, volume: f64) -> () {
        let absolute_volume: f64 = (volume * self.range.1 / 100.0) - self.range.0;
        // This might rounddown, causing some inaccuracy but nothing catastrophic.
        self.set_volume(absolute_volume as i64)
    }

    // A test that everything works as it is supposed to.
    pub fn test(&self) -> bool {
        let old_volume = self.get_volume();

        let new_volume = 12;

        self.set_volume(new_volume);
        if old_volume != new_volume  {
            return false;
        };

        self.set_volume_percent(self.get_volume_percent());
        if self.get_volume() != new_volume {
            return false;
        };
        return true;
    }
}

pub struct Knob {
    left : DigitalInputDevice,
    right : DigitalInputDevice,
    ticks: u8,
    required_ticks: u8
}

impl Knob {
    pub fn new(left_pin:u8, right_pin:u8, required_ticks:u8) -> Self {
        Knob {
            left: DigitalInputDevice::new_with_pullup(left_pin),
            right: DigitalInputDevice::new_with_pullup(right_pin),
            ticks: 0,
            required_ticks
        }
    }
    pub fn update(&mut self) -> bool {
        let l_status : bool  = self.left.is_active();
        let r_status : bool  = self.right.is_active();
        if l_status && r_status {
            self.ticks += 1;

            if self.ticks <= self.required_ticks {
                print!("+\n");
                self.ticks = 0;
            }
        } else if l_status && !r_status {
            self.ticks = if self.ticks <= 0 { 0 } else {self.ticks - 1};

            if self.ticks <= self.required_ticks {
                print!("-\n");
                self.ticks = 0;
            }
        }
        return false;
    }
}