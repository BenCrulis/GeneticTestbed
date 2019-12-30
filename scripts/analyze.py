import os
import sys
from pprint import pprint
import numpy as np
import pandas as pd
import json
import matplotlib as mat
import matplotlib.pyplot as plt


paths = sys.argv[1:]

# name, features, hyperparameters
colors = {
    ("SimpleReplacement",False,False): "grey",
    ("Generalized MAP Elite algorithm",False,False): "green",
    ("Generalized MAP Elite algorithm",False,True): "green",
    ("Generalized MAP Elite algorithm",True,False): "green",
    ("Generalized MAP Elite algorithm",True,True): "green",
    ("MAP Elite",False,False): "green",
    ("Simple Adaptive GA",False,False): "green",
}

symb = {
    "Greedy_selection": "+",
    "Metropolis-Hastings": "o"
}

cols = ['repetition', 'algorithm index', 'iteration', 'duration (ns)',
       'sum score', 'min score', 'max score', 'mean score', 'median score',
       'number of organisms', 'variance', 'generations',
       'mean genetic distance']

for path in paths:
    print("reading",path)
    
    js = None
    with open(path) as f:
        js = f.readline()
        js = json.loads(js[1:-2])
        pprint(js)
    
    df = pd.DataFrame()
    reader = pd.read_table(path, sep=",", chunksize=16*1024, skiprows=1)
    for chunk in reader:
        chunk["intgen"] = chunk["generations"].apply(np.int32)
        test = chunk[chunk["iteration"] % 50 == 0]
        print(test["intgen"])
        
        plt.scatter(chunk["intgen"], chunk["max score"])
        
    plt.show()
