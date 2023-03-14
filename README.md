# Provided Tools

TODO

## Command

### Serial USB

Logs on USB Serial.
Can interact with wioterminal, see command `help` wich list the commands.

### UI **[WIP]**

Navigate between different screen (for ex: wifi, game, ...) with left and middle upper button.
Something like that, i think, ...

# To build the sources

Put the Wio Terminal in bootloader mode first and then to build and flash the firmware:

```
cargo hf2 --release
```

# Prerequisite

`cargo-hf2` for flashing the firmware.

```
cargo install cargo-hf2
```

# Reference

https://github.com/tomoyuki-nakabayashi/Embedded-Rust-from-Basics

https://tomo-wait-for-it-yuki.hatenablog.com/entry/2021/04/04/140831

- https://tomo--wait--for--it--yuki-hatenablog-com.translate.goog/entry/2021/04/04/140831?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=en&_x_tr_pto=wapp

https://tomoyuki-nakabayashi.github.io/embedded-rust-techniques/01-introduction/introduction.html

- https://tomoyuki--nakabayashi-github-io.translate.goog/embedded-rust-techniques/01-introduction/introduction.html?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=en&_x_tr_pto=wapp

https://tomoyuki-nakabayashi.github.io/book/intro/index.html

- https://tomoyuki--nakabayashi-github-io.translate.goog/book/intro/index.html?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=fr&_x_tr_pto=wapp

https://os.phil-opp.com/

https://github.com/rtic-rs/cortex-m-rtic

https://doc-rust--jp-rs.translate.goog/?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=fr&_x_tr_pto=wapp
