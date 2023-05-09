# bpg2hevc

Command line tool that allows you to losslessly convert compatible [BPG](https://bellard.org/bpg/) files to HEIC format that is more widely supported.
The tool itself handles extraction of [HEVC](https://en.wikipedia.org/wiki/High_Efficiency_Video_Coding) stream from BPG files. That stream can be consumed by other tools like [FFmpeg](https://ffmpeg.org/) or [MP4Box](https://github.com/gpac/gpac/wiki/MP4Box).

## Example

```
$ bpgenc sample.png -o sample.bpg
$ bpg2hevc sample.bpg > sample.hvc
$ MP4Box -add-image sample.hvc:primary -new sample.heic
```

## Limitations

The tool was created to process my own files, which are rather uniform, and may fail to handle arbitrary BPG files.

* Only basic BPG files are supported: no alpha, no animations, only one specific colourspace, etc.
* Most things are just hard coded - only picture width and height are handled carefully. BPG files that were encoded differently (e.g. non-default `-m` or `-b` or not x265 encoder) may fail to be converted.

## Installation

Download a pre-built executable from [Github releases](https://github.com/vi/bpg2hevc/releases) or install from source code with `cargo install --path .`  or `cargo install bpg2hevc`.

## CLI options

<details><summary> bpg2hevc --help output</summary>

```
ARGS:
    <path>
      BPG file to read and convert to HEVC raw stream to stdout.
      `MP4Box -add-image w.hvc:primary -new w.heic` would help to pack it as a HEIF image.

OPTIONS:
    -h, --help
      Prints help information.
```
</details>
