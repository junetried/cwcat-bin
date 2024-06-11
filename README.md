# cwcat-bin
A CLI program that can concatenate videos from the game
[Content Warning](https://store.steampowered.com/app/2881650/Content_Warning/).

See [the cwcat repository](https://github.com/junetried/cwcat) if you want to
know how it works.

```text
A CLI program that can concatenate videos from the game Content Warning

Usage: cwcat [OPTIONS] -i <INPUT DIRECTORY>

Options:
  -i <INPUT DIRECTORY>
          recording directory to concatenate

  -o <OUTPUT FILE>
          the file to write the concatenated webm to

  -k, --keep-second-track
          keep the second audio track, which contains only game audio

  -f, --force
          if output file exists, overwrite anyway

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Usage
You should find the `rec` directory, which is the temporary directory that the
game saves recordings in. This directory and its contents are deleted when you
quit the game, so be careful! The default location for this directory looks
something like this:

**Windows:** `%LOCALAPPDATA%\Temp\rec`

**Linux:** `~/.local/share/Steam/steamapps/compatdata/2881650/pfx/drive_c/users/steamuser/Temp/rec`

Each subdirectory here contains a recording from a different camera. To find the
exact recording you want, you might want to look through the clips in it, found
in its subdirectories in a file called `output.webm`. It might also be helpful
to sort by creation date.

Once you've found the recording, you can put it into this program with no
arguments to get the number of clips and the order they were created:

```text
cwcat -i ~/.local/share/Steam/steamapps/compatdata/2881650/pfx/drive_c/users/steamuser/Temp/rec/MY_RECORDING
```

or, save them to a single, finalized video:

```text
cwcat -i ~/.local/share/Steam/steamapps/compatdata/2881650/pfx/drive_c/users/steamuser/Temp/rec/MY_RECORDING -o my_recovered_recording.webm
```