from matplotlib import pyplot as plt
import numpy as np
import math
from scipy.optimize import curve_fit
from scipy import asarray as ar,exp
from sklearn import linear_model

file_path = '../out/b_out.txt'

with open(file_path) as fp:
	line = fp.readline()
	result = eval(line)
	value_s = [ i for i in result.keys()]
	value = map(int, value_s)
	num_s = [i for i in result.values()]
	nums = map(int, num_s)

#	for val in value:
#		val = (val - 45) ** 2
for i in range(len(value)):
	value[i] = (value[i] - 45) ** 2

#	for n in nums:
#		n = math.log(n)
for i in range(len(nums)):
	nums[i] = math.log(nums[i])

value = np.array(value).reshape(-1, 1)
nums = np.array(nums).reshape(-1, 1)

lr = linear_model.LinearRegression(fit_intercept=True)
lr.fit(value, nums)

plt.figure(figsize=(11, 8))
plt.scatter(value, nums, c='b', label='Money C owned')
plt.plot(value, lr.predict(value), c='r', label='Prediction with Linear Regression')
plt.legend()
plt.title("Distrubution of C's money when B bankrupt")
plt.xlabel('(x - 45) ** 2')
plt.ylabel('log(y)')

plt.show()