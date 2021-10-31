File watcher that sends images to Gyazo. This allows you to use Gyazo with screenshot
programs that are more usable.

Installation
============

Install using cargo.

`cargo install kya`

Perform the first-run setup:

`kya-for-gyazo --first-run`

Then edit the configuration file. Kya requires an access token from the Gyazo API.
Receiving an access token is fast and painless, simply go to
[the developer page](https://gyazo.com/oauth/applications/) and create a new app.
You can leave the callback URL as "http://example.com" or whatever you like.

Edit the config file with the access token and provide the location of where screenshots
are saved. Kya will watch for any new files that show up **while Kya is running.**

Systemd user Unit
=================

Kya's first-run also provides a systemd Unit that it saves in the user's .config/systemd/user
directory, which is one of the locations where user services are saved. The kya service can be
enabled and launched without root privileges:

```systemctl --user start kya
systemctl --user enable kya```

If Kya is unresponsive, you might be able to find help in the systemd logs:

`journalctl --user -u kya`

You can use d or Ctrl+d to scroll to the bottom.
