# consolet
Pixel art in Terminal

## Controls Cheat Sheet

### General Controls

|     |     |
| --- | --- |
| Key(s) | Action |
| **:** | Enter **Command Mode** |
| **h** | Show/Hide Help Screen |
| **q** or **Ctrl+c** | Quit the application |
| **Ctrl+z** | Undo last action |
| **Ctrl+y** | Redo last action |
| **p** | Hide/Show the side panel (palette, tools, layers) |
| **Tab** | Cycle focus between Color Palette, Tool Palette, and Canvas |

### Drawing Mode

|     |     |
| --- | --- |
| Key(s) | Action |
| **Arrow Keys** | Move the cursor |
| **Spacebar** or **Enter** | Draw with the current tool/color |
| **Backspace** | Erase with the current pen shape and size |
| **f** | Fill an area with the current color |
| **c** | Pick color from the active layer under the cursor |
| **s** | Cycle through the four symmetry modes |
| **g** | Hold to use the Spray tool |

### Brush & Canvas Controls

|     |     |
| --- | --- |
| Key(s) | Action |
| **\[** or **\-** | Decrease pen size |
| **\]** or **\=** | Increase pen size |
| **,** or **\_** | Decrease opacity |
| **.** or **+** | Increase opacity |
| **' '** (Apostrophe) | Toggle pen shape (Circular/Square) |
| **w**, **a**, **s**, **d** | Pan the canvas view (Up, Left, Down, Right) |
| **PageUp** / **PageDown** | Zoom in / Zoom out |

### Layer Controls

|     |     |
| --- | --- |
| Key(s) | Action |
| **Alt + Up Arrow** | Select the layer above |
| **Alt + Down Arrow** | Select the layer below |
| **Alt + a** | Add a new layer above the active one |
| **Alt + d** | Delete the active layer |
| **Alt + v** | Toggle visibility of the active layer |
| **Alt + k** | Move the active layer up in the stack |
| **Alt + j** | Move the active layer down in the stack |

### Onion Skinning Controls

|     |     |
| --- | --- |
| Key(s) | Action |
| **i** | Toggle Onion Skinning on/off |
| **u** | Increase the opacity of the onion skin layer |
| **y** | Decrease the opacity of the onion skin layer |

### Mouse Controls

|     |     |
| --- | --- |
| Action | Description |
| **Left-Click & Drag** | Draw on the canvas |
| **Right-Click & Drag** | Erase on the canvas |
| **Middle-Click & Drag** | Pan the canvas view |
| **Scroll Wheel Up/Down** | Change pen size (default) or opacity (configurable) |
| **Click in Panels** | Select a color, tool, or layer |
| **Scroll in Panels** | Scroll through the color palette or layer list |

* * *

## Command Mode Reference

Press **:** to enter Command Mode. Type a command and press Enter. Most commands that change settings can have their changes saved permanently by adding a --save flag at the end (e.g., pen\_size=5 --save).

### File & Application

|     |     |     |
| --- | --- | --- |
| Command | Usage / Example | Description |
| save | save "my\_art" or save --explorer | Saves the project. Opens a file browser if no name is given. |
| load | load "my\_art" or load --explorer | Loads a .consolet project file. Opens file browser if no name. |
| export | export -o "art.png" -u 4 or export --explorer | Exports the canvas to a PNG. Use -o for output, -u for scale. |
| quit | quit | Exits the application. |

### Canvas & Drawing

|     |     |     |
| --- | --- | --- |
| Command | Usage / Example | Description |
| resize | resize=64x48 | Resizes the canvas to a new width and height. |
| clear | clear | Clears the currently active layer. |
| pen\_size | pen\_size=3 | Sets the pen/eraser size. |
| opacity | opacity=0.5 | Sets the drawing opacity (0.0 to 1.0). |
| pen\_shape | pen\_shape=Square | Sets the pen shape. Options: Circular, Square. |
| symmetry\_mode | symmetry\_mode=Vertical | Sets the symmetry mode. Options: Off, Vertical, Horizontal. |
| shade\_factor | shade\_factor=0.1 | Sets the strength of the Lighter/Darker tools. |
| spray\_size | spray\_size=10 | Sets the radius of the spray tool. |
| spray\_intensity | spray\_intensity=0.2 | Sets the density of the spray tool. |
| #RRGGBB | #FF0000 | Sets the current color to the given hex code. |

### Layers & Onion Skinning

|     |     |     |
| --- | --- | --- |
| Command | Usage / Example | Description |
| add\_layer | add\_layer | Adds a new layer. |
| delete\_layer | delete\_layer | Deletes the active layer. |
| rename\_layer | rename\_layer=Background | Renames the active layer. |
| layer\_opacity | layer\_opacity=0.75 | Sets the opacity of the entire active layer. |
| merge\_down | merge\_down | Merges the active layer with the one below it. |
| export\_mode | export\_mode=separate | Sets PNG export mode. Options: united (one file), separate (one file per layer). |
| onion\_skin | onion\_skin=true | Toggles onion skinning. Options: true, false. |
| onion\_opacity | onion\_opacity=0.25 | Sets the opacity of the onion skin preview. |

### Palettes

|     |     |     |
| --- | --- | --- |
| Command | Usage / Example | Description |
| colorpalette:<name> | colorpalette:pico8 or colorpalette:pico8 --add | Switches to a loaded palette. Use --add to merge instead of replace. |
| savepalette:<name> | savepalette:my\_palette | Saves the current color palette to a file. |
| import palette | import palette --explorer or import palette "path/to/file.consolet" | Imports a .consolet palette file into the application. |
| colorpalette\_image | colorpalette\_image --explorer or colorpalette\_image --explorer --add | Opens a browser to generate a palette from an image. --add merges it. |
| colorpalette\_image save | colorpalette\_image save "MyImagePalette" | Saves the last palette generated from an image. |

### Configuration & Scripts

|     |     |     |
| --- | --- | --- |
| Command | Usage / Example | Description |
| config\_save | config\_save | Saves all current settings to the config.json file. |
| config\_reset | config\_reset | Resets all settings to their default values. |
| keybindings\_reset | keybindings\_reset | Resets all keybindings to their defaults. |
| script <name> | script myscript | Runs a script from the scripts folder. |
| script\_save | script\_save | Saves the content of the current script editor. |

* * *

## Configuration

Consolet is highly configurable. Upon first run, it will create a .consolet directory in your user's home folder. Inside, you will find:

- config.json: A human-readable file to tweak sensitivities, default modes, and more.
- keybindings.json: A file where you can remap every action to your preferred key combination.
- /palettes: A folder where your saved color palettes are stored.
- /saved\_projects: The default location for your saved artwork.
- /scripts: A folder for your .cscript automation files.

<br>
