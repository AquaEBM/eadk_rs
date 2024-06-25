# `eadk_rs`

Safe bindings to [Numworks](https://www.numworks.com/)' Epsilon App Development Kit.

## How to create a custom App for your Numworks Calculator

- In your terminal, create a new Rust binary project with cargo:

```shell
cargo new --bin epsilon_app
cd epsilon_app
```

- You might want to add this crate as a dependency in the newly created `Cargo.toml` file:

```toml
# Cargo.toml
[package]

# ...

[dependencies]

# ...
eadk = { git = "https://github.com/AquaEBM/eadk_rs.git" }
```

- Copy the `icon.nwi` at the root of this repository to the `src` directory of your project.

- Then copy the following example code into `src/main.rs`:

```rust
//! Template Epsilon App
#![no_std]
#![no_main]

use eadk::{ion::*, kandinsky::*};

#[used]
#[link_section = ".rodata.eadk_app_name"]
/// null-terminated string used as the app's name
static APP_NAME: [u8; 14] = *b"YOUR_APP_NAME\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
/// API level of the app, always zero
static API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
/// Bytes of an LZ4-compressed RGB565 55x56 pixels image.
static ICON: [u8; 4250] = *include_bytes!("icon.nwi");

#[no_mangle]
fn main() {

    const COL: Color = Color::from_rgb([248, 180, 48]);
    const BG_COL: Color = Color::from_rgb([78, 78, 78]);

    fill_rect( Rect { point: Point { x: 0, y: 18 }, w: 320, h: 222 }, BG_COL);

    let string = b"HELLO NUMWORKS\0";

    // draw "HELLO NUMWORKS" on the center of the screen.
    unsafe {
        draw_string_unchecked(
            string.as_ptr(),
            Point {
                x: 160 - (string.len() - 1) as i16 * 3,
                y: 129,
            },
            false,
            COL,
            BG_COL,
        )
    };

    // Exit the app when the back button is pressed
    while !KeyboardState::scan().key_down(Key::Back) {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Freeze on panic
    loop {}
}
```

- Then, build your app:

```shell
cargo build -r --target=thumbv7em-none-eabihf
```

- Finally, connect your calculator via USB to your device, and follow the instructions in [this website](https://my.numworks.com/apps) to upload it. The file you need to upload is exactly:
```
target/thumbv7em-none-eabihf/release/epsilon_app
```
