## Dependencies

 - libssl-dev
 - [mySql](https://linuxhint.com/installing_mysql_workbench_ubuntu/)


## usage

To get the current scores

- `-t <TABLE>` REQUIRED selects the challenge
- `-p` REQUIRED Prints the score board (use `-n` to limit the lines printed)
- `-n` OPTIONAL Number of entries to print (if not provided the entire table is returned)
- `-s <COLUMN>` OPTIONAL Sort by the given column

To add a score

- `-t <TABLE>` REQUIRED selects the challenge you're adding a result for
- `-u <NAME>` REQUIRED The name you want displayed on the scoreboard
- `-c <COMMAND>` REQUIRED The command that was run
- `-r <TIME in NS>` REQUIRED The result as an INT given in nanoseconds

### Debug Options
- `-v <LEVEL>` OPTIONAL Defaults to `info`
    - error
    - info
    - warn
    - debug
    - trace
- `-o <OUTPUT>` OPTIONAL Defaults to `syslog`
    - syslog
    - stdout