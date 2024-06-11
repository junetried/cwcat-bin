# cwcat-bin
A CLI program that can concatenate videos from the game
[Content Warning](https://store.steampowered.com/app/2881650/Content_Warning/).

See [the cwcat repository](https://github.com/junetried/cwcat) if you want to
know how it works.

```text
A CLI program that can concatenate videos from the game Content Warning

Usage: cwcat [OPTIONS] -i <INPUT DIRECTORY> -I <RECORDING NAME>

Options:
  -i <INPUT DIRECTORY>
          Recording directory to concatenate

  -I <RECORDING NAME>
          Recording to concatenate from default rec path

  -l, --list
          Print the clip details at the input directory and exit

  -L, --list-default
          List clips at the default rec path and exit

  -o <OUTPUT FILE>
          The file to write the concatenated webm to

  -k, --keep-second-track
          Keep the second audio track, which contains only game audio

  -f, --force
          If output file exists, overwrite anyway

  -r, --rec-path
          Print the default rec path and exit

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Usage
To find all videos currently in the game's rec directory, you can use the
command `cwcat -L`.

This prints the total number of videos, followed by the name, duration, and
creation date of each video:

```text
$ cwcat -L
Total of 1 videos

Name                                      Duration         Creation Date
e860da9b-46a0-447c-beb0-d2132dfd4e8b      0m 20s 964ms     11 Jun 2024 03:16:59 PM
```

From here, you can give the program the name of the desired video as an input.

To see video details, pass the `-l` flag or omit the `-o` flag. This prints the
number of clips, followed by each clip's name, duration, and offset from the
initial clip's creation time:

```text
$ cwcat -I e860da9b-46a0-447c-beb0-d2132dfd4e8b
Total of 6 clips, total duration is 0m 20s 964ms, created at 11 Jun 2024 03:16:59 PM

Name                                      Duration         Recording Time
900d301e-4121-44fe-a562-a0701f607ff9      0m 2s 261ms      +0m 0s 0ms
dc59c672-6e90-4670-9c8a-0f2d50707038      0m 1s 491ms      +0m 5s 33ms
e7155261-3de2-49f9-9e1e-d1efa502b68e      0m 2s 803ms      +0m 7s 53ms
a5df15b4-6f07-45fe-aff8-149a36c12e7a      0m 2s 803ms      +0m 9s 890ms
38a48509-65e6-4940-967e-c9b57c10ef34      0m 8s 595ms      +0m 15s 880ms
9a7acc6d-4fb5-4526-975c-0a5aa7a10956      0m 3s 11ms       +0m 27s 140ms
```

To save the video, add the `-o` flag with the path to the output file:

```text
$ cwcat -I e860da9b-46a0-447c-beb0-d2132dfd4e8b -o ./video.webm
Concatenated files (in 968.226 KB), saving to "./video.webm"
File written successfully.
```

### Input by path

You should find the `rec` directory, which is the temporary directory that the
game saves recordings in. This directory and its contents are deleted when you
quit the game, so be careful! The default location for this directory looks like
this:

**Windows:** `%LOCALAPPDATA%\Temp\rec`

**Linux:** `~/.local/share/Steam/steamapps/compatdata/2881650/pfx/drive_c/users/steamuser/Temp/rec`

Use `cwcat --rec-path` to find the default path to the rec path.

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