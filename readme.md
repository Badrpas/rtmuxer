# rTMUXer

Use `yaml` config to define `tmux` sessions and windows layout.


### Example

```yaml
# create a session named `foo`
foo:

  # create a window named `bar`
  bar:
    # set current dir to `/www`
    cwd: /www
    # run a command in the window
    cmd: http-server .
    # set PORT environment variable
    env:
      PORT: 8080
```

