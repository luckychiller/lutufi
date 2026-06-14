"""
Interbank Risk Contagion Example.
This example models systemic risk in a synthetic interbank lending network 
with hidden macroeconomic confounders.
"""

import lutufi
from lutufi.inference import InferenceEngine

def run_finance_example():
    # 1. Build the network
    # Nodes: 
    #   MarketShock: Hidden confounder (True/False)
    #   Bank_A...Bank_E: Operational status (Healthy/Default)
    #   SystemicCrisis: Overall network state (True/False)
    
    builder = (lutufi.BayesianNetwork.builder()
        .add_variable("MarketShock", domain=["True", "False"])
        .add_variable("Bank_A", domain=["Healthy", "Default"])
        .add_variable("Bank_B", domain=["Healthy", "Default"])
        .add_variable("Bank_C", domain=["Healthy", "Default"])
        .add_variable("Bank_D", domain=["Healthy", "Default"])
        .add_variable("Bank_E", domain=["Healthy", "Default"])
        .add_variable("SystemicCrisis", domain=["True", "False"])
        
        # Market shock affects all banks
        .add_edge("MarketShock", "Bank_A")
        .add_edge("MarketShock", "Bank_B")
        .add_edge("MarketShock", "Bank_C")
        .add_edge("MarketShock", "Bank_D")
        .add_edge("MarketShock", "Bank_E")
        
        # Lending ties (Contagion paths)
        .add_edge("Bank_A", "Bank_B") # B lent to A
        .add_edge("Bank_B", "Bank_C") # C lent to B
        .add_edge("Bank_C", "Bank_D") # D lent to C
        
        # Systemic crisis depends on multiple defaults
        .add_edge("Bank_D", "SystemicCrisis")
        .add_edge("Bank_E", "SystemicCrisis"))

    # 2. Set CPTs
    # P(MarketShock=True) = 0.05
    builder.set_cpd("MarketShock", [0.05, 0.95])

    # Banks depend on shock and lending ties
    # Simplified logic: if shock=True, high default prob. If borrower defaults, high default prob.
    
    # Healthy default rates
    p_healthy = 0.01
    p_shock = 0.3
    p_contagion = 0.5

    # Bank_A (only depends on MarketShock)
    # Row 0: Healthy, Row 1: Default
    builder.set_cpd("Bank_A", [
        [1.0 - p_shock, 1.0 - p_healthy], # Healthy given Shock=T, Shock=F
        [p_shock, p_healthy]              # Default given Shock=T, Shock=F
    ])

    # Bank_B depends on MarketShock and Bank_A (borrower)
    # Parent configs: (T, H), (T, D), (F, H), (F, D)
    # Row 0: Healthy
    # Row 1: Default
    builder.set_cpd("Bank_B", [
        [0.2, 0.05, 0.9, 0.4], # Healthy
        [0.8, 0.95, 0.1, 0.6]  # Default
    ])

    # For brevity, we'll use similar logic for C and D
    builder.set_cpd("Bank_C", [
        [0.2, 0.05, 0.9, 0.4],
        [0.8, 0.95, 0.1, 0.6]
    ])
    builder.set_cpd("Bank_D", [
        [0.2, 0.05, 0.9, 0.4],
        [0.8, 0.95, 0.1, 0.6]
    ])
    
    # Bank_E only depends on MarketShock
    builder.set_cpd("Bank_E", [
        [1.0 - p_shock, 1.0 - p_healthy],
        [p_shock, p_healthy]
    ])

    # SystemicCrisis depends on D and E
    # Parent configs: (D=H, E=H), (D=H, E=D), (D=D, E=H), (D=D, E=D)
    # Row 0: True, Row 1: False
    builder.set_cpd("SystemicCrisis", [
        [0.01, 0.5, 0.6, 0.95], # True
        [0.99, 0.5, 0.4, 0.05]  # False
    ])

    network = builder.build()
    engine = InferenceEngine(network)

    # 3. Run Queries
    print("Scenario 1: Baseline probability of Systemic Crisis")
    res1 = engine.query(["SystemicCrisis"])
    print(res1.to_dict()["distributions"]["SystemicCrisis"])

    print("\nScenario 2: Prob of Crisis given Bank_A defaults (contagion check)")
    res2 = engine.query(["SystemicCrisis"], evidence={"Bank_A": "Default"})
    print(res2.to_dict()["distributions"]["SystemicCrisis"])

    print("\nScenario 3: Prob of MarketShock given SystemicCrisis=True (diagnostic)")
    res3 = engine.query(["MarketShock"], evidence={"SystemicCrisis": "True"})
    print(res3.to_dict()["distributions"]["MarketShock"])

if __name__ == "__main__":
    run_finance_example()
