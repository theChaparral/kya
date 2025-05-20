File watcher for Linux that sends images to Gyazo. This allows you to use Gyazo with screenshot
programs that are more usable, such as [Spectacle](https://apps.kde.org/spectacle/).

Installation
============

Install using cargo.

`cargo install kya`

On Arch Linux install the [**kya** package](https://aur.archlinux.org/packages/kya) from AUR.

Perform the first-run setup:

`kya-for-gyazo --first-run`

Then edit the configuration file at `.config/kya` in your home directory.
Kya requires an access token from the Gyazo API.
Receiving an access token is fast and painless, simply go to
[the developer page](https://gyazo.com/oauth/applications/) and create a new app.
You can leave the callback URL as "http://example.com" or whatever you like.

Edit the configuration TOML file with the access token and provide the location of where screenshots
are saved. Kya will watch for any new files that show up **while Kya is running.** Configuration example:

```
access_token = "SomethingSomething"
directory = "/home/gert/Pictures/Screenshot"
open_in_browser = true
delete_after_upload = false
```

If you don't want the daemon to open the new image link in the browser, set `open_in_browser` to false.
The `delete_after_upload` option in the configuration file (`true` or `false`) determines the default behavior for deleting files after upload. This behavior can be overridden for a specific run using the command-line flag described below.

Command-line Options
====================

Kya supports the following command-line options:

*   `-d`, `--delete`: If this flag is provided, the screenshot will be deleted from your local machine after a successful upload to Gyazo. If this flag is not provided, the image will be kept on your local machine, regardless of the `delete_after_upload` setting in the configuration file.
*   `--first-run`: Initializes the configuration file.
*   `--create-user-unit`: Creates a systemd user service file for automatic startup.
*   `--help`: Displays the help message with all available command-line options.
*   `--version`: Displays the application version.

Automatically starting Kya
==========================

Kya is designed to be started automatically, such as by KDE autostart, or GNOME startup applications.
You can also add it to .xinitrc, for example alongside KDE:

```
kya-for-gyazo &
exec startplasma-x11
```

Kya does a check to prevent running multiple instances of itself at once.

