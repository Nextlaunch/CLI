![Nextlaunch Logo](https://raw.githubusercontent.com/Nextlaunch/media-assets/main/Logo-red.png)
# NextLaunch CLI
An advanced terminal app to track upcoming launches from SpaceX, Nasa, Roscosmos, ULA, and many other organizations across the globe!

## Features
- Power efficient.
- Feature rich.
- Easy to use.
- Easy to understand.
- Regular feed updates.
- Beautiful, text based user interface.
- Interactive user interface.
- Powered by a community of space nerds!
- Follow launches on the go with [Space Launch Now](https://spacelaunchnow.me) (Using our handy QR codes).
- Check out the sweet logos of these LSPs with our logo panel.
- Get statistics about the launch directly in your terminal.

## Usage
Simply launch the program from your terminal, or file explorer, and it will work out of the box.

For help regarding how to interact with NextLaunch, please press F1 when you are using the application, and it will display a helpful popup window.

## Installation
To see how to install NextLaunch, please check the [latest release](https://github.com/Nextlaunch/CLI/releases/latest)


## Why should I upgrade from V1 to V2?
Well, firstly, V1 just really sucks. It is slow, inefficient, and prone to crashing or panicking. Those of you who use the program have more than likely seen a panic since installing it.
V2 doesnt panic, it just doesnt. Everything is coded in a crash resistant way.

Secondly, V2 is multi-threaded, meaning that it is able to do more, in the same amount of time.

Third of all, V2 takes advantage of an updated version of the Launch Library API which it relies on for launch information, as well as a non-retired version of the Space News API which it relies on for news data.

Fourth, finally, and most certainly the best reason to switch, V2 offers an actual user interface, that's right a user interface in the terminal. It is powered by your keyboard and is simple to use and easy to remember.

## Screenshots
### Here it is in action!
![image](https://user-images.githubusercontent.com/63651404/110542823-76162a00-8121-11eb-8f3b-42021a8da190.png)



## Credits
Without the following people, services, and open-source libraries, NextLaunch would not have been possible.
Thank you to everyone on this list from the bottom of my heart for helping me make this program,
and putting up with my constant requests for comments on design and style.


### Data Providers
|Category|Name|Link|
|:---:|:---:|:---:|
|News|Space Flight News API|https://thespacedevs.com/snapi|
|Launches|Launch Library 2|https://thespacedevs.com/llapi|
|Telemetry|Launch Dashboard|https://github.com/shahar603/Launch-Dashboard-API|


### Interface (Alphabetical)
|Name|Link|
|:---:|:---:|
|Accusitive|https://github.com/accusitive
|Jas777|https://github.com/jas777
|Nosu|https://twitter.com/Nosudrum|
|Starman|No Link Specified|
|Zane|https://github.com/AnotherZane

### Alpha Testers (Alphabetical)
|Name|Link|
|:---:|:---:|
|Nosu|https://twitter.com/Nosudrum|
|Starman|No Link Specified|


### Third Party Code
Without these projects, Nextlaunch wouldnt be possible.

|Package| Build Script | Installer | Executable |
|:--:|:--:|:--:|:--:|
| [Locale Config](https://crates.io/crates/locale_config) | | | X |
| [Tokio](https://crates.io/crates/tokio) | | | X |
| [Reqwest](https://crates.io/crates/reqwest) | X | X | X |
| [Serde](https://crates.io/crates/serde) | | | X |
| [Serde JSON](https://crates.io/crates/serde_json) | | | X |
| [Chrono](https://crates.io/crates/chrono) | | | X |
| [Bincode](https://crates.io/crates/bincode) | | | X |
| [Crossterm](https://crates.io/crates/crossterm) | | | X |
| [Tui](https://crates.io/crates/tui) | | | X |
| [Term Size](https://crates.io/crates/term_size) | | | X |
| [Webbrowser](https://github.com/codota/webbrowser-rs) | | | X |
| [Dirs-2](https://crates.io/crates/dirs-2) | X | X | X |
| [Image](https://crates.io/crates/image) | | | X |
| [Bytes](https://crates.io/crates/bytes) | | | X |
| [QR2Term](https://crates.io/crates/qr2term) | | | X |
| [Zip](https://crates.io/crates/zip) | X | X | X |

### Changelog

> \- __Thursday, March 24th 2022__ 
> 
> "Added support for QR Codes using the `m` key" 
> 
> "Added support for V1-style logos using the `F3` key" 
> 
> "Added support for LSP Stats using the `F2` key" 
> 
> "Re-introduced the build script"
>
> "Added initial launch auto-config script"
>
> "Added 'integrity' check (incomplete/nonfunctional) to launch procedure"
>
> "Began implementing language pack support properly"
> 
> "Various other miscellaneous changes in line with requirements for aforementioned features"