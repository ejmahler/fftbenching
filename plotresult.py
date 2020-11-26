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

y_combo = [min((a,b,c)) for a, b, c in zip(ys[0], ys[2], ys[3])]

y_div12 = [y1/y2 for y1, y2 in zip(ys[0], ys[1])]

y_div32 = [y3/y2 for y3, y2 in zip(ys[2], ys[1])]

y_div42 = [y4/y2 for y4, y2 in zip(ys[3], ys[1])]

y_div31 = [y3/y1 for y3, y1 in zip(ys[2], ys[0])]

y_div41 = [y4/y1 for y4, y1 in zip(ys[3], ys[0])]

y_divcombo = [yc/yfftw for yc, yfftw in zip(y_combo, ys[1])]

fig1 = plt.figure(1, figsize=(10,7))
for y in ys: 
    plt.plot(x,y, linestyle='dashed', marker=".")
plt.legend(fftnames)
plt.xlabel("Length")
plt.ylabel("Time, us")

fig2 = plt.figure(2, figsize=(10,7))
for y in ys_norm: 
    plt.plot(x,y)
plt.legend(fftnames)
plt.xlabel("Length")
plt.ylabel("Time/NlogN")

fig3 = plt.figure(3, figsize=(10,7)) 
plt.plot(x,y_div12)
plt.grid(True)
fig3.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig3.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[0], fftnames[1]))

fig4 = plt.figure(4, figsize=(10,7)) 
plt.plot(x,y_div32)
plt.grid(True)
fig4.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig4.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[2], fftnames[1]))

fig5 = plt.figure(5, figsize=(10,7)) 
plt.plot(x,y_div42)
plt.grid(True)
fig5.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig5.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[3], fftnames[1]))

fig6 = plt.figure(6, figsize=(10,7)) 
plt.plot(x,y_div31)
plt.grid(True)
fig6.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig6.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[2], fftnames[0]))

fig7 = plt.figure(7, figsize=(10,7)) 
plt.plot(x,y_div41)
plt.grid(True)
fig7.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig7.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("{}/{}".format(fftnames[3], fftnames[0]))

fig8 = plt.figure(8, figsize=(10,7)) 
plt.plot(x,y_divcombo)
plt.grid(True)
fig8.gca().yaxis.set_major_locator(ticker.MultipleLocator(1))
fig8.gca().xaxis.set_major_locator(ticker.MultipleLocator(32))
plt.xlabel("Length")
plt.ylabel("Combo/FFTW")

plt.show()
#print(data)
