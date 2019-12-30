import os
import sys
from pprint import pprint
import numpy as np
import pandas as pd
import json
import matplotlib as mat
import matplotlib.pyplot as plt


def get_problem_name(filename):
    filename = filename.split("/")[-1].lower()
    if "rastrigin" in filename:
        return "custom Rastrigin function"
    elif "onemax" in filename:
        return "One Max"
    elif "tsp" in filename:
        return "2D Travelling Salesman Problem"
    else:
        return "unknown"

def optimize_table(df):
    df.drop("sum score", 1, inplace=True)
    df.drop("duration (ns)", 1, inplace=True)
    df.drop("median score", 1, inplace=True)
    df["algorithm index"] = df["algorithm index"].astype(np.uint8)
    df["repetition"] = df["repetition"].astype(np.uint8)
    df["iteration"] = df["iteration"].astype(np.uint32)
    df["number of organisms"] = df["number of organisms"].astype(np.uint16)
    df["generations"] = df["generations"].astype(np.float32)
    df["min score"] = df["min score"].astype(np.float32)
    df["max score"] = df["max score"].astype(np.float32)
    df["mean score"] = df["mean score"].astype(np.float32)
    
    
cols = ['repetition', 'algorithm index', 'iteration', 'duration (ns)',
       'sum score', 'min score', 'max score', 'mean score', 'median score',
       'number of organisms', 'variance', 'generations',
       'mean genetic distance']


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


for path in paths:
    print("reading",path)
    
    js = None
    with open(path) as f:
        js = f.readline()
        js = json.loads(js[1:-2])
        pprint(js)
    
    df = pd.DataFrame()
    l = []
    reader = pd.read_table(path, sep=",", chunksize=100*1024, skiprows=1)
    for chunk in reader:
        chunk["intgen"] = chunk["generations"].apply(np.int32)
        optimize_table(chunk)
        #test = chunk[chunk["iteration"] % 50 == 0]
        #print(test["intgen"])
        
        l.append(chunk)
    
    df = pd.concat(l)
    del l
    plt.scatter(df["intgen"], df["max score"])
        
    plt.show()
