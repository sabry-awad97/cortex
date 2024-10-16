++++++++++ # Input the number of terms (10 for example)
[
    >++++++++++
    >+
    >+
    <<<-
]
>[-]
>[-]
++++++++++ # Set initial values for 0 and 1
>+


# Main loop for Fibonacci
[
    # Output current number
    >>.
    
    # Calculate next number
    >[-<<+>>] # Move second number to first position
    <<[->>>+<<<] # Move first number to second position
    >>>[-<<<+>>>] # Add numbers
    
    # Move back to counter
    <<<-
]