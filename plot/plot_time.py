from matplotlib import pyplot as plt
import numpy as np
import math
from scipy.optimize import curve_fit
from scipy import asarray as ar,exp
from sklearn import linear_model

file_path = '../out/nsteps.txt'

with open(file_path) as fp:
	line = fp.readline()
	result = eval(line)
	time_stamp_s = [ i for i in result.keys()]
	time_stamp = map(int, time_stamp_s)
	nstamps_s = [i for i in result.values()]
	nstamps = map(int, nstamps_s)

time_stamp = np.array(time_stamp).reshape(-1, 1)
nstamps = np.array(nstamps).reshape(-1, 1)

lr = linear_model.LinearRegression(fit_intercept=True)
lr.fit(time_stamp, nstamps)

plt.scatter(time_stamp, nstamps, c='b')
# plt.scatter(time_stamp, lr.predict(time_stamp), c='r')
plt.legend()
plt.title("Steps taken to end the game")
plt.show()