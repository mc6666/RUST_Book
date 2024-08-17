import matplotlib.pyplot as plt
from sklearn import datasets

iris = datasets.load_iris()
plt.scatter(iris.data[:, 0], iris.data[:, 1], c=iris.target)
plt.show()
