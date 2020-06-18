import csv
from matplotlib import pyplot as plt
import matplotlib.ticker as ticker
import sys
import math

fname = sys.argv[1]
data = []
with open(fname) as f:
    reader = csv.reader(f)
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

ys_norm = []
for y in ys:
    ynorm = [1000*y/(n*math.log(n)) for n, y in zip(x,y)]
    ys_norm.append(ynorm)

y_div12 = [y1/y2 for y1, y2 in zip(ys[0], ys[1])]

fig1 = plt.figure(1)
for y in ys: 
    plt.plot(x,y)
plt.legend(fftnames)
plt.xlabel("Length")
plt.ylabel("Time, us")

fig2 = plt.figure(2)
for y in ys_norm: 
    plt.plot(x,y)
plt.legend(fftnames)
plt.xlabel("Length")
plt.ylabel("Time/NlogN")

fig3 = plt.figure(3) 
plt.plot(x,y_div12)
plt.grid(True)
fig3.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig3.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[0], fftnames[1]))


plt.show()
#print(data)
