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
~/code/a11ynull-prevalence$ cargo rr
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/a11ynull-prevalence`
Application                        Toolkit  Total      Null refs  Active frames
---------------------------------- -------- ---------- ---------- ----------
ibus-ui-gtk3                       gtk      1          0          0
xfce4-session                      gtk      1          0          0
xdg-desktop-portal-gtk             gtk      1          0          0
ibus-extension-gtk3                gtk      1          0          0
xfwm4                              gtk      2          0          0
xfsettingsd                        gtk      1          0          0
xfce4-panel                        gtk      74         0          0
Thunar                             gtk      1          0          0
xfdesktop                          gtk      4          0          0
xfce4-notifyd                      gtk      1          0          0
wrapper-2.0                        gtk      1          0          0
wrapper-2.0                        gtk      1          0          0
wrapper-2.0                        gtk      1          0          0
xfce4-power-manager                gtk      1          0          0
polkit-mate-authentication-agent-1 gtk      1          0          0
update-notifier                    gtk      1          0          0
xfce4-clipman                      gtk      1          0          0
nm-applet                          gtk      1          0          0
blueman-applet                     gtk      1          0          0
blueman-tray                       gtk      1          0          0
Firefox                            Gecko    9673       0          0
signal-desktop                     Chromium 3          1          0
element-desktop                    Chromium 3          1          0
Thunderbird                        Gecko    4444       0          0
Discord                            Chromium 3          1          0
xfce4-terminal                     gtk      51         0          1
```

## License
MIT License
