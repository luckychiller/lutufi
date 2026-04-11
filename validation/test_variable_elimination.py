import pytest
import numpy as np
from lutufi import BayesianNetwork, InferenceEngine
from validation.test_asia_construction import build_asia_network

def test_ve_asia_no_evidence():
    model = build_asia_network()
    engine = InferenceEngine(model, algorithm="variable_elimination")
    
    # P(Asia)
    res = engine.query(variables=["Asia"])
    # Asia domain is ["F", "T"], so index 1 is T
    # P(Asia=T) = 0.01
    np.testing.assert_allclose(res["Asia"], [0.99, 0.01], atol=1e-10)
    
    # P(Tuberculosis)
    # P(T=T) = 0.0104
    res = engine.query(variables=["Tuberculosis"])
    np.testing.assert_allclose(res["Tuberculosis"], [1 - 0.0104, 0.0104], atol=1e-10)
    
    # P(Smoking)
    res = engine.query(variables=["Smoking"])
    np.testing.assert_allclose(res["Smoking"], [0.5, 0.5], atol=1e-10)
    
    # P(LungCancer)
    # P(L=T) = 0.055
    res = engine.query(variables=["LungCancer"])
    np.testing.assert_allclose(res["LungCancer"], [1 - 0.055, 0.055], atol=1e-10)
    
    # P(Bronchitis)
    # P(B=T) = 0.45
    res = engine.query(variables=["Bronchitis"])
    np.testing.assert_allclose(res["Bronchitis"], [1 - 0.45, 0.45], atol=1e-10)

def test_ve_asia_with_evidence():
    model = build_asia_network()
    engine = InferenceEngine(model, algorithm="variable_elimination")
    
    # P(Tuberculosis | Asia=T)
    # Should be [0.95, 0.05]
    res = engine.query(variables=["Tuberculosis"], evidence={"Asia": "1"})
    np.testing.assert_allclose(res["Tuberculosis"], [0.95, 0.05], atol=1e-10)
    
    # P(TbOrCa | Tuberculosis=T)
    # Since it's an OR, if Tb=T, TbOrCa must be T
    res = engine.query(variables=["TbOrCa"], evidence={"Tuberculosis": "1"})
    np.testing.assert_allclose(res["TbOrCa"], [0.0, 1.0], atol=1e-10)

def test_ve_log_space_underflow():
    """
    Test that log-space arithmetic prevents underflow.
    We'll create a long chain of variables with small probabilities.
    """
    builder = BayesianNetwork.builder()
    n = 100
    builder.add_variable("V0", domain=["0", "1"])
    builder.set_cpd("V0", [0.9, 0.1])
    
    for i in range(1, n):
        prev = f"V{i-1}"
        curr = f"V{i}"
        builder.add_variable(curr, domain=["0", "1"])
        builder.add_edge(prev, curr)
        # P(Vi=1 | Vi-1=1) = 0.1, P(Vi=1 | Vi-1=0) = 0.1
        builder.set_cpd(curr, [[0.9, 0.9], [0.1, 0.1]])
        
    model = builder.build()
    engine = InferenceEngine(model, algorithm="variable_elimination")
    
    # P(V99=1) should be 0.1
    # In naive arithmetic, 0.1^100 would underflow to 0.0
    res = engine.query(variables=[f"V{n-1}"])
    np.testing.assert_allclose(res[f"V{n-1}"], [0.9, 0.1], atol=1e-10)
