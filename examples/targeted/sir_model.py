"""
Simple SIR (Susceptible-Infectious-Recovered) model as a Bayesian Network.
This example demonstrates how to build a network, set parameters, and run inference.
"""

import lutufi
from lutufi.inference import InferenceEngine

def run_example():
    # 1. Build the network
    # We model a single individual's state transitions over two time steps
    # Nodes:
    #   S0: Susceptible at t=0
    #   I0: Infectious at t=0
    #   S1: Susceptible at t=1
    #   I1: Infectious at t=1
    #   R1: Recovered at t=1

    builder = (lutufi.BayesianNetwork.builder()
        .add_variable("S0", domain=["True", "False"])
        .add_variable("I0", domain=["True", "False"])
        .add_variable("S1", domain=["True", "False"])
        .add_variable("I1", domain=["True", "False"])
        .add_variable("R1", domain=["True", "False"])
        .add_edge("S0", "S1")
        .add_edge("S0", "I1")
        .add_edge("I0", "I1")
        .add_edge("I0", "R1"))

    # 2. Set Conditional Probability Tables (CPTs) on the builder
    # P(S0=True) = 0.99
    builder.set_cpd("S0", [0.99, 0.01])
    # P(I0=True) = 0.01
    builder.set_cpd("I0", [0.01, 0.99])

    # P(S1 | S0)
    # Row 0: True, Row 1: False
    # Columns: S0=True, S0=False
    builder.set_cpd("S1", [
        [0.8, 0.0], # S1=True
        [0.2, 1.0]  # S1=False
    ])

    # P(I1 | S0, I0)
    # Parent configs: (S0=T, I0=T), (S0=T, I0=F), (S0=F, I0=T), (S0=F, I0=F)
    builder.set_cpd("I1", [
        [0.9, 0.05, 0.8, 0.0], # I1=True
        [0.1, 0.95, 0.2, 1.0]  # I1=False
    ])

    # P(R1 | I0)
    # Columns: I0=True, I0=False
    builder.set_cpd("R1", [
        [0.3, 0.0], # R1=True
        [0.7, 1.0]  # R1=False
    ])

    bn = builder.build()

    # 3. Run Inference
    engine = InferenceEngine(bn)

    # Query: Probability of being Infectious at t=1 given Susceptible at t=0
    print("Query 1: P(I1 | S0=True)")
    res1 = engine.query(["I1"], evidence={"S0": "True"})
    print(res1.to_dict()["distributions"]["I1"])

    # Query: Probability of being Recovered at t=1 given Infectious at t=0
    print("\nQuery 2: P(R1 | I0=True)")
    res2 = engine.query(["R1"], evidence={"I0": "True"})
    print(res2.to_dict()["distributions"]["R1"])

    # Most probable states
    print("\nMost probable states at t=1 given I0=True:")
    res3 = engine.query(["S1", "I1", "R1"], evidence={"I0": "True"})
    print(res3.most_probable())

if __name__ == "__main__":
    run_example()
