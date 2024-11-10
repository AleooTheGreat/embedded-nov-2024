//! Go to [Random Color Generator](https://randomwordgenerator.com/color.php)
//! Generate two colors and get the RGB encodings for them. These are the colors
//! you will need to display on the RGB LED.
//!
//! Your application should smoothly transition from one color to another. The colors will
//! be displayed sequentially for 3 seconds each, with a gradual transition period of 1 second.
//!
//! Keep in mind that the RGB LED is common anode.
//!
//! For displaying the color on the LED, PWM (Pulse Width Modulation) will need to be set up
//! on the pin. Connect them to pins: GPIO0 (Red), GPIO1 (Green), and
//! GPIO2 (Blue). (Hint: Pin 0 and 1 will share the same channel).

#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]

use defmt::*;
use embassy_rp::rom_data::double_funcs::dsin;
use embassy_executor::Spawner;
use embassy_rp::pwm;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2, PWM_SLICE0, PWM_SLICE1, PWM_SLICE2};
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    spawner.spawn(pwm_set_rgb(p.PWM_SLICE0, p.PWM_SLICE1, 
        p.PIN_0, p.PIN_1, p.PIN_2)).unwrap();
    // TODO 0 : Create the Config for the PWM that will drive the RGB LED.

    // TODO 1 : Modify the RGB values and loop through the configs to create a transition.
}

#[embassy_executor::task]
async fn pwm_set_rgb(slice_rg: PWM_SLICE0, slice_b: PWM_SLICE1, pin_r: PIN_0, pin_g: PIN_1, pin_b: PIN_2) {
    
    //config
    let mut config =pwm::Config::default();


    //color
    let color1 =(109, 63, 91);
    let color2 = (255, 164, 32);

    //config default
    config.compare_a = color1.0;
    config.compare_b = color1.1;
    config.top = 255;

    let mut swap_col = true;
    let mut pwm_rg = Pwm::new_output_ab(slice_rg, pin_r, pin_g, config.clone());

    config.compare_a  = color1.2;
    let mut pwm_b = Pwm::new_output_a(slice_b, pin_b, config.clone());

    let mut i = 0;    

    loop {
        config.compare_b = (color1.0 as f64 + (color2.0 as f64 - color1.0 as f64) * (dsin(i as f64) + 1.0) / 2.0) as u16;
        config.compare_a = (color1.1 as f64 + (color2.1 as f64 - color1.1 as f64) * (dsin(i as f64) + 1.0) / 2.0) as u16;
        pwm_rg.set_config(&config);

        config.compare_a = (color1.2 as f64 + (color2.2 as f64 - color1.2 as f64) * (dsin(i as f64) + 1.0) / 2.0) as u16;
        pwm_b.set_config(&config);

        Timer::after_millis(250).await;
        i += 1;

    }
}


