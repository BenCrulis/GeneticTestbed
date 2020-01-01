import os
import sys
from pprint import pprint
import numpy as np
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 unused import
from matplotlib import cm
import matplotlib as mat
import matplotlib.pyplot as plt


A = 10

B = 20.0

def custom_rastrigin(x,y):
    
    s = x*x - A*np.cos(2.0*np.pi*x+np.pi) + y*y - A*np.cos(2.0*np.pi*y+np.pi)
    
    s += 2*A
    
    reg = np.exp((-x*x - y*y)/B)
    #reg *= 10.0
    
    return reg*s/2.0

custom_rastrigin_config = (custom_rastrigin, 40.0, "Modified Rastrigin PDF")


def rastrigin(x,y):
    return A*2 + x*x-A*np.cos(2.0*np.pi*x) + y*y-A*np.cos(2.0*np.pi*y)

rastrigin_config = (rastrigin, 100.0, "Rastrigin")


configs = [custom_rastrigin_config, rastrigin_config]


for function, y_max, title in configs:


    fig = plt.figure()

    ax = fig.add_subplot(1, 1, 1, projection='3d')
    X = np.arange(-5, 5, 0.05)
    Y = np.arange(-5, 5, 0.05)
    X, Y = np.meshgrid(X, Y)

    Z = function(X,Y)

    surf = ax.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap=cm.viridis,
                           linewidth=0, antialiased=False)
    ax.set_zlim3d(0, y_max)
    
    ax.set_title(title)

    #ax.w_zaxis.set_major_locator(LinearLocator(10))
    #ax.w_zaxis.set_major_formatter(FormatStrFormatter('%.03f'))

    fig.colorbar(surf, shrink=0.5, aspect=5)

    plt.show()
