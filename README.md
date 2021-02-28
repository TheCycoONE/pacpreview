# pacpreview
Helpful unified package info screen for pacman

I was looking for a better info screen to use as the preview window for fzf over the pacman package list.

At this time the tool is designed for my personal use. It DOES NOT parse pacman.conf, and uses hard coded architecture, repo list and paths.

## Examples
### Package is not installed:
```
core/linux-aarch64 5.11.1-1

The Linux Kernel and modules - AArch64 multi-platform

Opt Depends:
    crda: to set the correct wireless channels of your country [installed]
Depends:
    coreutils [installed]
    linux-firmware [installed]
    kmod [installed]
    mkinitcpio 0.7 [installed]
```

### Package is installed and up to date:
```
community/sway 1:1.5.1-1 [installed]

Tiling Wayland compositor and replacement for the i3 window manager

Installed Reason: explicit
Opt Depends:
    alacritty: Terminal emulator used by the default config [installed]
    dmenu: Application launcher [installed]
    grim: Screenshot utility [installed]
    i3status: Status line
    mako: Lightweight notification daemon [installed]
    slurp: Select a region
    swayidle: Idle management daemon [installed]
    swaylock: Screen locker [installed]
    wallutils: Timed wallpapers
    waybar: Highly customizable bar [installed]
    xorg-server-xwayland: X11 support [satisfied by xorg-xwayland]
Depends:
    cairo [installed]
    gdk-pixbuf2 [installed]
    json-c [installed]
    pango [installed]
    polkit [installed]
    pcre [installed]
    swaybg [installed]
    ttf-font [satisfied by noto-fonts]
    wlroots [installed]
```

### Package is outdated
```
extra/gimp 2.10.22-2 [~installed]

GNU Image Manipulation Program

Installed Version: 2.10.22-1
Installed Reason: explicit
Opt Depends:
    gutenprint: for sophisticated printing only as gimp has built-in cups print support
    poppler-glib: for pdf support [installed]
    alsa-lib: for MIDI event controller module [installed]
    curl: for URI support [installed]
    ghostscript: for postscript support [installed]
Depends:
    babl [installed]
    dbus-glib [installed]
    desktop-file-utils [installed]
    gegl [installed]
    glib-networking [installed]
    hicolor-icon-theme [installed]
    openjpeg2 [installed]
    lcms2 [installed]
    libheif [installed]
    libexif [installed]
    libgudev [installed]
    libmng [installed]
    libmypaint [installed]
    librsvg [installed]
    libwebp [installed]
    libwmf [installed]
    libxmu [installed]
    libxpm [installed]
    mypaint-brushes1 [installed]
    openexr [installed]
    poppler-data [installed]
    gtk2 [installed]
    graphviz [installed]
```
