from matplotlib import pyplot as plt
import numpy as np
import math
from scipy.optimize import curve_fit
from scipy import asarray as ar,exp

file_path = '../out/nsteps.txt'

with open(file_path) as fp:
	line = fp.readline()
	result = eval(line)
	time_stamp_s = [ i for i in result.keys()]
	time_stamp = map(int, time_stamp_s)
	nstamps_s = [i for i in result.values()]
	nstamps = map(int, nstamps_s)

	plt.scatter(time_stamp, nstamps, c='b')
	plt.legend()
	plt.title("Steps taken to end the game")
	plt.show()