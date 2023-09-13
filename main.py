import pandas as pd
import matplotlib.pyplot as plt

data = pd.read_csv('HRDataset_v14.csv')

def calculate_statistics(file_path):
    try:
        # Reading the dataset from the CSV file
        data = pd.read_csv(file_path)
        for col in data.columns:
            data[col] = pd.to_numeric(data[col], errors='coerce')
        # Calculating mean, median and standard deviation
        mean = data.mean()
        median = data.median()
        std = data.std()
        return {'mean': mean, 'median': median, 'std': std}
    except Exception as e:
        return str(e)
      

def visualize_data(data):
    try:
        # Check if the input is a DataFrame
        if not isinstance(data, pd.DataFrame):
            raise ValueError("Input is not a pandas DataFrame")

        # Iterate over each numeric column and create a line plot
        for col in data.select_dtypes(include=['number']).columns:
            plt.figure(figsize=(10, 6))  # Set the figure size
            plt.plot(data.index, data[col], marker='o', linestyle='-', label=col)
            plt.title(f"Line Plot for {col}")
            plt.xlabel("Index")
            plt.ylabel(col)
            plt.legend()
            plt.grid(True)
            plt.show()
    
    except Exception as e:
        return str(e)

if __name__ == "__main__":
    result = calculate_statistics('HRDataset_v14.csv')
    print(result)

# calculate_statistics(data)
# visualize_data(data)
