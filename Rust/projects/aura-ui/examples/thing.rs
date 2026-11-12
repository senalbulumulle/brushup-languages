//      _                               _   _   ___ 
//     / \     _   _   _ __    __ _    | | | | |_ _|
//    / _ \   | | | | | '__|  / _` |   | | | |  | | 
//   / ___ \  | |_| | | |    | (_| |   | |_| |  | | 
//  /_/   \_\  \__,_| |_|     \__,_|    \___/  |___|
           
// this is an example code for aura-ui

use aura_ui::prelude::*;

fn main() {
    aura! {
        open "My First Aura App" size 640 by 460

        remember clicks = 0;
        remember stars = 0;

        paint {
            card {
                title "Aura Thing"
                whisper "This is a thing."
                line
                tag "Aura Speak"
                line
                say "Clicks" with clicks
                say "Stars" with stars
                line
                row {
                    press "Add click" => bump clicks
                    press "Add star" => bump stars
                    press "Reset" => set clicks 0
                    press "Night mode" => flip theme
                }
            }
        }
    }
}
