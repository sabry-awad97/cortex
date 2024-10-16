# This Brainfuck program calculates and prints the Fibonacci sequence

# Initialize the first two numbers of the sequence
++++++++++ # Set cell 0 to 10 (loop counter)
>+ # Set cell 1 to 1
>+ # Set cell 2 to 1
<<

# Main loop to calculate Fibonacci numbers
[
    # Move to cell 2 and 3, add their values, store in cell 4
    >>
    [<<+>>-]         # Move value from cell 3 to cell 1
    <[>+<-]          # Move value from cell 2 to cell 3
    >>[<<+>>-]       # Move sum from cell 4 to cell 2
    
    # Print the calculated Fibonacci number
    <<
    [>+<-]           # Move value to cell 3 for printing
    >.               # Print the number (as ASCII)
    <                # Move back to cell 2
    
    # Print a space
    >+++++++++.[-]   # Print space and clear cell
    
    # Decrement the counter
    <<<-
]

# Print a newline at the end
>>++++++++++++.      # Print newline character
