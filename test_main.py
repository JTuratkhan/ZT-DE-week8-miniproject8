import main
import pandas as pd
def test_calculate_statistics():
    # Creating a test CSV file
    data = {'Column1': [1, 2, 3, 4, 5], 'Column2': [5, 4, 3, 2, 1]}
    df = pd.DataFrame(data)
    df.to_csv('test_data.csv', index=False)
    # Calling the function with the test CSV file
    result = main.calculate_statistics('test_data.csv')
    # Checking the results
    assert result['mean']['Column1'] == 3.0
    assert result['median']['Column1'] == 3.0
    assert result['std']['Column1'] == 1.5811388300841898
    # Cleaning up
    import os
    os.remove('test_data.csv')
test_calculate_statistics()
