# Subtitle-Sync
A small CLI Utility that helps tackling with out-of-sync subtitles

## How to use
Let's say you have a subtitle file `some-awesome-show-S01E01.srt`. But when you play the video
with it, you notice the subtitle slightly goes out of sync; without being able to adjust it
by a simple offset.

This library aims at dealing with this kind of trouble.
Firstly, open the video and find a sentence (the later in the video the better). Write down
its time code. For instance, `00:43:50,200`.

Secondly, open the .srt file and find the time code of this same sentence. For
instance, `00:40:50,652`.

You can now use this script to adapt the file, by running in a terminal :
```bash
$ subtitle-sync --input some-awesome-show-S01E01.srt --from 00:40:50,652 --to 00:43:50,200
Computed ratio is 1.0732654
Sync finished in 6ms. Wrote to file new-some-awesome-show-S01.srt
```