# rustcarnum

## How To Contribute

Create new crates with `cargo new --lib ./crates/new_crate --vcs none`

# Tools

## Artconverter

The artconverter is almost verbatim taken from https://github.com/AxelStrem/ArtConverter

## Reverse engineering the .dat format

In his talks about the arcanum data (https://www.youtube.com/watch?v=VYw4ln0jxUY, https://www.youtube.com/watch?v=bmz6XSd7xGM), Tim mentions:
* All the files will be stored sequentially
* While storing files, they keep a table of metadata (e.g. start offsets)
* when finishing, the filetable will be written to the end
* The start of the file will be set to the start of the filetable
* sequential files overwrite previous versions, so if art/splashscreen.ART is in arcanum2.dat, it will overwrite the art/splashscreen.ART in arcanum1.dat

He also mentions, data is divided in 4 categories

Observations:
* Every file starts with `78 da` which does not seem to be a file offset
* filenames in tables have a three letter file ending followed by `0x00` and then 24 bytes of data until the next filename begins
* sometimes there seems to be a base folder, like `art`, followed by `art\item\` and lastly `art\item\P_tesla_gun.ART`

this: https://rpgcodex.net/forums/threads/arcanum-multiverse-edition.114150/page-30 mentions:
* there is a 28 byte footer at the end, consisting of a 16character uuid, a "magic number" that should be `1TAD` to mark a valid file, size of bytes of all filenames together and sum of bytes of everything written that isn't a deflate stream
* every entry consists of a filename, a crc or offset value, a type (0x00000400 for dir, 0x00000002 for deflate stream and 0x00000001 for a directly stored information), original size, deflate size and offset