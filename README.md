File watcher for Linux that sends images to Gyazo. This allows you to use Gyazo with screenshot
programs that are more usable, such as [Spectacle](https://apps.kde.org/spectacle/).

Installation
============

Install using cargo.

`cargo install kya`

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
```

If you don't want the daemon to open the new image link in the browser, set `open_in_browser` to false.
