
use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use rppal::gpio::{Gpio, Level, Mode, InputPin};

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
    left : InputPin,
    right: InputPin,
    gpio:Gpio,
    state: u8
}

impl Knob {
    pub fn new(gpio:Gpio, left_pin:u8, right_pin:u8) -> Self {
        Knob {
            left: gpio.get(left_pin).unwrap().into_input(),
            right: gpio.get(right_pin).unwrap().into_input(),
            state: 0,
            gpio: gpio
        }
    }
    pub fn update(&mut self) -> i32 {
       let mut s = self.state;

       if self.left.is_low() {
            s |= 0b100;
       }

        if self.right.is_low() {
            s |= 0b1000;
        }

        self.state = s >> 2;

        match s {
            0b0001 | 0b0111 | 0b1000 | 0b1110 => -1,
            0b0010 | 0b0100 | 0b1011 | 0b1101 => 1,
            _ => 0,
        }
    }
}