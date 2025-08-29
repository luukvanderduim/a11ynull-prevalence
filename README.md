# a11ynull-prevalence

Get insight in the prevalence of null-references in accessible applications on the Linux-desktop.

## Clone, Build and Run

### Clone

```bash
git clone https://github.com/luukvanderduim/a11ynull-prevalence.git
```

### Build & Run

```bash
cd a11ynull-prevalence
cargo run --release
```

## Example output

```bash
Application                         Toolkit  Total        Null refs
----------------------------------- -------- ------------ ------------
xfce4-session                       gtk      1            0
ibus-ui-gtk3                        gtk      1            0
xdg-desktop-portal-gtk              gtk      1            0
ibus-extension-gtk3                 gtk      1            0
xfwm4                               gtk      2            0
xfsettingsd                         gtk      1            0
xfce4-panel                         gtk      73           0
Thunar                              gtk      1            0
xfdesktop                           gtk      4            0
xfce4-notifyd                       gtk      1            0
wrapper-2.0                         gtk      1            0
wrapper-2.0                         gtk      1            0
wrapper-2.0                         gtk      1            0
xfce4-power-manager                 gtk      1            0
polkit-gnome-authentication-agent-1 gtk      1            0
update-notifier                     gtk      1            0
nm-applet                           gtk      1            0
xfce4-clipman                       gtk      1            0
blueman-applet                      gtk      1            0
blueman-tray                        gtk      1            0
Thunderbird                         Gecko    2705         0
Firefox                             Gecko    12422        0
xfrun4                              gtk      510          0
element-desktop                     Chromium 2            1
signal-desktop                      Chromium 2            1
```

## License
MIT License
