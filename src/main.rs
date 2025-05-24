mod wheel_of_fortune;

use leptos::prelude::*;
use wheel_of_fortune::WheelOfFortune;
fn main() {
    mount_to_body(|| view! {
        <WheelOfFortune/>
    })
}