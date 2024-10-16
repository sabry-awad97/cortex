# This Brainfuck program prints "Hello World!"

# Set up the initial values in the memory cells
++++++++    # Cell 0: Set to 8
[           # Start outer loop
    >++++   # Cell 1: Add 4
    [       # Start inner loop
        >++     # Cell 2: Add 2
        >+++    # Cell 3: Add 3
        >+++    # Cell 4: Add 3
        >+      # Cell 5: Add 1
        <<<<-   # Move back to Cell 1 and decrement
    ]       # End inner loop (repeat 4 times)
    >+      # Cell 2: Add 1
    >+      # Cell 3: Add 1
    >-      # Cell 4: Subtract 1
    >>+     # Cell 6: Add 1
    [<]     # Move back to first 0 cell
    <-      # Decrement Cell 1
]           # End outer loop (repeat 8 times)

# At this point, we have set up the memory cells with the ASCII values we need

# Output "Hello World!"
>>.         # Move to Cell 2 and output 'H'
>---.       # Move to Cell 3 and output 'e'
+++++++..   # Add 7 to Cell 3 and output 'll'
+++.        # Add 3 to Cell 3 and output 'o'
>>.         # Move to Cell 5 and output ' '
<-.         # Move to Cell 4 and output 'W'
<.          # Move to Cell 3 and output 'o'
+++.        # Add 3 to Cell 3 and output 'r'
------.     # Subtract 6 from Cell 3 and output 'l'
--------.   # Subtract 8 from Cell 3 and output 'd'
>>+.        # Move to Cell 5 and output '!'
>++.        # Move to Cell 6 and output a newline