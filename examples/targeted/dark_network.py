"""
Dark Network Reconstruction Example.
This example demonstrates how to infer the activity of a hidden leader 
in a covert network based on observed meetings and communication signals.
"""

import lutufi
from lutufi.inference import InferenceEngine

def run_intelligence_example():
    # 1. Build the network
    # Nodes:
    #   Leader: Hidden node (Active/Inactive)
    #   Meeting_A_B: Observed meeting between members A and B (Yes/No)
    #   Meeting_C_D: Observed meeting between members C and D (Yes/No)
    #   EncryptedSignal: Detected signal in the area (Detected/None)
    
    builder = (lutufi.BayesianNetwork.builder()
        .add_variable("Leader", domain=["Active", "Inactive"])
        .add_variable("Meeting_A_B", domain=["Yes", "No"])
        .add_variable("Meeting_C_D", domain=["Yes", "No"])
        .add_variable("EncryptedSignal", domain=["Detected", "None"])
        
        # Leader's activity increases probability of meetings and signals
        .add_edge("Leader", "Meeting_A_B")
        .add_edge("Leader", "Meeting_C_D")
        .add_edge("Leader", "EncryptedSignal"))

    # 2. Set CPTs
    # Prior: probability leader is active is low
    builder.set_cpd("Leader", [0.1, 0.9])

    # Meetings are more likely if leader is active (coordinating)
    # Row 0: Yes, Row 1: No
    # Columns: Leader=Active, Leader=Inactive
    builder.set_cpd("Meeting_A_B", [
        [0.7, 0.05], # Yes
        [0.3, 0.95]  # No
    ])

    builder.set_cpd("Meeting_C_D", [
        [0.6, 0.1], # Yes
        [0.4, 0.9]  # No
    ])

    # Signal probability
    # Row 0: Detected, Row 1: None
    builder.set_cpd("EncryptedSignal", [
        [0.8, 0.01], # Detected
        [0.2, 0.99]  # None
    ])

    network = builder.build()
    engine = InferenceEngine(network)

    # 3. Run Queries
    print("Scenario 1: Posterior probability of Leader activity given no evidence")
    res1 = engine.query(["Leader"])
    print(res1.to_dict()["distributions"]["Leader"])

    print("\nScenario 2: Prob of Leader activity given one meeting (A and B)")
    res2 = engine.query(["Leader"], evidence={"Meeting_A_B": "Yes"})
    print(res2.to_dict()["distributions"]["Leader"])

    print("\nScenario 3: Prob of Leader activity given meetings and encrypted signal (Strong Evidence)")
    res3 = engine.query(["Leader"], evidence={
        "Meeting_A_B": "Yes",
        "Meeting_C_D": "Yes",
        "EncryptedSignal": "Detected"
    })
    print(res3.to_dict()["distributions"]["Leader"])

if __name__ == "__main__":
    run_intelligence_example()
