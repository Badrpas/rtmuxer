# rTMUXer

Use `yaml` config to define `tmux` sessions and windows layout.

### Install
```shell
$ cargo install --git https://github.com/Badrpas/rtmuxer.git
```

### Usage

#### With specific config file(s):

Define config file:
```yaml
# server-conf.yaml

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
```shell
$ rtmuxer server-conf.yaml
```
#### Implicitly use config in current dir:

Filepath can be omitted if config named `rtmuxer.yaml` exists in current working directory:
```yaml
# rtmuxer.yaml
hello:
  world:
    cmd: echo hi
```
```shell
$ rtumxer
```
