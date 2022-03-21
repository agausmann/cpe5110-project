def carry_select_delay(groups): 
    groups = list(reversed(groups))

    # Initial delay from ripple carry.
    delay = 2 * (groups[0] + 1)

    # Add delays for each select module:
    for n in groups[1:]:
        # Make sure its own adder is ready.
        delay = max(delay, 2 * (n + 1))
        # Add delay for multiplexer.
        delay += 2

    return delay
