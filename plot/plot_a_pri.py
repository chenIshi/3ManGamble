from matplotlib import pyplot as plt
import numpy as np
import math
from scipy.optimize import curve_fit
from scipy import asarray as ar,exp
from sklearn import linear_model

file_path = '../out/a_out.txt'

with open(file_path) as fp:
	line = fp.readline()
	result = eval(line)
	value_s = [ i for i in result.keys()]
	value = map(int, value_s)
	num_s = [i for i in result.values()]
	nums = map(int, num_s)

value = np.array(value).reshape(-1, 1)
nums = np.array(nums).reshape(-1, 1)

value_new = (value - 45) ** 2
nums_new = np.log(nums)

lr = linear_model.LinearRegression(fit_intercept=True)
lr.fit(value_new, nums_new)
print(value,nums)

y_predict = np.exp(lr.predict(value_new))
plt.scatter(value, nums, c='r')
plt.scatter(value, y_predict, c='b')
plt.legend()
plt.title("Steps taken to end the game")
plt.show()