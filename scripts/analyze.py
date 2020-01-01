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
        if config[1:] == (False,False):
            fullname += "only grid"
        elif config[1:] == (False,True):
            fullname += "map hyperparameters"
        elif config[1:] == (True,False):
            fullname += "map features"
        elif config[1:] == (True,True):
            fullname += "map both"
    
    
    algo_elitism = algo["elitism"]
    
    if algo_elitism == "Metropolis-Hastings":
        fullname += " - MH"
    elif algo_elitism == "Greedy_selection":
        fullname += " - greedy"
    
    return (algo_elitism,
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

axis_names = {
    "max score": "max population score",
    "variance": "score diversity (variance)",
    "mean genetic distance": "genetic diversity  of population"
}

results_dir = "analysis_results"

if not os.path.isdir(results_dir):
    print("creating result folder \"{}\"...".format(results_dir))
    os.mkdir(results_dir)
else:
    print("results folder \"{}\" already created, skipping...")


def iteration_plot(aggregated, elitism, y_axe="max score"):
    
    plt.figure(figsize=(11,7))
    
    aggregated = aggregated.loc[elitism]
    
    for k,data in aggregated.groupby("algorithm index"):
        
        #plt.scatter(df["intgen"], df["max score"])
    
        iterations = data.loc[k][y_axe]
        
        iterations = iterations[iterations.index % 1000 == 0]
        
        elitism, props, fullname = algo_index_to_properties(k,js)
        color = colors[props]
        symb = symbols[elitism]
                        
        plt.errorbar(iterations.index,
                    iterations["mean"],
                    yerr=iterations["std"],
                    errorevery=1, c=color,label=fullname,
                    marker=symb,
                    alpha=0.7)
    plt.xlabel("number of fitness calls (iterations)")
    
    y_axis_name = axis_names[y_axe]
    
    plt.ylabel(y_axis_name)
    
    plt.title(problem_name)
    plt.legend()
    
    filename = results_dir + "/"
    filename += "{}_iterations_{}_{}.png".format(problem_name,
                                                elitism, y_axe)
    
    plt.subplots_adjust(left=0.09, bottom=0.08, right=.98, top=.95, wspace=None, hspace=None)
    #plt.subplots_adjust(wspace=0.01, hspace=0.01)
    
    #plt.savefig(filename, bbox_inches="tight", pad_inches=0.1)
    #plt.savefig(filename, pad_inches=0)
    plt.savefig(filename, pad_inches=0)
    print("save \"{}\"".format(filename))
    plt.close(plt.gcf())
    #plt.show()

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
        
        chunk["elitism"] = chunk["algorithm index"].apply(
            lambda x: algo_index_to_properties(x,js)[0])
        
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
    df.set_index(["elitism","algorithm index"], inplace=True)
    
    
    aggregated = df.groupby(["elitism",
                        "algorithm index",
                        "iteration"]).agg(["mean", "std"])
    
    iteration_plot(aggregated,"Metropolis-Hastings", "max score")
    iteration_plot(aggregated,"Greedy_selection", "max score")

    iteration_plot(aggregated,"Metropolis-Hastings", "variance")
    iteration_plot(aggregated,"Greedy_selection", "variance")
    
    iteration_plot(aggregated,"Metropolis-Hastings", "mean genetic distance")
    iteration_plot(aggregated,"Greedy_selection", "mean genetic distance")























