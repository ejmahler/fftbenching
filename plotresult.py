import csv
from matplotlib import pyplot as plt
import matplotlib.ticker as ticker
from matplotlib.ticker import (MultipleLocator, AutoMinorLocator)
import sys
import math
import codecs

fname = sys.argv[1]
data = []
result_file = codecs.open(fname, 'rU', 'utf-16')
reader = csv.reader(result_file)
headers = next(reader)
for header in headers:
    data.append([])
for line in reader:
    for n, val in enumerate(line):
        data[n].append(float(val))

#print(data)
x = data[0]
ys = data[1:]
fftnames = headers[1:]

fig1 = plt.figure(1, figsize=(10,7))
for y in ys: 
    plt.plot(x,y, linestyle='dashed', marker=".")
plt.legend(fftnames)
plt.xlabel("Inner FFT Size")
plt.ylabel("Multiplier")
plt.grid(True, 'both')
axes = fig1.gca()
axes.xaxis.set_major_locator(MultipleLocator(32768))
axes.yaxis.set_major_locator(MultipleLocator(1))
axes.yaxis.set_minor_locator(MultipleLocator(0.25))

plt.show()
