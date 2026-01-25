# DB Architecture

## Song Identification

In Moonsound, we have a challenge with identifying songs:

1) A song's filesystem path (`fsPath`) can change when the user moves the file
2) A song's hash (IE `md5`) is expensive to compute and is likely to change
3) Using the last modified time (`mtime`) and size of the file (`size`) isn't reliable because tag metadata is often stored in the file itself

However, there is a bit of a silver bullet (with two major asterisks):

[`inode`s](https://en.wikipedia.org/wiki/Inode).

These act as a reference point for each file in the filesystem for POSIX systems. Each file has a unique ID that persists for each file on the device, even across file moves and content modifications. Even Windows has similar equivilant APIs.

However, the two asterisks are:

1) The `inode` needs to be tracked alongside `device_id`, which is drive (partition?) specific. Meaning that if the user changes from, IE, C:// to D:// the files will lose the `inode` number
2) Some programs, like song metadata editors, will often write to a `/tmp` location and then replace the original song with the new file; which itself will have a new `inode` file.

Long-story-short, we can track the `inode` number as a heuristic of how to identify a song that has changed.

We can use this package to track `inode` numbers:

https://docs.rs/file-id/latest/file_id/

## File Watching

See a list of known problems with file watching, their docs are better than mine will be: https://docs.rs/notify/latest/notify/#known-problems