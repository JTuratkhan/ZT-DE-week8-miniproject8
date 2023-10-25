# Introduction:
In this mini-project, the primary aim was to rewrite an existing Python script responsible for data processing in Rust to investigate potential performance benefits. The Python script performed certain operations on a dataset, including statistical calculations and visualizations. A similar approach was then replicated in Rust.

# Methodology:
To compare the performance between the Python and Rust implementations, two key metrics were observed:

# Execution time: The duration taken by the script to complete its operations.
Memory Usage: The amount of system memory consumed by the script during its execution.
## Results:
Execution Time:
Language	Time
Rust	0m0.022s
Python	0m0.892s
Note: The Rust version was significantly faster in terms of execution time compared to the Python version. The Rust script completed its execution in just 22 milliseconds, while the Python script took 892 milliseconds. This showcases an almost 40x speed improvement with the Rust implementation.

# Conclusion:
From the performance results, it's evident that Rust provides a substantial improvement in execution speed over the Python script for this specific data processing task. Such improvements can be critical in real-world applications where large datasets are processed, and time efficiency is paramount. However, it's also essential to consider other factors, such as development time, ease of maintenance, and library ecosystem, when choosing between languages for a specific task.

