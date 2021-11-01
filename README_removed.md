
Systemd user Unit
=================

Kya's first-run also provides a systemd Unit that it saves in the user's .config/systemd/user
directory, which is one of the locations where user services are saved. The kya service can be
enabled and launched without root privileges:

```
systemctl --user start kya
systemctl --user enable kya
```

If Kya is unresponsive, you might be able to find help in the systemd logs:

`journalctl --user -u kya`

You can use d or Ctrl+d to scroll to the bottom.