"""
Test goes here

"""

from main import data_filter
import pandas as pd


def test_data_filter():
    mock_csv_file = pd.DataFrame(
        {
            "Name": ["Alice", "Bob", "Charlie", "David", "Eve", "Frank"],
            ' "Test3"': [83, 82, 81, 80, 79, 78],
        }
    )
    res = data_filter(mock_csv_file)
    assert len(res) == 4


if __name__ == "__main__":
    test_data_filter()
    print("All tests passed.")
