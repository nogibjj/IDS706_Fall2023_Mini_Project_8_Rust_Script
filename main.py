import pandas as pd
import matplotlib.pyplot as plt


def data_filter(df):
    return df[df[' "Test3"'] >= 80]


if __name__ == "__main__":
    dataframe = pd.read_csv("./grades.csv")
    print(data_filter(dataframe))
    plt.scatter(dataframe["Last name"], dataframe[' "Test3"'])
    plt.savefig("data_visualization.png")

    # calculate statistics
    series = dataframe[' "Test3"']
    print("Mean: ", series.mean())
    print("Median: ", series.median())
    print("Standard Deviation: ", series.std())
