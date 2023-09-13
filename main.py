import pandas as pd
def calculate_statistics(file_path):
    try:
        # Reading the dataset from the CSV file
        data = pd.read_csv(file_path)
        # Calculating mean, median and standard deviation
        mean = data.mean()
        median = data.median()
        std = data.std()
        return {'mean': mean, 'median': median, 'std': std}
    except Exception as e:
        return str(e)
      
if __name__ == "__main__":
    result = calculate_statistics('path/to/your/csvfile.csv')
    print(result)
