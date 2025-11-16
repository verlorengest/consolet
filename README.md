
## Pixel art app in terminal                                                                                    


<img width="2055" height="1444" alt="image" src="https://github.com/user-attachments/assets/705bd823-d0f6-4459-9072-00863b8f9a81" />

https://www.youtube.com/watch?v=ZMCFYdf-4Js&t=52s

You can launch it in 2 ways:
Download the release
Compile from source

## Download the release (Windows)

https://github.com/verlorengest/consolet/releases/download/1.0.0/consolet.exe
Open the program by double-clicking on it quickly in windows.
or
Right-click and open the context menu, click the Open button.

or click one time with left click of ur mouse, and press enter button in your keyboard.

## Compile from source

## 1\. Prerequisites

Before you begin, you need to have the Rust programming language toolchain installed on your system. The primary way to do this is by using rustup, which is the official Rust installer and version manager.

- **To install Rust:**

Open your terminal (or Command Prompt on Windows) and run the following command:



```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command will download and execute a script that installs rustc (the Rust compiler), cargo (the Rust package manager and build tool), and other necessary tools. Follow the on-screen instructions to complete the installation.

## 2\. Compiling the Project

Once you have Rust and Cargo installed, compiling the project is straightforward on all supported platforms.

1. **Clone the Repository (if you haven't already):**

If you have the project files in a Git repository, clone it to your local machine:



```
git clone <repository-url>
cd <repository-directory>
```

If you just have the source code files, make sure you are in the root directory of the project (the one containing the Cargo.toml file).
2. **Build the Project:**

To compile the project, use the cargo build command. For an optimized release version (which is recommended for regular use), use the --release flag:



```
cargo build --release
```

Cargo will automatically download and compile all the necessary dependencies.

The executable file will be located in the target/release/ directory.

## 3\. Platform-Specific Build Requirements

While cargo handles most of the heavy lifting, each operating system has some specific requirements to compile Rust applications, especially those with dependencies that might link to system libraries.

### Linux

On most Linux distributions, you will need to have the build-essential package or its equivalent installed, which includes gcc and other necessary build tools. Some dependencies might also require development headers for certain libraries.

- **On Debian-based systems (like Ubuntu):**



```
sudo apt update
sudo apt install build-essential libssl-dev pkg-config
```
- **On Fedora/CentOS/RHEL:**



```
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel pkg-config
```
- **On Arch Linux:**



```
sudo pacman -S base-devel openssl pkg-config
```

After installing these prerequisites, you can proceed with the cargo build --release command.

### Windows

To compile Rust projects on Windows, you need the Microsoft C++ (MSVC) build tools. The rustup installer for Windows will help you set this up.

1. **Run the rustup-init.exe installer.** If you haven't installed Rust yet, download it from the official [Rust website](https://www.google.com/url?sa=E&q=https%3A%2F%2Fwww.rust-lang.org%2Ftools%2Finstall).
2. During the installation, when prompted, choose to install the **MSVC build tools**. rustup will guide you through the Visual Studio Installer process.
3. If you already have Rust installed but are missing the build tools, you can install them separately. Open the **Visual Studio Installer** (which can be downloaded from the [Visual Studio website](https://www.google.com/url?sa=E&q=https%3A%2F%2Fvisualstudio.microsoft.com%2Fvisual-cpp-build-tools%2F)) and make sure the "Desktop development with C++" workload is selected and installed.

Once the build tools are in place, you can compile the project from the Command Prompt or PowerShell:



<br>

<br>

```
cargo build --release
```

### macOS

On macOS, you will need the **Xcode Command Line Tools**.

1. **Open the Terminal** (you can find it in /Applications/Utilities/).
2. Run the following command to install the tools:



```
xcode-select --install
```
3. A dialog box will appear asking you to install the tools. Click "Install" and agree to the terms and conditions.

With the Xcode Command Line Tools installed, you can now compile the project:



<br>

<br>

```
cargo build --release
```

## 4\. Running the Application

After a successful compilation, you can run the application using the following commands:

- **On Linux and macOS:**



```
./target/release/<executable_name>
```

_(Replace <executable\_name> with the actual name of the executable file, which is typically the name of the project)._
- **On Windows:**



```
.\target\release\<executable_name>.exe
```

Alternatively, you can use cargo to compile and run the application in one step:



<br>

<br>

```
cargo run --release
```

<br>



## Default Keybindings

The following table lists the default keybindings for various actions within the application. These can be changed using the keybindings command.

Note: It supports mouse too. Left Click Draw, Right Click Erase.

|     |     |     |
| --- | --- | --- |
| Action | Default Key | Description |
| **Canvas Navigation** | <br> | <br> |
| MoveCursorUp | Up Arrow | Move the drawing cursor up. |
| MoveCursorDown | Down Arrow | Move the drawing cursor down. |
| MoveCursorLeft | Left Arrow | Move the drawing cursor left. |
| MoveCursorRight | Right Arrow | Move the drawing cursor right. |
| PanViewUp | k   | Pan the canvas view up. |
| PanViewDown | j   | Pan the canvas view down. |
| PanViewLeft | h   | Pan the canvas view left. |
| PanViewRight | l   | Pan the canvas view right. |
| ZoomIn | \=  | Zoom into the canvas. |
| ZoomOut | \-  | Zoom out of the canvas. |
| **Drawing & Tools** | <br> | <br> |
| Draw | Space | Apply the current color or tool. |
| Erase | e   | Erase pixels under the cursor. |
| Fill | f   | Fill an area with the selected color. |
| PickColor | r   | Pick a color from the canvas. |
| IncreasePenSize | \]  | Increase the brush/pen size. |
| DecreasePenSize | \[  | Decrease the brush/pen size. |
| IncreaseOpacity | p   | Increase the current opacity. |
| DecreaseOpacity | o   | Decrease the current opacity. |
| Spray | _Unbound_ | Apply the spray tool (requires binding). |
| CycleSymmetry | s   | Cycle through symmetry modes (Off, Vertical, Diagonal, etc.). |
| AdjustSymmetryPositive | n   | Adjust the symmetry line. |
| AdjustSymmetryNegative | m   | Adjust the symmetry line. |
| **History** | <br> | <br> |
| Undo | Ctrl + z | Undo the last action. |
| Redo | Ctrl + y | Redo the last undone action. |
| **UI & Palettes** | <br> | <br> |
| OpenCommandPrompt | Esc | Open the command prompt. |
| OpenColorPicker | c   | Enter color selection mode. |
| OpenToolPicker | t   | Enter tool selection mode. |
| QuickSelectColorUp | Ctrl + Up | Navigate the color palette up. |
| QuickSelectColorDown | Ctrl + Down | Navigate the color palette down. |
| QuickSelectColorLeft | Ctrl + Left | Navigate the color palette left. |
| QuickSelectColorRight | Ctrl + Right | Navigate the color palette right. |
| QuickSelectToolLeft | Shift + Left | Navigate the tool palette left. |
| QuickSelectToolRight | Shift + Right | Navigate the tool palette right. |
| **Layer Management** | <br> | <br> |
| SelectLayerUp | Alt + Up | Select the layer above. |
| SelectLayerDown | Alt + Down | Select the layer below. |
| MoveLayerUp | Alt + k | Move the active layer up. |
| MoveLayerDown | Alt + j | Move the active layer down. |
| AddLayer | Alt + a | Add a new layer. |
| DeleteLayer | Alt + d | Delete the active layer. |
| ToggleLayerVisibility | Alt + v | Toggle visibility of the active layer. |
| ToggleOnionSkin | i   | Toggle onion skinning to see the layer below. |
| IncreaseOnionOpacity | u   | Increase onion skin opacity. |
| DecreaseOnionOpacity | y   | Decrease onion skin opacity. |
| **Application** | <br> | <br> |
| Quit | _Unbound_ | Quit the application (use quit command). |

* * *

## Command Prompt

Press Esc to open the command prompt at the bottom of the screen. Here you can type commands to perform actions or change settings.

- **Syntax:** command\_name or setting=value
- **Saving Settings:** To make a configuration change permanent, add --save at the end of the command.
    - _Example:_ penShape=square --save

* * *

## Commands Reference

### General Commands

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| help | Displays the keybindings cheatsheet. | help | help |
| quit / q | Quits the application. | quit | quit |
| undo | Undo the last action. | undo | undo |
| redo | Redo the last undone action. | redo | redo |
| clear | Clears the entire canvas on the active layer. | clear | clear |
| resize | Begin the interactive process for resizing the canvas. | resize | resize |
| keybindings | Opens the keybinding configuration panel. | keybindings | keybindings |
| keybindings:reset | Resets all keybindings to their default values. | keybindings:reset | keybindings:reset |
| config | Opens the configuration editor panel. | config | config |

### File & Project Commands

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| save | Saves the project. | save <name.consolet> \[-a mins\] \[-p path\] \[-f\] | save art.consolet -a 5 |
| load | Loads a project. | load <name.consolet> | load art.consolet |
| export | Exports canvas to a PNG image. | export \[-o path\] \[-u scale\] \[-bg\] | export -o image.png -u 10 |
| import | Imports a palette file for later use. | import palette <path> | import palette my\_palette.consolet |
| colorpalette:<name> | Switches to a loaded color palette. | colorpalette:<name> | colorpalette:default |
| savepalette:<name> | Saves the current set of colors as a new palette. | savepalette:<name> | savepalette:my-palette |
| colorpalette\_image | Generate a new palette from an image file. | colorpalette\_image \[--add\] | colorpalette\_image |

### Layer Commands

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| add\_layer | Add a new layer on top of the stack. | add\_layer | add\_layer |
| delete\_layer | Delete the currently active layer. | delete\_layer | delete\_layer |
| merge\_down | Merge the active layer with the layer below it. | merge\_down | merge\_down |
| rename\_layer | Rename the active layer. | rename\_layer=<new\_name> | rename\_layer=Background |
| layer\_opacity | Set the opacity of the active layer. | layer\_opacity=<0.0-1.0> | layer\_opacity=0.5 |
| onion\_skin | Toggle onion skinning (shows the layer below). | onion\_skin={true\|false} | onion\_skin=true |
| onion\_opacity | Set the opacity of the onion skinning effect. | onion\_opacity=<0.0-1.0> | onion\_opacity=0.3 |
| export\_mode | Set export to save all layers as one PNG or separately. | export\_mode={united\|separate} | export\_mode=separate |

### Drawing & Canvas Settings

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| minimap | Toggles the minimap display. | minimap={true\|false} | minimap=true |
| highlighter | Toggles the cursor highlighter. | highlighter={true\|false} | highlighter=false |
| protectStroke | Prevents drawing over the same pixel in one stroke. | protectStroke={true\|false} | protectStroke=false |
| mouseEvents | Enables or disables all mouse event handling. | mouseEvents={true\|false} | mouseEvents=false |
| penShape | Sets the brush shape. | penShape={circular\|square} | penShape=square |
| canvasScrollAction | Sets mouse wheel action on the canvas. | canvasScrollAction={ChangePenSize\|ChangeOpacity} | canvasScrollAction=ChangeOpacity |
| colorMode | Sets color mode for rendering. | colorMode={TrueColor\|Ansi256} | colorMode=Ansi256 |

### Tool Configuration

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| penSizeSensitivity | Sets pen size change sensitivity. | penSizeSensitivity={1-20} | penSizeSensitivity=2 |
| opacitySensitivity | Sets opacity change sensitivity. | opacitySensitivity={0.01-0.5} | opacitySensitivity=0.1 |
| highlighterMode | Sets highlighter mode. | highlighterMode={0\|1} | highlighterMode=1 |
| highlighterValue | Sets highlighter strength/blend amount. | highlighterValue={0.0-1.0} | highlighterValue=0.5 |
| pencilDensity | Sets the density for the Lighter/Darker tool. | pencilDensity={0.01-1.0} | pencilDensity=0.05 |
| applyColorSec | Sets auto-apply interval for holding Spacebar. | applyColorSec={0.05-2.0} | applyColorSec=0.1 |
| spraySize | Sets the size of the spray tool area. | spraySize={1-50} | spraySize=10 |
| spraySpeed | Sets the density/speed of the spray tool. | spraySpeed={1-100} | spraySpeed=5 |
| sprayIntensity | Sets the intensity/density of the spray tool. | sprayIntensity={0.01-1.0} | sprayIntensity=0.5 |

### Scripting Commands

|     |     |     |     |
| --- | --- | --- | --- |
| Command | Description | Usage | Example |
| edit\_script | Opens the command drawing script editor. | edit\_script | edit\_script |
| draw\_script | Executes the command drawing script. | draw\_script | draw\_script |

<br>
