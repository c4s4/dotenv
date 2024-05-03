# Dotenv

This tool runs given command in an environment defined in a dotenv file.

## Installation

### Unix users (Linux and MacOSX)

Unix users may download and install latest *dotenv* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/dotenv/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/dotenv/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *dotenv* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/dotenv/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *dotenv*.

## Usage

You can get help in terminal with command `dotenv --help`:

```
$ dotenv --help
Run command in environment loaded from dotenv file

Usage: dotenv [OPTIONS] [CMD]...

Arguments:
  [CMD]...  The command to run

Options:
  -e, --env <ENV>  The dotenv file to load [default: .env]
  -c, --clear      Clear environment before loading env file
  -s, --shell      Run command in a shell
  -h, --help       Print help
```

To run command *foo* with its arguments *args...* in the environment defined in *.env* file in current directory, type:

```bash
dotenv foo args...
```

*.env* file might define environment such as:

```bash
FOO=BAR
SPAM=EGGS
```

Command *foo* will then be able to access the environment defined in *.env* file.

You can specify another dotenv file with `--env file` option:

```bash
dotenv --env /etc/foo.env foo args...
```

You can also load multiple dotenv files, repeating `--env file` option on command line :

```bash
dotenv --env /etc/foo.env --env /etc/bar.env foo args...
```

The environment files are evaluated in the order of the command line, so that in previous example variables defined in *bar.env* would overwrite those defined in *foo.env*.

You can delete all environment variables before loading those defined in dotenv files with `--clear` option. For instance, to print only environment defined in *.env* file, you could run:

```bash
dotenv --clear --env .env /usr/bin/env
FOO=SPAM
BAR=EGGS
```

## Shell

Let's say you have following *.env* file:

```bash
FOO=BAR
```

You would probably expect following:

```bash
$ dotenv echo $FOO
BAR
```

But this is not what happens:

```bash
$ dotenv echo $FOO

```

Because `$FOO` will be evaluated by the shell before running *dotenv* and replaced with its value on command line. To have expected behavior, you must run:

```bash
$ dotenv --shell 'echo $FOO'
BAR
```

In this case, command `echo $FOO` will not be evaluated until it runs in a shell. This shell will run in environment defined with dotenv file passed on command line and will print expected value on the console.

Note that you could try to obtain expected result with command `dotenv 'echo $FOO'`, but this won't work because *dotenv* will try to run command `echo $FOO` which doesn't exist.

On Unix, *dotenv* will run command in a shell with `sh -c command` and `cmd /c command` on Windows.

*Enjoy!*
