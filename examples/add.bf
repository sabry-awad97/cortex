,       # Read first number into cell 0
>       # Move to cell 1
,       # Read second number into cell 1
[       # Start loop
    <   # Move to cell 0
    +   # Increment cell 0
    >   # Move to cell 1
    -   # Decrement cell 1
]       # Repeat until cell 1 is 0
<       # Move back to cell 0
.       # Output result