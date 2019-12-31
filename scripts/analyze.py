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

def run_groups(df):
    for i in df:
        print(i)
        break

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


def algo_index_to_properties(index, js):
    algo = js["algorithms"][index]
    
    prop = algo["algorithm config"]
    config = (algo["algorithm name"],
            prop.get("use features", False),
            prop.get("use spatial hyperparameters", False))
    
    fullname = config[0]
    
    if "generalized map elite" in fullname.lower():
        fullname = "GMAP Elite: "
        if config[:1] == [False,False]:
            fullname += "only grid"
        elif config[:1] == [False,True]:
            fullname += "map hyperparameters"
        elif config[:1] == [True,False]:
            fullname += "map features"
        elif config[:1] == [False,True]:
            fullname += "map both"
    
    return (algo["elitism"],
            config,
            fullname)

paths = sys.argv[1:]

# name, features, hyperparameters
colors = {
    ("SimpleReplacement",False,False): "grey",
    ("Generalized MAP Elite algorithm",False,False): "pink",
    ("Generalized MAP Elite algorithm",False,True): "turquoise",
    ("Generalized MAP Elite algorithm",True,False): "green",
    ("Generalized MAP Elite algorithm",True,True): "indigo",
    ("MAP Elite",True,False): "gold",
    ("Simple Adaptive GA",False,False): "darkred",
}

symbols = {
    "Greedy_selection": ".",
    "Metropolis-Hastings": "v"
}


for path in paths:
    print("reading",path)
    
    problem_name = get_problem_name(path)
    print("Analysing results for",problem_name)
    
    print("reading config...")
    js = None
    with open(path) as f:
        js = f.readline()
        js = json.loads(js[1:-2])
        pprint(js)
    
    print("reading chunks...")
    df = None
    l = []
    reader = pd.read_table(path, sep=",", chunksize=500*1024, skiprows=1)
    
    i = 0
    for chunk in reader:
        chunk["intgen"] = chunk["generations"].apply(np.int32)
        optimize_table(chunk)
        #chunk = chunk[chunk["iteration"] % 50 == 0]
        chunk = chunk[chunk["mean genetic distance"].notnull() == True]
        #print(chunk)
        #print(test["intgen"])
        
        l.append(chunk)
        i += 1
        if i > 100:
            break
    print("read {} chunks".format(i))
    
    df = pd.concat(l)
    del l
    
    print("setting indexes...")
    df.set_index(["algorithm index"], inplace=True)
    
    
    aggregated = df.groupby(["algorithm index","iteration"]).agg(["mean", "std"])
    

    
    for k,data in aggregated.groupby("algorithm index"):
        
        #plt.scatter(df["intgen"], df["max score"])
    
        iterations = data.loc[k]["max score"]
        
        iterations = iterations[iterations.index % 1000 == 0]
        
        elitism, props, fullname = algo_index_to_properties(k,js)
        color = colors[props]
        symb = symbols[elitism]
        
        full_algo_name = fullname + " - " + elitism
                
        plt.errorbar(iterations.index,
                    iterations["mean"],
                    yerr=iterations["std"],
                    errorevery=3, c=color,label=full_algo_name,
                    marker=symb,
                    alpha=0.7)
    plt.xlabel("number of fitness calls (iterations)")
    plt.ylabel("max population score")
    plt.title(problem_name)
    plt.legend()
    plt.show()























