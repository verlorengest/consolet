pub fn get_default_help_text() -> &'static str {
    "--- CONSOLET: Command Reference ---\n\n\
    Press ESC to open the command prompt.\n\
    Use Arrow Keys or Mouse Wheel to scroll. Press ESC to return.\n\n\
    --- GENERAL COMMANDS ---\n\
    help              - Show this help screen.\n\
    quit / q          - Quit the application.\n\
    resize            - Begin resizing the canvas (clears canvas).\n\
    undo / redo       - Perform undo/redo actions.\n\
    keybindings       - Open the interactive keybinding editor.\n\
    config            - Open the interactive configuration editor.\n\n\
    --- FILE & PROJECT COMMANDS ---\n\
    save <name.consolet> - Save the project. Args: -a {mins}, -p \"path\", -f\n\
    \tExample: save my_art.consolet -a 5\n\n\
    load <name.consolet>  - Load a project. Searches default folder if no path is given.\n\n\
    export            - Export the canvas to a PNG. Args: -o \"path\", -u {scale}, -bg\n\
    \tExample: export -u 10 -o \"art.png\"\n\n\
    import palette <path> - Import a .consolet palette file for later use.\n\
    colorpalette:<name>   - Switch to a loaded palette (e.g., colorpalette:default).\n\
    colorpalette:<name>   - Switch to a loaded palette (e.g., colorpalette:default).\n\
    savepalette:<name>    - Save the current set of colors as a new palette.\n\
    colorpalette:image    - Generate a new palette from an image file.\n\
    #RRGGBB           - Enter a hex code to add it to the current palette.\n\n\
    --- SCRIPTING COMMANDS ---\n\
    edit_script       - Open the text editor for the command drawing script.\n\
    draw_script       - Executes the drawing commands in command_draw.json.\n\
    \tCommands: apply_color:#RRGGBB X,Y X2,Y2 X3,Y3-X4,Y4\n\
    \t          erase X,Y X2,Y2-X3,Y3\n\
    \t          fill:#RRGGBB X,Y\n\
    \tSymmetry Block Example:\n\
    \t{ \"symmetry\": { \"mode\": \"vertical\", \"coordinate\": 15 },\n\
    \t  \"commands\": [ \"apply_color:#00FF00 10,12\" ] }\n\n\
    --- CONFIGURATION ---\n\
    To change a setting, use 'setting=value'.\n\
    To make a change permanent across sessions, add '--save' at the end.\n\
    Example: penShape=square --save\n\n\
    For a full list of keybindings, use the 'keybindings' command."
}