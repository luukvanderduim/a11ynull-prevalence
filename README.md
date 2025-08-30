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
Finished `release` profile [optimized] target(s) in 0.07s
 Running `target/release/a11ynull-prevalence`
Application                        Toolkit  Total      Null refs  Active states
---------------------------------- -------- ---------- ---------- ----------
ibus-ui-gtk3                       gtk      1          0          0
xfce4-session                      gtk      1          0          0
xdg-desktop-portal-gtk             gtk      1          0          0
ibus-extension-gtk3                gtk      1          0          0
xfwm4                              gtk      2          0          1
xfsettingsd                        gtk      1          0          0
xfce4-panel                        gtk      67         0          0
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
Thunderbird                        Gecko    2769       0          2
Firefox                            Gecko    7640       0          1
signal-desktop                     Chromium 3          1          0
```

## License
MIT License
