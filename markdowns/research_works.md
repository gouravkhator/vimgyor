# Research Done or To Be Done

## Research to be Done

- Research if `execute!` macro is better than `queue!` macro or not, in terms of operations time taken.
  - `execute!` macro does flushing out the commands output one at a time..
  - `queue!` macro only adds the commands output to the stdout, and only flushes when `Enter` key is pressed, or when we do `stdout().flush()` ourselves programmatically.

## Research Done

- Linux Color Codes is explained in detail [here](https://unix.stackexchange.com/a/124409), as well as noted below:

  - xterm 256 color codes are supported in most of terminals and the chart is as mentioned [here](./linux_color_codes_chart.png).
  - The ANSI sequence to select these, using the number in the bottom left corner in the chart, starts 38;5; for the foreground and 48;5; for the background, then the color number.
  - Example:

    ```sh
    # Note: `95` is the code for tan color, and `214` is the code for light orange
    echo -e "\\033[48;5;95;38;5;214mhello world\\033[0m"
    ```

    Above command gives an output of "hello world", in light-orange foreground on a tan background.
