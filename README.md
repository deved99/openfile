# openfile
`openfile` opens each argument with the
command defined in `~/.config/openfile/openfile.json`.
The output is like:
```
EXEC: zathura
ARGS: ["pdfs/environ.pdf"]
```

This is an example configuration:
```
{
  "filetypes": {
	"mp4": "video",
	"mkv": "video",
	"jpg": "image",
    "pdf": "document",
	"": "raw"
  },
  "commands": {
    "document": ["zathura"],
	"video": ["mpv"],
	"image": ["sxiv"],
	"raw": ["alacritty", "-e", "nvim"]
  }
}
```
`filetypes` maps each extension to a category,
while `command` maps each category to the
respective command.
