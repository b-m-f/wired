The statefile corresponds is a fully defined config file.

Values that were unspecified will be saved with default values that represent absence.

These are as follows:

#### Server
- persistentkeepalive: -1
- fwmark: non_existing

#### Client
- listenport = -1

On subsequent runs, the program will always check for a statefile before
regenerating any keys.

