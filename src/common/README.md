# Common
This directory contains code for common facilities which will be required by the GUI and TUI.
The main focus here currently is to:
> `Scan a directory recursively and print metadata for each file`

**Current plan:**
Currently the plan is to develop a way to scan and filter the files and store their data
inside a sqlite instance. This way I do not have to implement the scanning and filtering
logic for both GUI and TUI separately.
